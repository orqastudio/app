import { describe, it, expect, beforeEach } from "vitest";
import {
	startGate,
	submitVerdict,
	getOpenGates,
	getGateSession,
	clearGateSessions,
	setAiRecommendation,
	type GateSession,
	type GateVerdictInput,
	type GateResult,
} from "../src/lib/gate-engine.js";
import { parseWorkflowYaml, clearWorkflowCache } from "../src/lib/workflow-engine.js";
import type { ArtifactContext, ActorContext } from "../src/lib/workflow-engine.js";
import type { WorkflowDefinition } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

const SIMPLE_APPROVAL_WORKFLOW_YAML = `
name: simple-gate-test
version: "1.0.0"
artifact_type: task
plugin: test-plugin
initial_state: active

states:
  active:
    category: active
  review:
    category: review
  completed:
    category: completed
  rework:
    category: active

transitions:
  - from: active
    to: review
    event: submit
    gate: task-review
  - from: review
    to: completed
    event: approve
  - from: review
    to: rework
    event: reject

gates:
  task-review:
    pattern: simple_approval
    description: "Standard task review."
    phases:
      gather:
        fields:
          - description
          - priority
          - relationships
        pre_checks:
          - type: field_check
            description: "Must have a description"
            params:
              field: description
              operator: not_empty
          - type: relationship_check
            description: "Must deliver to an epic"
            params:
              relationship_type: delivers
              condition: exists
        summary_template: "Task: \${id}, Priority: \${priority}"
      present:
        sections:
          - title: "Task Summary"
            content_field: description
          - title: "Deliverables"
            content_template: "Related epics: \${relationships:delivers}"
      collect:
        verdicts:
          - key: approve
            label: "Approve"
            transitions_to: completed
          - key: reject
            label: "Reject"
            transitions_to: rework
          - key: request_changes
            label: "Request Changes"
            transitions_to: active
        require_rationale: false
      execute:
        actions:
          - type: append_log
            params:
              message: "Gate verdict: \${verdict}"
      learn:
        on_fail:
          create_lesson: true
          track_recurrence: true
        on_pass:
          track_cycle_time: true
`;

const STRUCTURED_REVIEW_WORKFLOW_YAML = `
name: structured-gate-test
version: "1.0.0"
artifact_type: epic
plugin: test-plugin
initial_state: active

states:
  active:
    category: active
  review:
    category: review
  completed:
    category: completed

transitions:
  - from: active
    to: review
    event: submit
    gate: epic-review
  - from: review
    to: completed
    event: approve

gates:
  epic-review:
    pattern: structured_review
    description: "Epic review — AI then human."
    phases:
      gather:
        fields:
          - description
        pre_checks:
          - type: field_check
            description: "Must have description"
            params:
              field: description
              operator: not_empty
      collect:
        verdicts:
          - key: approve
            label: "Approve"
            transitions_to: completed
          - key: request_changes
            label: "Request Changes"
            transitions_to: active
        require_rationale: true
      learn:
        on_fail:
          create_lesson: true
          track_recurrence: true
`;

const MULTI_REVIEWER_WORKFLOW_YAML = `
name: multi-gate-test
version: "1.0.0"
artifact_type: task
plugin: test-plugin
initial_state: active

states:
  active:
    category: active
  review:
    category: review
  completed:
    category: completed

transitions:
  - from: active
    to: review
    event: submit
    gate: four-eyes
  - from: review
    to: completed
    event: approve

gates:
  four-eyes:
    pattern: multi_reviewer
    description: "Four-eyes review — two reviewers required."
    phases:
      gather:
        fields:
          - description
      collect:
        verdicts:
          - key: approve
            label: "Approve"
            transitions_to: completed
          - key: reject
            label: "Reject"
            transitions_to: active
        min_reviewers: 2
      learn:
        on_fail:
          create_lesson: true
          track_recurrence: true
`;

const ESCALATION_WORKFLOW_YAML = `
name: escalation-gate-test
version: "1.0.0"
artifact_type: task
plugin: test-plugin
initial_state: active

states:
  active:
    category: active
  review:
    category: review
  completed:
    category: completed

transitions:
  - from: active
    to: review
    event: submit
    gate: escalation-gate
  - from: review
    to: completed
    event: approve

gates:
  escalation-gate:
    pattern: escalation
    description: "Escalation gate with timeout."
    timeout:
      duration: "24h"
      action: escalate
    phases:
      collect:
        verdicts:
          - key: approve
            label: "Approve"
            transitions_to: completed
          - key: reject
            label: "Reject"
            transitions_to: active
`;

const SCOPE_DECISION_WORKFLOW_YAML = `
name: scope-gate-test
version: "1.0.0"
artifact_type: epic
plugin: test-plugin
initial_state: active

states:
  active:
    category: active
  review:
    category: review
  completed:
    category: completed
  archived:
    category: terminal

transitions:
  - from: active
    to: review
    event: submit
    gate: scope-gate
  - from: review
    to: completed
    event: approve

gates:
  scope-gate:
    pattern: scope_decision
    description: "Scope decision — multiple outcome paths."
    phases:
      gather:
        fields:
          - description
      collect:
        verdicts:
          - key: proceed
            label: "Proceed as planned"
            transitions_to: completed
          - key: descope
            label: "Reduce scope"
            transitions_to: completed
          - key: expand
            label: "Expand scope"
            transitions_to: active
          - key: cancel
            label: "Cancel"
            transitions_to: archived
`;

function makeArtifact(
	overrides: Partial<ArtifactContext> = {},
): ArtifactContext {
	return {
		id: "TASK-001",
		state: "active",
		artifact_type: "task",
		fields: {
			description: "Implement the feature",
			priority: "high",
		},
		relationships: {
			delivers: [{ target_id: "EPIC-001" }],
		},
		...overrides,
	};
}

function makeActor(
	overrides: Partial<ActorContext> = {},
): ActorContext {
	return {
		id: "human:alice",
		roles: ["developer"],
		...overrides,
	};
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("Gate Engine — startGate", () => {
	beforeEach(() => {
		clearWorkflowCache();
		clearGateSessions();
	});

	it("creates a gate session with GATHER and PRESENT phases completed", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const artifact = makeArtifact();
		const actor = makeActor();

		const session = startGate(artifact, "task-review", workflow, actor);

		expect(session.gateName).toBe("task-review");
		expect(session.artifactId).toBe("TASK-001");
		expect(session.pattern).toBe("simple_approval");
		expect(session.phase).toBe("collect");
		expect(session.verdictOptions).toEqual([
			"approve",
			"reject",
			"request_changes",
		]);
	});

	it("runs pre-checks during GATHER phase", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const artifact = makeArtifact();
		const actor = makeActor();

		const session = startGate(artifact, "task-review", workflow, actor);

		expect(session.preCheckResults).toHaveLength(2);
		expect(session.preCheckResults[0]!.description).toBe(
			"Must have a description",
		);
		expect(session.preCheckResults[0]!.passed).toBe(true);
		expect(session.preCheckResults[1]!.description).toBe(
			"Must deliver to an epic",
		);
		expect(session.preCheckResults[1]!.passed).toBe(true);
	});

	it("pre-checks fail when artifact data is missing", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const artifact = makeArtifact({
			fields: { description: "" },
			relationships: {},
		});
		const actor = makeActor();

		const session = startGate(artifact, "task-review", workflow, actor);

		expect(session.preCheckResults[0]!.passed).toBe(false);
		expect(session.preCheckResults[1]!.passed).toBe(false);
	});

	it("generates a summary from the template", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const artifact = makeArtifact();
		const actor = makeActor();

		const session = startGate(artifact, "task-review", workflow, actor);

		expect(session.gatherSummary).toBe("Task: TASK-001, Priority: high");
	});

	it("produces markdown presentation", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const artifact = makeArtifact();
		const actor = makeActor();

		const session = startGate(artifact, "task-review", workflow, actor);

		expect(session.presentation).toContain("# Gate Review");
		expect(session.presentation).toContain("Standard task review");
		expect(session.presentation).toContain("Pre-Check Results");
		expect(session.presentation).toContain("[PASS]");
		expect(session.presentation).toContain("Task Summary");
		expect(session.presentation).toContain("Implement the feature");
	});

	it("gathers specified fields", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const artifact = makeArtifact();
		const actor = makeActor();

		const session = startGate(artifact, "task-review", workflow, actor);

		expect(session.gatheredFields.description).toBe(
			"Implement the feature",
		);
		expect(session.gatheredFields.priority).toBe("high");
		expect(session.gatheredFields.relationships).toEqual({
			delivers: [{ target_id: "EPIC-001" }],
		});
	});

	it("throws when gate name does not exist", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const artifact = makeArtifact();
		const actor = makeActor();

		expect(() =>
			startGate(artifact, "nonexistent", workflow, actor),
		).toThrow("Gate 'nonexistent' not found");
	});

	it("registers session in open gates", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const artifact = makeArtifact();
		const actor = makeActor();

		const session = startGate(artifact, "task-review", workflow, actor);
		const open = getOpenGates();

		expect(open).toHaveLength(1);
		expect(open[0]!.id).toBe(session.id);
	});

	it("can retrieve session by ID", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const artifact = makeArtifact();
		const actor = makeActor();

		const session = startGate(artifact, "task-review", workflow, actor);
		const retrieved = getGateSession(session.id);

		expect(retrieved).toBeDefined();
		expect(retrieved!.gateName).toBe("task-review");
	});
});

describe("Gate Engine — Simple Approval", () => {
	let workflow: WorkflowDefinition;
	let session: GateSession;
	const artifact = makeArtifact();
	const actor = makeActor();

	beforeEach(() => {
		clearWorkflowCache();
		clearGateSessions();
		workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		session = startGate(artifact, "task-review", workflow, actor);
	});

	it("approves and returns transition target", async () => {
		const result = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "Looks good",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.resolved).toBe(true);
		expect(result.finalVerdict).toBe("approve");
		expect(result.transitionsTo).toBe("completed");
		expect(result.errors).toHaveLength(0);
		expect(result.lessonAction).toBeNull(); // no lesson on approval
	});

	it("rejects and creates lesson action", async () => {
		const result = await submitVerdict(
			session,
			{
				verdict: "reject",
				rationale: "Tests are missing",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.resolved).toBe(true);
		expect(result.finalVerdict).toBe("reject");
		expect(result.transitionsTo).toBe("rework");
		expect(result.lessonAction).not.toBeNull();
		expect(result.lessonAction!.type).toBe("create");
		expect(result.lessonAction!.rationale).toBe("Tests are missing");
		expect(result.lessonAction!.trackRecurrence).toBe(true);
	});

	it("request_changes creates lesson action", async () => {
		const result = await submitVerdict(
			session,
			{
				verdict: "request_changes",
				rationale: "Need more docs",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.resolved).toBe(true);
		expect(result.finalVerdict).toBe("request_changes");
		expect(result.transitionsTo).toBe("active");
		expect(result.lessonAction).not.toBeNull();
	});

	it("rejects invalid verdict key", async () => {
		const result = await submitVerdict(
			session,
			{
				verdict: "invalid_verdict",
				rationale: "",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.resolved).toBe(false);
		expect(result.errors.length).toBeGreaterThan(0);
		expect(result.errors[0]).toContain("Invalid verdict");
	});

	it("runs EXECUTE phase actions", async () => {
		const result = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "Fine",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		// Should have append_log from execute phase
		const logs = result.effects.filter((e) => e.type === "append_log");
		expect(logs.length).toBe(1);
		expect(logs[0]!.entry).toContain("Gate verdict: approve");
	});

	it("removes session from open gates after completion", async () => {
		await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(getOpenGates()).toHaveLength(0);
	});

	it("records reviewer verdicts", async () => {
		const result = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "All good",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.verdicts).toHaveLength(1);
		expect(result.verdicts[0]!.reviewerId).toBe("human:bob");
		expect(result.verdicts[0]!.verdict).toBe("approve");
		expect(result.verdicts[0]!.rationale).toBe("All good");
	});
});

describe("Gate Engine — Structured Review (Maker-Checker)", () => {
	let workflow: WorkflowDefinition;
	const artifact = makeArtifact({
		artifact_type: "epic",
		fields: { description: "Epic description" },
	});
	const actor = makeActor();

	beforeEach(() => {
		clearWorkflowCache();
		clearGateSessions();
		workflow = parseWorkflowYaml(STRUCTURED_REVIEW_WORKFLOW_YAML);
	});

	it("requires two reviews — AI then human", async () => {
		const session = startGate(artifact, "epic-review", workflow, actor);
		expect(session.pattern).toBe("structured_review");

		// AI review (first verdict — not resolved)
		const aiResult = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "AI: all checks pass",
				reviewerId: "agent:reviewer",
			},
			workflow,
			artifact,
			actor,
		);

		expect(aiResult.resolved).toBe(false);
		expect(session.aiRecommendation).toBe("approve");

		// Human review (second verdict — resolved)
		const humanResult = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "Confirmed by human",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);

		expect(humanResult.resolved).toBe(true);
		expect(humanResult.finalVerdict).toBe("approve");
		expect(humanResult.transitionsTo).toBe("completed");
	});

	it("human can override AI recommendation", async () => {
		const session = startGate(artifact, "epic-review", workflow, actor);

		// AI approves
		await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "AI: looks fine",
				reviewerId: "agent:reviewer",
			},
			workflow,
			artifact,
			actor,
		);

		// Human rejects
		const result = await submitVerdict(
			session,
			{
				verdict: "request_changes",
				rationale: "Missing acceptance criteria",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.resolved).toBe(true);
		expect(result.finalVerdict).toBe("request_changes");
		expect(result.transitionsTo).toBe("active");
	});

	it("requires rationale when configured", async () => {
		const session = startGate(artifact, "epic-review", workflow, actor);

		const result = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "",
				reviewerId: "agent:reviewer",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.resolved).toBe(false);
		expect(result.errors).toContain(
			"Rationale is required for this gate",
		);
	});
});

describe("Gate Engine — Multi-Reviewer (Four-Eyes)", () => {
	let workflow: WorkflowDefinition;
	const artifact = makeArtifact();
	const actor = makeActor();

	beforeEach(() => {
		clearWorkflowCache();
		clearGateSessions();
		workflow = parseWorkflowYaml(MULTI_REVIEWER_WORKFLOW_YAML);
	});

	it("requires minimum number of reviewers", async () => {
		const session = startGate(artifact, "four-eyes", workflow, actor);
		expect(session.pattern).toBe("multi_reviewer");
		expect(session.minReviewers).toBe(2);

		// First reviewer
		const r1 = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "OK from reviewer 1",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);

		expect(r1.resolved).toBe(false);

		// Second reviewer
		const r2 = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "OK from reviewer 2",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(r2.resolved).toBe(true);
		expect(r2.finalVerdict).toBe("approve");
		expect(r2.transitionsTo).toBe("completed");
	});

	it("rejects if any reviewer rejects", async () => {
		const session = startGate(artifact, "four-eyes", workflow, actor);

		await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "Approved",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);

		const r2 = await submitVerdict(
			session,
			{
				verdict: "reject",
				rationale: "Issues found",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(r2.resolved).toBe(true);
		expect(r2.finalVerdict).toBe("reject");
		expect(r2.transitionsTo).toBe("active");
	});

	it("tracks all reviewer verdicts", async () => {
		const session = startGate(artifact, "four-eyes", workflow, actor);

		await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "R1",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);

		const result = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "R2",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.verdicts).toHaveLength(2);
		expect(result.verdicts[0]!.reviewerId).toBe("human:alice");
		expect(result.verdicts[1]!.reviewerId).toBe("human:bob");
	});
});

describe("Gate Engine — Escalation", () => {
	let workflow: WorkflowDefinition;
	const artifact = makeArtifact();
	const actor = makeActor();

	beforeEach(() => {
		clearWorkflowCache();
		clearGateSessions();
		workflow = parseWorkflowYaml(ESCALATION_WORKFLOW_YAML);
	});

	it("creates session with timeout config", () => {
		const session = startGate(
			artifact,
			"escalation-gate",
			workflow,
			actor,
		);

		expect(session.pattern).toBe("escalation");
		expect(session.timeout).toEqual({
			duration: "24h",
			action: "escalate",
		});
	});

	it("resolves immediately on single verdict (like simple_approval)", async () => {
		const session = startGate(
			artifact,
			"escalation-gate",
			workflow,
			actor,
		);

		const result = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "Approved before timeout",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.resolved).toBe(true);
		expect(result.finalVerdict).toBe("approve");
	});
});

describe("Gate Engine — Scope Decision", () => {
	let workflow: WorkflowDefinition;
	const artifact = makeArtifact({
		artifact_type: "epic",
		fields: { description: "Epic scope decision" },
	});
	const actor = makeActor();

	beforeEach(() => {
		clearWorkflowCache();
		clearGateSessions();
		workflow = parseWorkflowYaml(SCOPE_DECISION_WORKFLOW_YAML);
	});

	it("offers multiple outcome paths", () => {
		const session = startGate(artifact, "scope-gate", workflow, actor);
		expect(session.pattern).toBe("scope_decision");
		expect(session.verdictOptions).toEqual([
			"proceed",
			"descope",
			"expand",
			"cancel",
		]);
	});

	it("transitions to correct state for each verdict", async () => {
		// Test proceed
		let session = startGate(artifact, "scope-gate", workflow, actor);
		let result = await submitVerdict(
			session,
			{
				verdict: "proceed",
				rationale: "On track",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);
		expect(result.transitionsTo).toBe("completed");

		// Test descope
		clearGateSessions();
		session = startGate(artifact, "scope-gate", workflow, actor);
		result = await submitVerdict(
			session,
			{
				verdict: "descope",
				rationale: "Too much scope",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);
		expect(result.transitionsTo).toBe("completed");

		// Test expand
		clearGateSessions();
		session = startGate(artifact, "scope-gate", workflow, actor);
		result = await submitVerdict(
			session,
			{
				verdict: "expand",
				rationale: "Need more features",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);
		expect(result.transitionsTo).toBe("active");

		// Test cancel
		clearGateSessions();
		session = startGate(artifact, "scope-gate", workflow, actor);
		result = await submitVerdict(
			session,
			{
				verdict: "cancel",
				rationale: "No longer needed",
				reviewerId: "human:alice",
			},
			workflow,
			artifact,
			actor,
		);
		expect(result.transitionsTo).toBe("archived");
	});
});

describe("Gate Engine — Learning Integration", () => {
	let workflow: WorkflowDefinition;
	const artifact = makeArtifact();
	const actor = makeActor();

	beforeEach(() => {
		clearWorkflowCache();
		clearGateSessions();
		workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
	});

	it("creates lesson on rejection", async () => {
		const session = startGate(artifact, "task-review", workflow, actor);

		const result = await submitVerdict(
			session,
			{
				verdict: "reject",
				rationale: "No tests",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.lessonAction).not.toBeNull();
		expect(result.lessonAction!.type).toBe("create");
		expect(result.lessonAction!.artifactId).toBe("TASK-001");
		expect(result.lessonAction!.gateName).toBe("task-review");
		expect(result.lessonAction!.rationale).toBe("No tests");
		expect(result.lessonAction!.trackRecurrence).toBe(true);
		expect(result.lessonAction!.promoteToRule).toBe(false);
	});

	it("does not create lesson on approval", async () => {
		const session = startGate(artifact, "task-review", workflow, actor);

		const result = await submitVerdict(
			session,
			{
				verdict: "approve",
				rationale: "Good",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
		);

		expect(result.lessonAction).toBeNull();
	});

	it("promotes to rule when recurrence threshold reached", async () => {
		const session = startGate(artifact, "task-review", workflow, actor);

		const result = await submitVerdict(
			session,
			{
				verdict: "reject",
				rationale: "Same issue again",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
			{
				lessonRecurrenceCounts: { "TASK-001": 2 },
				promotionThreshold: 3,
			},
		);

		expect(result.lessonAction).not.toBeNull();
		expect(result.lessonAction!.promoteToRule).toBe(true);
	});

	it("does not promote when below threshold", async () => {
		const session = startGate(artifact, "task-review", workflow, actor);

		const result = await submitVerdict(
			session,
			{
				verdict: "reject",
				rationale: "Issue",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
			{
				lessonRecurrenceCounts: { "TASK-001": 1 },
				promotionThreshold: 3,
			},
		);

		expect(result.lessonAction).not.toBeNull();
		expect(result.lessonAction!.promoteToRule).toBe(false);
	});

	it("uses default threshold of 3 when not specified", async () => {
		const session = startGate(artifact, "task-review", workflow, actor);

		const result = await submitVerdict(
			session,
			{
				verdict: "reject",
				rationale: "Issue",
				reviewerId: "human:bob",
			},
			workflow,
			artifact,
			actor,
			{
				lessonRecurrenceCounts: { "TASK-001": 2 },
				// No promotionThreshold — should default to 3
			},
		);

		expect(result.lessonAction!.promoteToRule).toBe(true);
	});
});

describe("Gate Engine — Session Management", () => {
	beforeEach(() => {
		clearWorkflowCache();
		clearGateSessions();
	});

	it("tracks multiple open gates", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const actor = makeActor();

		const a1 = makeArtifact({ id: "TASK-001" });
		const a2 = makeArtifact({ id: "TASK-002" });

		startGate(a1, "task-review", workflow, actor);
		startGate(a2, "task-review", workflow, actor);

		expect(getOpenGates()).toHaveLength(2);
	});

	it("clearGateSessions removes all sessions", () => {
		const workflow = parseWorkflowYaml(SIMPLE_APPROVAL_WORKFLOW_YAML);
		const actor = makeActor();
		const artifact = makeArtifact();

		startGate(artifact, "task-review", workflow, actor);
		expect(getOpenGates()).toHaveLength(1);

		clearGateSessions();
		expect(getOpenGates()).toHaveLength(0);
	});

	it("returns undefined for nonexistent session ID", () => {
		expect(getGateSession("nonexistent")).toBeUndefined();
	});
});

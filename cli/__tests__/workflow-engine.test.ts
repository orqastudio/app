import { describe, it, expect, beforeEach } from "vitest";
import {
	parseWorkflowYaml,
	evaluateFieldCheck,
	evaluateRelationshipCheck,
	evaluateRoleCheck,
	evaluateGuardsSync,
	findMatchingTransitions,
	resolveTransition,
	executeTransition,
	selectVariant,
	applyVariant,
	getAvailableEvents,
	executeSetField,
	executeAppendLog,
	executeCreateArtifact,
	executeNotify,
	clearWorkflowCache,
	WorkflowError,
	type ArtifactContext,
	type ActorContext,
} from "../src/lib/workflow-engine.js";
import type { WorkflowDefinition, Guard } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

const MINIMAL_WORKFLOW_YAML = `
name: test-workflow
version: "1.0.0"
artifact_type: task
plugin: test-plugin
initial_state: draft

states:
  draft:
    category: planning
  active:
    category: active
    on_enter:
      - type: set_field
        params:
          field: started_at
          value: "\${now}"
      - type: append_log
        params:
          message: "Activated by \${actor}"
  completed:
    category: completed
    on_enter:
      - type: set_field
        params:
          field: completed_at
          value: "\${now}"

transitions:
  - from: draft
    to: active
    event: start
    guards:
      - type: field_check
        description: "Must have a title"
        params:
          field: title
          operator: not_empty

  - from: draft
    to: active
    event: force_start
    description: "Start without guards"

  - from: active
    to: completed
    event: complete
    actions:
      - type: notify
        params:
          channel: ui
          message: "Task \${id} completed"

  - from: [draft, active]
    to: completed
    event: close
    description: "Close from any non-terminal state"
`;

const GUARDED_WORKFLOW_YAML = `
name: guarded-workflow
version: "1.0.0"
artifact_type: task
plugin: test-plugin
initial_state: draft

states:
  draft:
    category: planning
  review:
    category: review
  completed:
    category: completed

transitions:
  - from: draft
    to: review
    event: submit
    guards:
      - type: field_check
        description: "Must have description"
        params:
          field: description
          operator: not_empty
      - type: relationship_check
        description: "Must deliver to an epic"
        params:
          relationship_type: delivers
          condition: exists

  - from: draft
    to: review
    event: submit_with_role
    guards:
      - type: role_check
        description: "Must be a developer"
        params:
          roles: ["developer", "lead"]

  - from: review
    to: completed
    event: approve
    gate: review-gate

gates:
  review-gate:
    pattern: simple_approval
    phases:
      collect:
        verdicts:
          - key: approve
            label: Approve
          - key: reject
            label: Reject
`;

const VARIANT_WORKFLOW_YAML = `
name: variant-workflow
version: "1.0.0"
artifact_type: task
plugin: test-plugin
initial_state: captured

states:
  captured:
    category: planning
  ready:
    category: planning
  active:
    category: active
  review:
    category: review
  completed:
    category: completed

transitions:
  - from: captured
    to: ready
    event: triage
  - from: ready
    to: active
    event: start
  - from: active
    to: review
    event: submit
    gate: task-review
  - from: review
    to: completed
    event: approve

gates:
  task-review:
    pattern: simple_approval
    phases:
      collect:
        verdicts:
          - key: approve
            label: Approve
          - key: reject
            label: Reject

variants:
  quickfix:
    description: "Skip planning, no review gate"
    skip_states:
      - ready
    skip_gates:
      - task-review
    override_transitions:
      - from: captured
        to: active
        event: start

selection_rules:
  - variant: quickfix
    priority: 10
    conditions:
      - type: field_check
        params:
          field: labels
          operator: in
          value: ["quickfix", "bugfix"]
`;

function makeArtifact(overrides: Partial<ArtifactContext> = {}): ArtifactContext {
	return {
		id: "TASK-001",
		state: "draft",
		artifact_type: "task",
		fields: { title: "Test task", description: "A test" },
		relationships: {},
		...overrides,
	};
}

function makeActor(overrides: Partial<ActorContext> = {}): ActorContext {
	return {
		id: "human:alice",
		roles: ["developer"],
		...overrides,
	};
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("Workflow Loader", () => {
	beforeEach(() => {
		clearWorkflowCache();
	});

	it("parses a valid workflow YAML", () => {
		const wf = parseWorkflowYaml(MINIMAL_WORKFLOW_YAML);
		expect(wf.name).toBe("test-workflow");
		expect(wf.version).toBe("1.0.0");
		expect(wf.artifact_type).toBe("task");
		expect(wf.initial_state).toBe("draft");
		expect(Object.keys(wf.states)).toHaveLength(3);
		expect(wf.transitions).toHaveLength(4);
	});

	it("rejects YAML with missing required fields", () => {
		const bad = `
name: bad
version: "1.0.0"
states:
  a:
    category: planning
`;
		expect(() => parseWorkflowYaml(bad)).toThrow(WorkflowError);
	});

	it("rejects workflow with fewer than 2 states", () => {
		const bad = `
name: bad
version: "1.0.0"
artifact_type: task
plugin: test
initial_state: only
states:
  only:
    category: planning
transitions:
  - from: only
    to: only
    event: loop
`;
		expect(() => parseWorkflowYaml(bad)).toThrow("at least 2 states");
	});

	it("rejects workflow with unknown initial_state", () => {
		const bad = `
name: bad
version: "1.0.0"
artifact_type: task
plugin: test
initial_state: nonexistent
states:
  a:
    category: planning
  b:
    category: active
transitions:
  - from: a
    to: b
    event: go
`;
		expect(() => parseWorkflowYaml(bad)).toThrow("not found in states");
	});

	it("rejects transition referencing unknown state", () => {
		const bad = `
name: bad
version: "1.0.0"
artifact_type: task
plugin: test
initial_state: a
states:
  a:
    category: planning
  b:
    category: active
transitions:
  - from: a
    to: unknown
    event: go
`;
		expect(() => parseWorkflowYaml(bad)).toThrow("unknown target state");
	});

	it("rejects non-YAML content", () => {
		expect(() => parseWorkflowYaml("not { yaml: at all")).toThrow(WorkflowError);
	});
});

describe("Guard: field_check", () => {
	it("exists — passes when field is present", () => {
		const artifact = makeArtifact({ fields: { title: "hello" } });
		expect(
			evaluateFieldCheck(artifact, { field: "title", operator: "exists" }),
		).toBe(true);
	});

	it("exists — fails when field is missing", () => {
		const artifact = makeArtifact({ fields: {} });
		expect(
			evaluateFieldCheck(artifact, { field: "title", operator: "exists" }),
		).toBe(false);
	});

	it("not_empty — passes for non-empty string", () => {
		const artifact = makeArtifact({ fields: { title: "hello" } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "title",
				operator: "not_empty",
			}),
		).toBe(true);
	});

	it("not_empty — fails for empty string", () => {
		const artifact = makeArtifact({ fields: { title: "" } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "title",
				operator: "not_empty",
			}),
		).toBe(false);
	});

	it("not_empty — fails for empty array", () => {
		const artifact = makeArtifact({ fields: { labels: [] } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "labels",
				operator: "not_empty",
			}),
		).toBe(false);
	});

	it("equals — passes on match", () => {
		const artifact = makeArtifact({ fields: { status: "draft" } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "status",
				operator: "equals",
				value: "draft",
			}),
		).toBe(true);
	});

	it("equals — fails on mismatch", () => {
		const artifact = makeArtifact({ fields: { status: "active" } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "status",
				operator: "equals",
				value: "draft",
			}),
		).toBe(false);
	});

	it("not_equals — passes on mismatch", () => {
		const artifact = makeArtifact({ fields: { status: "active" } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "status",
				operator: "not_equals",
				value: "draft",
			}),
		).toBe(true);
	});

	it("in — passes when scalar value is in array", () => {
		const artifact = makeArtifact({ fields: { priority: "high" } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "priority",
				operator: "in",
				value: ["high", "critical"],
			}),
		).toBe(true);
	});

	it("in — passes when array field overlaps with value array", () => {
		const artifact = makeArtifact({
			fields: { labels: ["bugfix", "docs"] },
		});
		expect(
			evaluateFieldCheck(artifact, {
				field: "labels",
				operator: "in",
				value: ["bugfix", "quickfix"],
			}),
		).toBe(true);
	});

	it("in — fails when no overlap", () => {
		const artifact = makeArtifact({
			fields: { labels: ["feature"] },
		});
		expect(
			evaluateFieldCheck(artifact, {
				field: "labels",
				operator: "in",
				value: ["bugfix", "quickfix"],
			}),
		).toBe(false);
	});

	it("not_in — passes when value not in array", () => {
		const artifact = makeArtifact({ fields: { priority: "low" } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "priority",
				operator: "not_in",
				value: ["high", "critical"],
			}),
		).toBe(true);
	});

	it("matches — passes on regex match", () => {
		const artifact = makeArtifact({ fields: { name: "TASK-123" } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "name",
				operator: "matches",
				value: "^TASK-\\d+$",
			}),
		).toBe(true);
	});

	it("matches — fails on non-match", () => {
		const artifact = makeArtifact({ fields: { name: "EPIC-123" } });
		expect(
			evaluateFieldCheck(artifact, {
				field: "name",
				operator: "matches",
				value: "^TASK-\\d+$",
			}),
		).toBe(false);
	});

	it("supports nested field access", () => {
		const artifact = makeArtifact({
			fields: { meta: { priority: "high" } },
		});
		expect(
			evaluateFieldCheck(artifact, {
				field: "meta.priority",
				operator: "equals",
				value: "high",
			}),
		).toBe(true);
	});
});

describe("Guard: relationship_check", () => {
	it("exists — passes when relationships exist", () => {
		const artifact = makeArtifact({
			relationships: {
				delivers: [{ target_id: "EPIC-001" }],
			},
		});
		expect(
			evaluateRelationshipCheck(artifact, {
				relationship_type: "delivers",
				condition: "exists",
			}),
		).toBe(true);
	});

	it("exists — fails when no relationships of type", () => {
		const artifact = makeArtifact({ relationships: {} });
		expect(
			evaluateRelationshipCheck(artifact, {
				relationship_type: "delivers",
				condition: "exists",
			}),
		).toBe(false);
	});

	it("min_count — passes when count >= threshold", () => {
		const artifact = makeArtifact({
			relationships: {
				"depends-on": [
					{ target_id: "T-1" },
					{ target_id: "T-2" },
				],
			},
		});
		expect(
			evaluateRelationshipCheck(artifact, {
				relationship_type: "depends-on",
				condition: "min_count",
				min_count: 2,
			}),
		).toBe(true);
	});

	it("min_count — fails when count < threshold", () => {
		const artifact = makeArtifact({
			relationships: {
				"depends-on": [{ target_id: "T-1" }],
			},
		});
		expect(
			evaluateRelationshipCheck(artifact, {
				relationship_type: "depends-on",
				condition: "min_count",
				min_count: 2,
			}),
		).toBe(false);
	});

	it("all_targets_in_status — passes when all targets match", () => {
		const artifact = makeArtifact({
			relationships: {
				"depends-on": [
					{ target_id: "T-1", target_status: "completed" },
					{ target_id: "T-2", target_status: "archived" },
				],
			},
		});
		expect(
			evaluateRelationshipCheck(artifact, {
				relationship_type: "depends-on",
				condition: "all_targets_in_status",
				statuses: ["completed", "archived"],
			}),
		).toBe(true);
	});

	it("all_targets_in_status — fails when one target does not match", () => {
		const artifact = makeArtifact({
			relationships: {
				"depends-on": [
					{ target_id: "T-1", target_status: "completed" },
					{ target_id: "T-2", target_status: "active" },
				],
			},
		});
		expect(
			evaluateRelationshipCheck(artifact, {
				relationship_type: "depends-on",
				condition: "all_targets_in_status",
				statuses: ["completed", "archived"],
			}),
		).toBe(false);
	});

	it("any_target_in_status — passes when at least one matches", () => {
		const artifact = makeArtifact({
			relationships: {
				delivers: [
					{ target_id: "E-1", target_status: "active" },
					{ target_id: "E-2", target_status: "completed" },
				],
			},
		});
		expect(
			evaluateRelationshipCheck(artifact, {
				relationship_type: "delivers",
				condition: "any_target_in_status",
				statuses: ["completed"],
			}),
		).toBe(true);
	});

	it("no_targets_in_status — passes when none match", () => {
		const artifact = makeArtifact({
			relationships: {
				delivers: [
					{ target_id: "E-1", target_status: "active" },
					{ target_id: "E-2", target_status: "active" },
				],
			},
		});
		expect(
			evaluateRelationshipCheck(artifact, {
				relationship_type: "delivers",
				condition: "no_targets_in_status",
				statuses: ["completed"],
			}),
		).toBe(true);
	});
});

describe("Guard: role_check", () => {
	it("passes when actor has a matching role", () => {
		const actor = makeActor({ roles: ["developer", "reviewer"] });
		expect(evaluateRoleCheck(actor, { roles: ["reviewer"] })).toBe(true);
	});

	it("fails when actor has no matching role", () => {
		const actor = makeActor({ roles: ["developer"] });
		expect(evaluateRoleCheck(actor, { roles: ["admin", "lead"] })).toBe(
			false,
		);
	});
});

describe("Transition Resolver", () => {
	let workflow: WorkflowDefinition;

	beforeEach(() => {
		clearWorkflowCache();
		workflow = parseWorkflowYaml(MINIMAL_WORKFLOW_YAML);
	});

	it("finds matching transitions for state + event", () => {
		const matches = findMatchingTransitions(workflow, "draft", "start");
		expect(matches).toHaveLength(1);
		expect(matches[0]!.to).toBe("active");
	});

	it("finds transitions with array from", () => {
		const matches = findMatchingTransitions(workflow, "draft", "close");
		expect(matches).toHaveLength(1);
		expect(matches[0]!.to).toBe("completed");
	});

	it("returns empty array for unmatched event", () => {
		const matches = findMatchingTransitions(
			workflow,
			"draft",
			"nonexistent",
		);
		expect(matches).toHaveLength(0);
	});

	it("returns available events from a state", () => {
		const events = getAvailableEvents(workflow, "draft");
		expect(events).toContain("start");
		expect(events).toContain("force_start");
		expect(events).toContain("close");
		expect(events).not.toContain("complete");
	});

	it("resolves first transition where guards pass", async () => {
		const artifact = makeArtifact({
			fields: { title: "Valid" },
		});
		const actor = makeActor();
		const result = await resolveTransition(
			workflow,
			artifact,
			"start",
			actor,
		);
		expect(result).not.toBeNull();
		expect(result!.guards_passed).toBe(true);
		expect(result!.transition.to).toBe("active");
	});

	it("returns failed candidate when guards fail", async () => {
		const artifact = makeArtifact({
			fields: { title: "" }, // empty title fails not_empty
		});
		const actor = makeActor();
		const result = await resolveTransition(
			workflow,
			artifact,
			"start",
			actor,
		);
		expect(result).not.toBeNull();
		expect(result!.guards_passed).toBe(false);
		expect(result!.guard_errors.length).toBeGreaterThan(0);
	});

	it("returns null when no transitions match", async () => {
		const artifact = makeArtifact();
		const actor = makeActor();
		const result = await resolveTransition(
			workflow,
			artifact,
			"nonexistent",
			actor,
		);
		expect(result).toBeNull();
	});

	it("throws on unknown state", async () => {
		const artifact = makeArtifact({ state: "unknown" });
		const actor = makeActor();
		await expect(
			resolveTransition(workflow, artifact, "start", actor),
		).rejects.toThrow("unknown state");
	});
});

describe("Guard: combined evaluation", () => {
	let workflow: WorkflowDefinition;

	beforeEach(() => {
		clearWorkflowCache();
		workflow = parseWorkflowYaml(GUARDED_WORKFLOW_YAML);
	});

	it("passes when all guards pass", async () => {
		const artifact = makeArtifact({
			fields: { description: "A real description" },
			relationships: { delivers: [{ target_id: "EPIC-001" }] },
		});
		const actor = makeActor();
		const result = await resolveTransition(
			workflow,
			artifact,
			"submit",
			actor,
		);
		expect(result!.guards_passed).toBe(true);
	});

	it("fails when field_check fails", async () => {
		const artifact = makeArtifact({
			fields: { description: "" },
			relationships: { delivers: [{ target_id: "EPIC-001" }] },
		});
		const actor = makeActor();
		const result = await resolveTransition(
			workflow,
			artifact,
			"submit",
			actor,
		);
		expect(result!.guards_passed).toBe(false);
	});

	it("fails when relationship_check fails", async () => {
		const artifact = makeArtifact({
			fields: { description: "Has desc" },
			relationships: {},
		});
		const actor = makeActor();
		const result = await resolveTransition(
			workflow,
			artifact,
			"submit",
			actor,
		);
		expect(result!.guards_passed).toBe(false);
	});

	it("role_check passes for matching role", async () => {
		const artifact = makeArtifact();
		const actor = makeActor({ roles: ["developer"] });
		const result = await resolveTransition(
			workflow,
			artifact,
			"submit_with_role",
			actor,
		);
		expect(result!.guards_passed).toBe(true);
	});

	it("role_check fails for non-matching role", async () => {
		const artifact = makeArtifact();
		const actor = makeActor({ roles: ["viewer"] });
		const result = await resolveTransition(
			workflow,
			artifact,
			"submit_with_role",
			actor,
		);
		expect(result!.guards_passed).toBe(false);
	});
});

describe("Action Executors", () => {
	const artifact = makeArtifact();
	const actor = makeActor();

	it("set_field produces a SetFieldEffect", () => {
		const effect = executeSetField(
			{ field: "priority", value: "high" },
			artifact,
			actor,
		);
		expect(effect.type).toBe("set_field");
		expect(effect.field).toBe("priority");
		expect(effect.value).toBe("high");
	});

	it("set_field interpolates template values", () => {
		const effect = executeSetField(
			{ field: "started_at", value: "${now}" },
			artifact,
			actor,
		);
		expect(effect.type).toBe("set_field");
		expect(typeof effect.value).toBe("string");
		// Should be an ISO date string
		expect((effect.value as string).length).toBeGreaterThan(10);
	});

	it("append_log produces an AppendLogEffect with interpolation", () => {
		const effect = executeAppendLog(
			{ message: "Started by ${actor}" },
			artifact,
			actor,
		);
		expect(effect.type).toBe("append_log");
		expect(effect.log_field).toBe("audit_log");
		expect(effect.entry).toBe("Started by human:alice");
	});

	it("append_log uses custom log_field", () => {
		const effect = executeAppendLog(
			{ message: "test", log_field: "changelog" },
			artifact,
			actor,
		);
		expect(effect.log_field).toBe("changelog");
	});

	it("create_artifact produces a CreateArtifactEffect", () => {
		const effect = executeCreateArtifact({
			artifact_type: "lesson",
			template: "review-lesson",
			relationship: "produces",
		});
		expect(effect.type).toBe("create_artifact");
		expect(effect.artifact_type).toBe("lesson");
		expect(effect.template).toBe("review-lesson");
		expect(effect.relationship).toBe("produces");
	});

	it("notify produces a NotifyEffect with interpolation", () => {
		const effect = executeNotify(
			{ channel: "ui", message: "Task ${id} done", severity: "info" },
			artifact,
			actor,
		);
		expect(effect.type).toBe("notify");
		expect(effect.channel).toBe("ui");
		expect(effect.message).toBe("Task TASK-001 done");
		expect(effect.severity).toBe("info");
	});
});

describe("Execute Transition", () => {
	let workflow: WorkflowDefinition;

	beforeEach(() => {
		clearWorkflowCache();
		workflow = parseWorkflowYaml(MINIMAL_WORKFLOW_YAML);
	});

	it("executes a valid transition and returns effects", async () => {
		const artifact = makeArtifact({ fields: { title: "Valid" } });
		const actor = makeActor();
		const result = await executeTransition(
			workflow,
			artifact,
			"start",
			actor,
		);

		expect(result.success).toBe(true);
		expect(result.from_state).toBe("draft");
		expect(result.to_state).toBe("active");
		expect(result.gate_required).toBeNull();

		// Should have: status set_field + on_enter set_field + on_enter append_log
		const setFields = result.effects.filter((e) => e.type === "set_field");
		expect(setFields.length).toBe(2); // status + started_at
		expect(setFields[0]!.field).toBe("status");
		expect(setFields[0]!.value).toBe("active");

		const logs = result.effects.filter((e) => e.type === "append_log");
		expect(logs.length).toBe(1);
		expect(logs[0]!.entry).toContain("Activated by human:alice");
	});

	it("returns failure when guards fail", async () => {
		const artifact = makeArtifact({ fields: { title: "" } });
		const actor = makeActor();
		const result = await executeTransition(
			workflow,
			artifact,
			"start",
			actor,
		);

		expect(result.success).toBe(false);
		expect(result.to_state).toBeNull();
		expect(result.errors.length).toBeGreaterThan(0);
		expect(result.effects).toHaveLength(0);
	});

	it("returns failure when no transition matches", async () => {
		const artifact = makeArtifact();
		const actor = makeActor();
		const result = await executeTransition(
			workflow,
			artifact,
			"nonexistent",
			actor,
		);

		expect(result.success).toBe(false);
		expect(result.errors[0]!.code).toBe("NO_MATCHING_TRANSITION");
	});

	it("returns failure for unknown current state", async () => {
		const artifact = makeArtifact({ state: "phantom" });
		const actor = makeActor();
		const result = await executeTransition(
			workflow,
			artifact,
			"start",
			actor,
		);

		expect(result.success).toBe(false);
		expect(result.errors[0]!.code).toBe("UNKNOWN_STATE");
	});

	it("collects transition actions", async () => {
		const artifact = makeArtifact({
			state: "active",
			fields: { title: "test" },
		});
		const actor = makeActor();
		const result = await executeTransition(
			workflow,
			artifact,
			"complete",
			actor,
		);

		expect(result.success).toBe(true);
		expect(result.to_state).toBe("completed");

		// Should have: status set_field + notify (transition) + on_enter fields
		const notifies = result.effects.filter((e) => e.type === "notify");
		expect(notifies.length).toBe(1);
		expect(notifies[0]!.message).toBe("Task TASK-001 completed");
	});

	it("handles gate-required transitions", async () => {
		const workflow = parseWorkflowYaml(GUARDED_WORKFLOW_YAML);
		const artifact = makeArtifact({
			state: "review",
			fields: {},
		});
		const actor = makeActor();
		const result = await executeTransition(
			workflow,
			artifact,
			"approve",
			actor,
		);

		expect(result.success).toBe(false);
		expect(result.gate_required).toBe("review-gate");
		expect(result.effects).toHaveLength(0);
	});

	it("handles multi-source transitions (array from)", async () => {
		const artifact = makeArtifact({
			state: "active",
			fields: { title: "test" },
		});
		const actor = makeActor();
		const result = await executeTransition(
			workflow,
			artifact,
			"close",
			actor,
		);

		expect(result.success).toBe(true);
		expect(result.to_state).toBe("completed");
	});
});

describe("Variant Selection", () => {
	let workflow: WorkflowDefinition;

	beforeEach(() => {
		clearWorkflowCache();
		workflow = parseWorkflowYaml(VARIANT_WORKFLOW_YAML);
	});

	it("selects variant when conditions match", () => {
		const artifact = makeArtifact({
			fields: { labels: ["bugfix"] },
		});
		const actor = makeActor();
		const result = selectVariant(workflow, artifact, actor);
		expect(result).not.toBeNull();
		expect(result!.name).toBe("quickfix");
	});

	it("returns null when no variant matches", () => {
		const artifact = makeArtifact({
			fields: { labels: ["feature"] },
		});
		const actor = makeActor();
		const result = selectVariant(workflow, artifact, actor);
		expect(result).toBeNull();
	});

	it("applies variant — removes skipped states", () => {
		const variant = workflow.variants!["quickfix"]!;
		const applied = applyVariant(workflow, variant);
		expect(applied.states["ready"]).toBeUndefined();
		expect(applied.states["captured"]).toBeDefined();
		expect(applied.states["active"]).toBeDefined();
	});

	it("applies variant — removes transitions to/from skipped states", () => {
		const variant = workflow.variants!["quickfix"]!;
		const applied = applyVariant(workflow, variant);
		// Original had captured→ready (triage) and ready→active (start)
		// Both should be gone since ready is skipped
		const hasTriageTransition = applied.transitions.some(
			(t) => t.event === "triage",
		);
		expect(hasTriageTransition).toBe(false);
	});

	it("applies variant — adds override transitions", () => {
		const variant = workflow.variants!["quickfix"]!;
		const applied = applyVariant(workflow, variant);
		const startFromCaptured = applied.transitions.find(
			(t) => t.event === "start" && !Array.isArray(t.from) && t.from === "captured",
		);
		expect(startFromCaptured).toBeDefined();
		expect(startFromCaptured!.to).toBe("active");
	});

	it("applies variant — removes skipped gate references", () => {
		const variant = workflow.variants!["quickfix"]!;
		const applied = applyVariant(workflow, variant);
		// The submit transition had gate: task-review, should be removed
		const submitTransition = applied.transitions.find(
			(t) => t.event === "submit",
		);
		expect(submitTransition?.gate).toBeUndefined();
	});
});

describe("Sync guard evaluation", () => {
	it("passes all sync guards", () => {
		const artifact = makeArtifact({
			fields: { title: "Hello" },
			relationships: { delivers: [{ target_id: "EPIC-001" }] },
		});
		const actor = makeActor({ roles: ["developer"] });
		const guards: Guard[] = [
			{
				type: "field_check",
				params: { field: "title", operator: "not_empty" },
			},
			{
				type: "relationship_check",
				params: { relationship_type: "delivers", condition: "exists" },
			},
			{
				type: "role_check",
				params: { roles: ["developer"] },
			},
		];

		const result = evaluateGuardsSync(guards, artifact, actor);
		expect(result.passed).toBe(true);
		expect(result.errors).toHaveLength(0);
	});

	it("collects all guard failure messages", () => {
		const artifact = makeArtifact({
			fields: { title: "" },
			relationships: {},
		});
		const actor = makeActor({ roles: ["viewer"] });
		const guards: Guard[] = [
			{
				type: "field_check",
				description: "Title required",
				params: { field: "title", operator: "not_empty" },
			},
			{
				type: "relationship_check",
				description: "Must deliver to epic",
				params: { relationship_type: "delivers", condition: "exists" },
			},
			{
				type: "role_check",
				description: "Must be developer",
				params: { roles: ["developer"] },
			},
		];

		const result = evaluateGuardsSync(guards, artifact, actor);
		expect(result.passed).toBe(false);
		expect(result.errors).toHaveLength(3);
		expect(result.errors).toContain("Title required");
		expect(result.errors).toContain("Must deliver to epic");
		expect(result.errors).toContain("Must be developer");
	});

	it("skips async guards (query, code_hook) in sync mode", () => {
		const artifact = makeArtifact({ fields: { title: "ok" } });
		const actor = makeActor();
		const guards: Guard[] = [
			{
				type: "field_check",
				params: { field: "title", operator: "not_empty" },
			},
			{
				type: "query",
				params: { query_name: "test-query" },
			},
			{
				type: "code_hook",
				params: { hook: "test-hook" },
			},
		];

		const result = evaluateGuardsSync(guards, artifact, actor);
		// query and code_hook are skipped in sync mode
		expect(result.passed).toBe(true);
	});
});

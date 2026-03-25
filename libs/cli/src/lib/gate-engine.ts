/**
 * Gate engine — five-phase human gate execution pipeline.
 *
 * Gates are structured sub-workflows that intercept transitions requiring
 * human review. They are defined declaratively in workflow YAML files and
 * executed by this engine.
 *
 * Five phases:
 *   1. GATHER  — collect data, run pre-checks, generate summary
 *   2. PRESENT — format gathered data for the reviewer
 *   3. COLLECT — reviewer provides verdict + rationale
 *   4. EXECUTE — apply transition, run post-actions, log audit trail
 *   5. LEARN   — on FAIL: create/update lesson, track recurrence
 *
 * Five gate patterns:
 *   - simple_approval      — single reviewer, approve/reject
 *   - structured_review    — AI review then human review (Maker-Checker)
 *   - multi_reviewer       — multiple independent reviewers (Four-Eyes)
 *   - escalation           — timeout triggers escalation
 *   - scope_decision       — multiple outcome paths
 *
 * The engine never modifies files directly — it returns structured results
 * that the caller applies, consistent with the workflow engine's design.
 */

import type {
	Gate,
	GatePattern,
	WorkflowDefinition,
} from "@orqastudio/types";

import {
	evaluateGuardsSync,
	executeActions,
	type ArtifactContext,
	type ActorContext,
	type ActionEffect,
	type ActionHookHandler,
} from "./workflow-engine.js";

// ---------------------------------------------------------------------------
// Gate phase enum
// ---------------------------------------------------------------------------

export type GatePhase =
	| "gather"
	| "present"
	| "collect"
	| "execute"
	| "learn"
	| "completed";

// ---------------------------------------------------------------------------
// Gate session — tracks the lifecycle of a single gate interaction
// ---------------------------------------------------------------------------

export interface GateSession {
	/** Unique session ID (gate name + artifact ID + timestamp). */
	id: string;
	/** The artifact being reviewed. */
	artifactId: string;
	/** The gate name from the workflow definition. */
	gateName: string;
	/** The gate pattern. */
	pattern: GatePattern;
	/** Current phase. */
	phase: GatePhase;
	/** Markdown presentation generated in PRESENT phase. */
	presentation: string;
	/** Available verdict options from COLLECT phase config. */
	verdictOptions: string[];
	/** Pre-check results from GATHER phase. */
	preCheckResults: PreCheckResult[];
	/** Summary generated in GATHER phase. */
	gatherSummary: string;
	/** Collected field values from GATHER phase. */
	gatheredFields: Record<string, unknown>;
	/** When the gate session was started. */
	startedAt: string;
	/** Reviewer verdicts collected so far (for multi_reviewer). */
	reviewerVerdicts: ReviewerVerdict[];
	/** Minimum reviewers required (from collect config). */
	minReviewers: number;
	/** AI recommendation (for structured_review). */
	aiRecommendation: string | null;
	/** Timeout configuration. */
	timeout: { duration: string; action: string } | null;
}

/** Result of a single pre-check guard. */
export interface PreCheckResult {
	description: string;
	passed: boolean;
}

/** A single reviewer's verdict (for multi_reviewer pattern). */
export interface ReviewerVerdict {
	reviewerId: string;
	verdict: string;
	rationale: string;
	timestamp: string;
}

// ---------------------------------------------------------------------------
// Gate verdict — what the reviewer submits
// ---------------------------------------------------------------------------

export interface GateVerdictInput {
	/** The verdict key (must match one of the verdictOptions). */
	verdict: string;
	/** Reviewer rationale/feedback. */
	rationale: string;
	/** Who submitted the verdict. */
	reviewerId: string;
}

// ---------------------------------------------------------------------------
// Gate result — what the engine returns after a gate completes
// ---------------------------------------------------------------------------

export interface GateResult {
	/** Whether the gate was resolved (all verdicts collected). */
	resolved: boolean;
	/** The final verdict (null if not yet resolved). */
	finalVerdict: string | null;
	/** The state to transition to (from verdict's transitions_to). */
	transitionsTo: string | null;
	/** Effects to apply (from EXECUTE phase). */
	effects: ActionEffect[];
	/** Lesson to create/update (from LEARN phase). */
	lessonAction: LessonAction | null;
	/** All collected reviewer verdicts. */
	verdicts: ReviewerVerdict[];
	/** Errors encountered. */
	errors: string[];
}

/** Action for the LEARN phase — create or update a lesson. */
export interface LessonAction {
	type: "create" | "update";
	/** The artifact ID that was rejected. */
	artifactId: string;
	/** The gate that rejected it. */
	gateName: string;
	/** The rationale from the rejection. */
	rationale: string;
	/** Whether to track recurrence. */
	trackRecurrence: boolean;
	/** Whether this should be promoted to a rule (recurrence >= threshold). */
	promoteToRule: boolean;
}

/** Cycle time data from the LEARN on_pass phase. */
export interface CycleTimeData {
	artifactId: string;
	gateName: string;
	startedAt: string;
	completedAt: string;
	durationMs: number;
}

// ---------------------------------------------------------------------------
// Session storage (in-memory — caller persists if needed)
// ---------------------------------------------------------------------------

const activeSessions = new Map<string, GateSession>();

// ---------------------------------------------------------------------------
// ID generation
// ---------------------------------------------------------------------------

function generateSessionId(
	gateName: string,
	artifactId: string,
): string {
	const ts = Date.now().toString(36);
	return `gate-${gateName}-${artifactId}-${ts}`;
}

// ---------------------------------------------------------------------------
// Phase 1: GATHER
// ---------------------------------------------------------------------------

function runGatherPhase(
	gate: Gate,
	artifact: ArtifactContext,
	actor: ActorContext,
): {
	gatheredFields: Record<string, unknown>;
	preCheckResults: PreCheckResult[];
	summary: string;
} {
	const gatherConfig = gate.phases.gather;
	const gatheredFields: Record<string, unknown> = {};
	const preCheckResults: PreCheckResult[] = [];
	let summary = "";

	// Collect specified fields
	if (gatherConfig?.fields) {
		for (const field of gatherConfig.fields) {
			if (field === "relationships") {
				gatheredFields[field] = artifact.relationships;
			} else {
				gatheredFields[field] = artifact.fields[field] ?? null;
			}
		}
	}

	// Run pre-checks
	if (gatherConfig?.pre_checks) {
		for (const guard of gatherConfig.pre_checks) {
			const { passed } = evaluateGuardsSync(
				[guard],
				artifact,
				actor,
			);
			preCheckResults.push({
				description: guard.description ?? `${guard.type} check`,
				passed,
			});
		}
	}

	// Generate summary from template
	if (gatherConfig?.summary_template) {
		summary = simpleTemplateInterpolate(
			gatherConfig.summary_template,
			artifact,
		);
	} else {
		summary = `Review for ${artifact.id}`;
	}

	return { gatheredFields, preCheckResults, summary };
}

// ---------------------------------------------------------------------------
// Phase 2: PRESENT
// ---------------------------------------------------------------------------

function runPresentPhase(
	gate: Gate,
	artifact: ArtifactContext,
	gatherResult: {
		gatheredFields: Record<string, unknown>;
		preCheckResults: PreCheckResult[];
		summary: string;
	},
): string {
	const presentConfig = gate.phases.present;
	const lines: string[] = [];

	lines.push(`# Gate Review: ${gate.description ?? "Review Required"}`);
	lines.push("");

	// Summary
	lines.push(`## Summary`);
	lines.push(gatherResult.summary);
	lines.push("");

	// Pre-check results
	if (gatherResult.preCheckResults.length > 0) {
		lines.push(`## Pre-Check Results`);
		for (const check of gatherResult.preCheckResults) {
			const icon = check.passed ? "PASS" : "FAIL";
			lines.push(`- [${icon}] ${check.description}`);
		}
		lines.push("");
	}

	// Configured sections
	if (presentConfig?.sections) {
		for (const section of presentConfig.sections) {
			lines.push(`## ${section.title}`);
			if (section.content_field) {
				const value = section.content_field === "relationships"
					? JSON.stringify(artifact.relationships, null, 2)
					: String(
							gatherResult.gatheredFields[section.content_field] ??
								artifact.fields[section.content_field] ??
								"(not available)",
						);
				lines.push(value);
			} else if (section.content_template) {
				lines.push(
					simpleTemplateInterpolate(
						section.content_template,
						artifact,
					),
				);
			}
			lines.push("");
		}
	}

	// Gathered fields
	if (Object.keys(gatherResult.gatheredFields).length > 0) {
		lines.push(`## Gathered Data`);
		for (const [key, value] of Object.entries(
			gatherResult.gatheredFields,
		)) {
			if (key === "relationships") continue; // already shown in sections
			lines.push(`- **${key}**: ${String(value ?? "(empty)")}`);
		}
		lines.push("");
	}

	return lines.join("\n");
}

// ---------------------------------------------------------------------------
// Phase 3: COLLECT — verdict validation
// ---------------------------------------------------------------------------

function validateVerdict(
	gate: Gate,
	verdict: GateVerdictInput,
): string[] {
	const errors: string[] = [];
	const collectConfig = gate.phases.collect;

	if (!collectConfig) {
		errors.push("Gate has no collect phase configured");
		return errors;
	}

	const validKeys = collectConfig.verdicts.map((v) => v.key);
	if (!validKeys.includes(verdict.verdict)) {
		errors.push(
			`Invalid verdict '${verdict.verdict}'. Valid options: ${validKeys.join(", ")}`,
		);
	}

	if (collectConfig.require_rationale && !verdict.rationale.trim()) {
		errors.push("Rationale is required for this gate");
	}

	return errors;
}

// ---------------------------------------------------------------------------
// Phase 4: EXECUTE — produce effects
// ---------------------------------------------------------------------------

async function runExecutePhase(
	gate: Gate,
	artifact: ArtifactContext,
	actor: ActorContext,
	verdict: string,
	actionHookHandler?: ActionHookHandler,
): Promise<ActionEffect[]> {
	const executeConfig = gate.phases.execute;
	if (!executeConfig?.actions) return [];

	// Create a modified artifact context with the verdict info
	const enrichedArtifact: ArtifactContext = {
		...artifact,
		fields: {
			...artifact.fields,
			verdict,
		},
	};

	return executeActions(
		executeConfig.actions,
		enrichedArtifact,
		actor,
		actionHookHandler,
	);
}

// ---------------------------------------------------------------------------
// Phase 5: LEARN — lesson creation/recurrence
// ---------------------------------------------------------------------------

const DEFAULT_PROMOTION_THRESHOLD = 3;

function runLearnPhase(
	gate: Gate,
	artifact: ArtifactContext,
	gateName: string,
	verdict: string,
	rationale: string,
	isFailure: boolean,
): LessonAction | null {
	const learnConfig = gate.phases.learn;
	if (!learnConfig) return null;

	if (isFailure && learnConfig.on_fail) {
		if (learnConfig.on_fail.create_lesson) {
			return {
				type: "create",
				artifactId: artifact.id,
				gateName,
				rationale,
				trackRecurrence: learnConfig.on_fail.track_recurrence ?? false,
				promoteToRule: false, // caller checks recurrence count
			};
		}
	}

	return null;
}

// ---------------------------------------------------------------------------
// Pattern-specific logic
// ---------------------------------------------------------------------------

/** Determine if a verdict is a "failure" for the given gate pattern. */
function isFailureVerdict(
	gate: Gate,
	verdict: string,
): boolean {
	// Verdicts that trigger the LEARN on_fail path
	const failureKeys = new Set([
		"reject",
		"request_changes",
	]);
	return failureKeys.has(verdict);
}

/** Get the transitions_to state for a verdict key. */
function getTransitionsTo(
	gate: Gate,
	verdictKey: string,
): string | null {
	const collectConfig = gate.phases.collect;
	if (!collectConfig) return null;

	const verdictDef = collectConfig.verdicts.find(
		(v) => v.key === verdictKey,
	);
	return verdictDef?.transitions_to ?? null;
}

/**
 * Check if a multi_reviewer gate has enough verdicts.
 */
function isMultiReviewerComplete(
	session: GateSession,
): boolean {
	return session.reviewerVerdicts.length >= session.minReviewers;
}

/**
 * Compute the final verdict for a multi_reviewer gate.
 * All reviewers must approve for the gate to pass.
 */
function computeMultiReviewerVerdict(
	session: GateSession,
): string {
	const allApprove = session.reviewerVerdicts.every(
		(v) => v.verdict === "approve",
	);
	return allApprove ? "approve" : "reject";
}

// ---------------------------------------------------------------------------
// Template interpolation (simplified for gate summaries)
// ---------------------------------------------------------------------------

function simpleTemplateInterpolate(
	template: string,
	artifact: ArtifactContext,
): string {
	return template.replace(
		/\$\{([^}]+)\}/g,
		(_match, key: string) => {
			if (key === "id") return artifact.id;
			if (key === "state") return artifact.state;
			if (key.startsWith("relationship_count:")) {
				const relType = key.slice("relationship_count:".length);
				return String(
					(artifact.relationships[relType] ?? []).length,
				);
			}
			if (key.startsWith("relationships:")) {
				const relType = key.slice("relationships:".length);
				const rels = artifact.relationships[relType] ?? [];
				return rels.map((r) => r.target_id).join(", ");
			}
			if (key === "relationships") {
				return JSON.stringify(artifact.relationships);
			}
			const fieldVal = artifact.fields[key];
			if (fieldVal !== undefined && fieldVal !== null) {
				return String(fieldVal);
			}
			return `\${${key}}`;
		},
	);
}

// ---------------------------------------------------------------------------
// Public API: GateEngine
// ---------------------------------------------------------------------------

export interface GateEngineOptions {
	actionHookHandler?: ActionHookHandler;
	/** Recurrence counts for existing lessons, keyed by artifact ID. */
	lessonRecurrenceCounts?: Record<string, number>;
	/** Threshold for promoting a lesson to a rule (default: 3). */
	promotionThreshold?: number;
}

/**
 * Start a gate session. Runs GATHER and PRESENT phases immediately.
 * Returns a GateSession ready for verdict collection.
 */
export function startGate(
	artifact: ArtifactContext,
	gateName: string,
	workflow: WorkflowDefinition,
	actor: ActorContext,
): GateSession {
	const gate = workflow.gates?.[gateName];
	if (!gate) {
		throw new Error(
			`Gate '${gateName}' not found in workflow '${workflow.name}'`,
		);
	}

	// Phase 1: GATHER
	const gatherResult = runGatherPhase(gate, artifact, actor);

	// Phase 2: PRESENT
	const presentation = runPresentPhase(gate, artifact, gatherResult);

	// Build verdict options
	const collectConfig = gate.phases.collect;
	const verdictOptions = collectConfig
		? collectConfig.verdicts.map((v) => v.key)
		: [];

	const minReviewers = collectConfig?.min_reviewers ?? 1;

	const session: GateSession = {
		id: generateSessionId(gateName, artifact.id),
		artifactId: artifact.id,
		gateName,
		pattern: gate.pattern,
		phase: "collect",
		presentation,
		verdictOptions,
		preCheckResults: gatherResult.preCheckResults,
		gatherSummary: gatherResult.summary,
		gatheredFields: gatherResult.gatheredFields,
		startedAt: new Date().toISOString(),
		reviewerVerdicts: [],
		minReviewers,
		aiRecommendation: null,
		timeout: gate.timeout
			? {
					duration: gate.timeout.duration,
					action: gate.timeout.action,
				}
			: null,
	};

	activeSessions.set(session.id, session);
	return session;
}

/**
 * Submit a verdict for a gate session.
 * Runs COLLECT validation, then EXECUTE and LEARN phases.
 */
export async function submitVerdict(
	session: GateSession,
	verdict: GateVerdictInput,
	workflow: WorkflowDefinition,
	artifact: ArtifactContext,
	actor: ActorContext,
	options: GateEngineOptions = {},
): Promise<GateResult> {
	const gate = workflow.gates?.[session.gateName];
	if (!gate) {
		return {
			resolved: false,
			finalVerdict: null,
			transitionsTo: null,
			effects: [],
			lessonAction: null,
			verdicts: session.reviewerVerdicts,
			errors: [
				`Gate '${session.gateName}' not found in workflow`,
			],
		};
	}

	// Phase 3: COLLECT — validate verdict
	const validationErrors = validateVerdict(gate, verdict);
	if (validationErrors.length > 0) {
		return {
			resolved: false,
			finalVerdict: null,
			transitionsTo: null,
			effects: [],
			lessonAction: null,
			verdicts: session.reviewerVerdicts,
			errors: validationErrors,
		};
	}

	// Record the reviewer verdict
	const reviewerVerdict: ReviewerVerdict = {
		reviewerId: verdict.reviewerId,
		verdict: verdict.verdict,
		rationale: verdict.rationale,
		timestamp: new Date().toISOString(),
	};
	session.reviewerVerdicts.push(reviewerVerdict);

	// Pattern-specific resolution check
	let finalVerdict: string;
	let resolved: boolean;

	switch (gate.pattern) {
		case "multi_reviewer": {
			if (!isMultiReviewerComplete(session)) {
				// Need more reviewers
				return {
					resolved: false,
					finalVerdict: null,
					transitionsTo: null,
					effects: [],
					lessonAction: null,
					verdicts: [...session.reviewerVerdicts],
					errors: [],
				};
			}
			finalVerdict = computeMultiReviewerVerdict(session);
			resolved = true;
			break;
		}
		case "structured_review": {
			// In structured review, the first verdict is the AI recommendation,
			// the second is the human verdict. If only AI has reviewed, wait.
			if (session.reviewerVerdicts.length === 1) {
				session.aiRecommendation = verdict.verdict;
				return {
					resolved: false,
					finalVerdict: null,
					transitionsTo: null,
					effects: [],
					lessonAction: null,
					verdicts: [...session.reviewerVerdicts],
					errors: [],
				};
			}
			// Human verdict is final
			finalVerdict = verdict.verdict;
			resolved = true;
			break;
		}
		case "simple_approval":
		case "escalation":
		case "scope_decision":
		default: {
			finalVerdict = verdict.verdict;
			resolved = true;
			break;
		}
	}

	if (!resolved) {
		return {
			resolved: false,
			finalVerdict: null,
			transitionsTo: null,
			effects: [],
			lessonAction: null,
			verdicts: [...session.reviewerVerdicts],
			errors: [],
		};
	}

	// Phase 4: EXECUTE
	const effects: ActionEffect[] = [];
	try {
		const executeEffects = await runExecutePhase(
			gate,
			artifact,
			actor,
			finalVerdict,
			options.actionHookHandler,
		);
		effects.push(...executeEffects);
	} catch (err) {
		return {
			resolved: true,
			finalVerdict,
			transitionsTo: getTransitionsTo(gate, finalVerdict),
			effects,
			lessonAction: null,
			verdicts: [...session.reviewerVerdicts],
			errors: [
				`EXECUTE phase error: ${err instanceof Error ? err.message : String(err)}`,
			],
		};
	}

	// Phase 5: LEARN
	const isFail = isFailureVerdict(gate, finalVerdict);
	let lessonAction = runLearnPhase(
		gate,
		artifact,
		session.gateName,
		finalVerdict,
		verdict.rationale,
		isFail,
	);

	// Check promotion threshold
	if (lessonAction && lessonAction.trackRecurrence) {
		const threshold =
			options.promotionThreshold ?? DEFAULT_PROMOTION_THRESHOLD;
		const currentCount =
			options.lessonRecurrenceCounts?.[artifact.id] ?? 0;
		// New rejection adds 1
		if (currentCount + 1 >= threshold) {
			lessonAction = { ...lessonAction, promoteToRule: true };
		}
	}

	// Update session state
	session.phase = "completed";

	// Track cycle time on pass
	const transitionsTo = getTransitionsTo(gate, finalVerdict);

	// Remove from active sessions
	activeSessions.delete(session.id);

	return {
		resolved: true,
		finalVerdict,
		transitionsTo,
		effects,
		lessonAction,
		verdicts: [...session.reviewerVerdicts],
		errors: [],
	};
}

/**
 * Set AI recommendation for a structured_review gate.
 * This is used when the AI review generates a recommendation before the
 * human reviewer sees it.
 */
export function setAiRecommendation(
	session: GateSession,
	recommendation: string,
): void {
	session.aiRecommendation = recommendation;
}

/**
 * Get all currently open gate sessions.
 */
export function getOpenGates(): GateSession[] {
	return [...activeSessions.values()].filter(
		(s) => s.phase !== "completed",
	);
}

/**
 * Get a specific gate session by ID.
 */
export function getGateSession(
	sessionId: string,
): GateSession | undefined {
	return activeSessions.get(sessionId);
}

/**
 * Clear all active gate sessions (for testing).
 */
export function clearGateSessions(): void {
	activeSessions.clear();
}

/**
 * Compute cycle time data for a completed gate (LEARN on_pass).
 */
export function computeCycleTime(
	session: GateSession,
): CycleTimeData | null {
	if (session.phase !== "completed") return null;
	const startedAt = session.startedAt;
	const completedAt = new Date().toISOString();
	const durationMs =
		new Date(completedAt).getTime() - new Date(startedAt).getTime();

	return {
		artifactId: session.artifactId,
		gateName: session.gateName,
		startedAt,
		completedAt,
		durationMs,
	};
}

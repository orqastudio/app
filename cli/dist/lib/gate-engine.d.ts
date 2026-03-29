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
import type { GatePattern, WorkflowDefinition } from "@orqastudio/types";
import { type ArtifactContext, type ActorContext, type ActionEffect, type ActionHookHandler } from "./workflow-engine.js";
export type GatePhase = "gather" | "present" | "collect" | "execute" | "learn" | "completed";
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
    timeout: {
        duration: string;
        action: string;
    } | null;
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
export interface GateVerdictInput {
    /** The verdict key (must match one of the verdictOptions). */
    verdict: string;
    /** Reviewer rationale/feedback. */
    rationale: string;
    /** Who submitted the verdict. */
    reviewerId: string;
}
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
export declare function startGate(artifact: ArtifactContext, gateName: string, workflow: WorkflowDefinition, actor: ActorContext): GateSession;
/**
 * Submit a verdict for a gate session.
 * Runs COLLECT validation, then EXECUTE and LEARN phases.
 */
export declare function submitVerdict(session: GateSession, verdict: GateVerdictInput, workflow: WorkflowDefinition, artifact: ArtifactContext, actor: ActorContext, options?: GateEngineOptions): Promise<GateResult>;
/**
 * Set AI recommendation for a structured_review gate.
 * This is used when the AI review generates a recommendation before the
 * human reviewer sees it.
 */
export declare function setAiRecommendation(session: GateSession, recommendation: string): void;
/**
 * Get all currently open gate sessions.
 */
export declare function getOpenGates(): GateSession[];
/**
 * Get a specific gate session by ID.
 */
export declare function getGateSession(sessionId: string): GateSession | undefined;
/**
 * Clear all active gate sessions (for testing).
 */
export declare function clearGateSessions(): void;
/**
 * Compute cycle time data for a completed gate (LEARN on_pass).
 */
export declare function computeCycleTime(session: GateSession): CycleTimeData | null;
//# sourceMappingURL=gate-engine.d.ts.map
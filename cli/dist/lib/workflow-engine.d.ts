/**
 * State machine evaluation engine for plugin-owned workflows.
 *
 * Loads YAML workflow definitions, resolves transitions, evaluates guards,
 * and produces execution results. The engine never modifies files directly —
 * it returns an ExecutionResult that the caller applies.
 *
 * Design constraints:
 * - Pure TypeScript, only dependency is the `yaml` package (already in CLI)
 * - Guard evaluation is synchronous for field_check/relationship_check,
 *   async for query/code_hook
 * - No backwards compatibility — breaking changes expected, data migrated
 *   via `orqa migrate`
 */
import type { WorkflowDefinition, Transition, Guard, Action, FieldCheckParams, RelationshipCheckParams, QueryGuardParams, RoleCheckParams, CodeHookGuardParams, SetFieldParams, AppendLogParams, CreateArtifactParams, NotifyParams, WorkflowVariant } from "@orqastudio/types";
export declare class WorkflowError extends Error {
    readonly code: WorkflowErrorCode;
    readonly details?: Record<string, unknown> | undefined;
    constructor(message: string, code: WorkflowErrorCode, details?: Record<string, unknown> | undefined);
}
export type WorkflowErrorCode = "LOAD_FAILED" | "PARSE_FAILED" | "VALIDATION_FAILED" | "NO_MATCHING_TRANSITION" | "GUARD_FAILED" | "GUARD_EVALUATION_ERROR" | "ACTION_EXECUTION_ERROR" | "UNKNOWN_STATE" | "UNKNOWN_EVENT" | "GATE_REQUIRED" | "HOOK_NOT_REGISTERED";
/** Minimal artifact representation the engine needs to evaluate guards. */
export interface ArtifactContext {
    /** Artifact identifier. */
    id: string;
    /** Current state in the workflow. */
    state: string;
    /** Artifact type key (must match workflow's artifact_type). */
    artifact_type: string;
    /** Frontmatter fields as a flat-ish record. */
    fields: Record<string, unknown>;
    /** Relationships grouped by type. */
    relationships: Record<string, RelationshipTarget[]>;
}
/** A relationship target with its current status. */
export interface RelationshipTarget {
    target_id: string;
    target_status?: string;
}
/** Actor context for role_check guards. */
export interface ActorContext {
    /** Actor identifier (e.g. "human:alice", "agent:reviewer"). */
    id: string;
    /** Roles the actor holds. */
    roles: string[];
}
/** Callback for graph query guards. */
export type QueryHandler = (queryName: string, args: Record<string, unknown>) => Promise<QueryResult>;
/** Result from a graph query. */
export interface QueryResult {
    count: number;
    items: unknown[];
}
/** Registered code hook function. */
export type CodeHookHandler = (hookName: string, args: Record<string, unknown>, artifact: ArtifactContext) => Promise<boolean>;
/** Registered action hook function. */
export type ActionHookHandler = (hookName: string, args: Record<string, unknown>, artifact: ArtifactContext) => Promise<ActionEffect[]>;
/** An effect that the caller must apply. */
export type ActionEffect = SetFieldEffect | AppendLogEffect | CreateArtifactEffect | NotifyEffect;
export interface SetFieldEffect {
    type: "set_field";
    field: string;
    value: unknown;
}
export interface AppendLogEffect {
    type: "append_log";
    log_field: string;
    entry: string;
}
export interface CreateArtifactEffect {
    type: "create_artifact";
    artifact_type: string;
    template?: string;
    relationship?: string;
}
export interface NotifyEffect {
    type: "notify";
    channel: string;
    message: string;
    severity: string;
}
/** Complete result of executing a transition. */
export interface ExecutionResult {
    /** Whether the transition was successful. */
    success: boolean;
    /** Previous state. */
    from_state: string;
    /** New state (only set if success). */
    to_state: string | null;
    /** The transition that fired. */
    transition: Transition | null;
    /** Effects to apply (on_exit + transition + on_enter actions). */
    effects: ActionEffect[];
    /** Errors encountered. */
    errors: WorkflowError[];
    /** Whether a human gate is required before this transition completes. */
    gate_required: string | null;
}
/** A candidate transition with its guard evaluation status. */
export interface TransitionCandidate {
    transition: Transition;
    guards_passed: boolean;
    guard_errors: string[];
}
export declare function evaluateFieldCheck(artifact: ArtifactContext, params: FieldCheckParams): boolean;
export declare function evaluateRelationshipCheck(artifact: ArtifactContext, params: RelationshipCheckParams): boolean;
export declare function evaluateRoleCheck(actor: ActorContext, params: RoleCheckParams): boolean;
export declare function evaluateQueryGuard(params: QueryGuardParams, queryHandler: QueryHandler | undefined): Promise<boolean>;
export declare function evaluateCodeHookGuard(params: CodeHookGuardParams, artifact: ArtifactContext, hookHandler: CodeHookHandler | undefined): Promise<boolean>;
export declare function executeSetField(params: SetFieldParams, artifact: ArtifactContext, actor: ActorContext): SetFieldEffect;
export declare function executeAppendLog(params: AppendLogParams, artifact: ArtifactContext, actor: ActorContext): AppendLogEffect;
export declare function executeCreateArtifact(params: CreateArtifactParams): CreateArtifactEffect;
export declare function executeNotify(params: NotifyParams, artifact: ArtifactContext, actor: ActorContext): NotifyEffect;
export declare function executeActions(actions: Action[], artifact: ArtifactContext, actor: ActorContext, actionHookHandler?: ActionHookHandler): Promise<ActionEffect[]>;
/**
 * Parse a YAML workflow file into a WorkflowDefinition.
 * Validates required fields and state references.
 */
export declare function loadWorkflow(filePath: string): WorkflowDefinition;
/**
 * Parse YAML content string into a WorkflowDefinition with validation.
 */
export declare function parseWorkflowYaml(content: string, source?: string): WorkflowDefinition;
/**
 * Load all resolved workflows from a directory.
 * Looks for *.resolved.yaml and *.workflow.yaml files.
 */
export declare function loadWorkflowsFromDir(dirPath: string): Map<string, WorkflowDefinition>;
/** Clear the workflow cache (useful for testing). */
export declare function clearWorkflowCache(): void;
/**
 * Find all transitions that match the given state and event.
 */
export declare function findMatchingTransitions(workflow: WorkflowDefinition, currentState: string, event: string): Transition[];
/**
 * Get all events available from the current state.
 */
export declare function getAvailableEvents(workflow: WorkflowDefinition, currentState: string): string[];
/**
 * Evaluate all guards on a transition synchronously (field_check, relationship_check, role_check).
 * Returns true if ALL guards pass.
 */
export declare function evaluateGuardsSync(guards: Guard[], artifact: ArtifactContext, actor: ActorContext): {
    passed: boolean;
    errors: string[];
};
/**
 * Evaluate all guards on a transition (full async support).
 */
export declare function evaluateGuards(guards: Guard[], artifact: ArtifactContext, actor: ActorContext, queryHandler?: QueryHandler, codeHookHandler?: CodeHookHandler): Promise<{
    passed: boolean;
    errors: string[];
}>;
/**
 * Resolve the first matching transition for a state + event.
 * Evaluates guards in order; first transition with all guards passing wins.
 */
export declare function resolveTransition(workflow: WorkflowDefinition, artifact: ArtifactContext, event: string, actor: ActorContext, queryHandler?: QueryHandler, codeHookHandler?: CodeHookHandler): Promise<TransitionCandidate | null>;
/**
 * Select the workflow variant to use based on artifact properties.
 * Returns the variant name and definition, or null for the base workflow.
 */
export declare function selectVariant(workflow: WorkflowDefinition, artifact: ArtifactContext, actor: ActorContext): {
    name: string;
    variant: WorkflowVariant;
} | null;
/**
 * Apply a variant to a workflow definition, producing a modified copy.
 * - Removes skip_states and their transitions
 * - Replaces/adds override_transitions
 * - Removes skip_gates references
 */
export declare function applyVariant(workflow: WorkflowDefinition, variant: WorkflowVariant): WorkflowDefinition;
/**
 * Get all transitions available from the current state, with guard evaluation status.
 * Useful for UI display of available actions.
 */
export declare function getAvailableTransitions(workflow: WorkflowDefinition, artifact: ArtifactContext, actor: ActorContext, queryHandler?: QueryHandler, codeHookHandler?: CodeHookHandler): Promise<TransitionCandidate[]>;
export interface WorkflowEngineOptions {
    queryHandler?: QueryHandler;
    codeHookHandler?: CodeHookHandler;
    actionHookHandler?: ActionHookHandler;
}
/**
 * Execute a state transition: resolve, check guards, collect effects.
 *
 * Returns an ExecutionResult that the caller applies — the engine does NOT
 * modify files or state directly.
 */
export declare function executeTransition(workflow: WorkflowDefinition, artifact: ArtifactContext, event: string, actor: ActorContext, options?: WorkflowEngineOptions): Promise<ExecutionResult>;
//# sourceMappingURL=workflow-engine.d.ts.map
/**
 * Workflow definition types for plugin-owned state machines.
 *
 * Each workflow is a YAML file that defines a complete state machine for an
 * artifact type. The core framework provides the evaluation engine and
 * guard/action primitives; plugins provide the actual definitions.
 *
 * See: workflow.schema.json for the JSON Schema these types mirror.
 */

// ---------------------------------------------------------------------------
// State Categories
// ---------------------------------------------------------------------------

/** Core state categories for cross-cutting UI treatment and aggregation. */
export type StateCategory = "planning" | "active" | "review" | "completed" | "terminal";

/** The canonical set of state categories as a runtime constant. */
export const STATE_CATEGORIES: readonly StateCategory[] = [
	"planning",
	"active",
	"review",
	"completed",
	"terminal",
] as const;

// ---------------------------------------------------------------------------
// Guards
// ---------------------------------------------------------------------------

/** Guard primitive types — declarative checks evaluated before a transition. */
export type GuardType = "field_check" | "relationship_check" | "query" | "role_check" | "code_hook";

/** Operators for field_check guards. */
export type FieldCheckOperator =
	| "exists"
	| "not_empty"
	| "equals"
	| "not_equals"
	| "in"
	| "not_in"
	| "matches";

/** Conditions for relationship_check guards. */
export type RelationshipCheckCondition =
	| "exists"
	| "min_count"
	| "all_targets_in_status"
	| "any_target_in_status"
	| "no_targets_in_status";

/** Expected result types for graph query guards. */
export type QueryExpectedResult =
	| "empty"
	| "non_empty"
	| "count_equals"
	| "count_gte"
	| "count_lte";

/** Parameters for a field_check guard. */
export interface FieldCheckParams {
	field: string;
	operator: FieldCheckOperator;
	value?: unknown;
}

/** Parameters for a relationship_check guard. */
export interface RelationshipCheckParams {
	relationship_type: string;
	condition: RelationshipCheckCondition;
	min_count?: number;
	statuses?: string[];
}

/** Parameters for a graph query guard. */
export interface QueryGuardParams {
	query_name: string;
	expected_result?: QueryExpectedResult;
	count?: number;
	args?: Record<string, unknown>;
}

/** Parameters for a role_check guard. */
export interface RoleCheckParams {
	roles: string[];
}

/** Parameters for a code_hook guard. */
export interface CodeHookGuardParams {
	hook: string;
	args?: Record<string, unknown>;
}

/** Union of all guard parameter types. */
export type GuardParams =
	| FieldCheckParams
	| RelationshipCheckParams
	| QueryGuardParams
	| RoleCheckParams
	| CodeHookGuardParams;

/** A declarative guard that must pass for a transition to fire. */
export interface Guard {
	type: GuardType;
	description?: string;
	params: GuardParams;
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

/** Action primitive types — operations executed during transitions. */
export type ActionType = "set_field" | "append_log" | "create_artifact" | "notify" | "code_hook";

/** Notification channels for notify actions. */
export type NotifyChannel = "ui" | "log" | "hook";

/** Severity levels for notifications. */
export type NotifySeverity = "info" | "warning" | "error";

/** Parameters for a set_field action. */
export interface SetFieldParams {
	field: string;
	value: unknown;
}

/** Parameters for an append_log action. */
export interface AppendLogParams {
	message: string;
	log_field?: string;
}

/** Parameters for a create_artifact action. */
export interface CreateArtifactParams {
	artifact_type: string;
	template?: string;
	relationship?: string;
}

/** Parameters for a notify action. */
export interface NotifyParams {
	channel: NotifyChannel;
	message: string;
	severity?: NotifySeverity;
}

/** Parameters for a code_hook action. */
export interface CodeHookActionParams {
	hook: string;
	args?: Record<string, unknown>;
}

/** Union of all action parameter types. */
export type ActionParams =
	| SetFieldParams
	| AppendLogParams
	| CreateArtifactParams
	| NotifyParams
	| CodeHookActionParams;

/** An action executed during a state transition or on state entry/exit. */
export interface Action {
	type: ActionType;
	description?: string;
	params: ActionParams;
}

// ---------------------------------------------------------------------------
// States
// ---------------------------------------------------------------------------

/** A state in the workflow state machine. */
export interface WorkflowState {
	category: StateCategory;
	description?: string;
	on_enter?: Action[];
	on_exit?: Action[];
}

// ---------------------------------------------------------------------------
// Transitions
// ---------------------------------------------------------------------------

/** A transition between states, triggered by a named event. */
export interface Transition {
	from: string | string[];
	to: string;
	event: string;
	description?: string;
	guards?: Guard[];
	actions?: Action[];
	gate?: string;
}

// ---------------------------------------------------------------------------
// Human Gates
// ---------------------------------------------------------------------------

/** Gate patterns determining the review mechanism. */
export type GatePattern =
	| "simple_approval"
	| "structured_review"
	| "multi_reviewer"
	| "escalation"
	| "scope_decision";

/** Timeout actions for gates. */
export type GateTimeoutAction = "escalate" | "auto_approve" | "auto_reject" | "notify";

/** Gate timeout configuration. */
export interface GateTimeout {
	duration: string;
	action: GateTimeoutAction;
}

/** A section displayed to the reviewer in the PRESENT phase. */
export interface GatePresentSection {
	title: string;
	content_field?: string;
	content_template?: string;
}

/** A verdict option in the COLLECT phase. */
export interface GateVerdict {
	key: string;
	label: string;
	transitions_to?: string;
}

/** GATHER phase: collect data and run pre-checks. */
export interface GatePhaseGather {
	fields?: string[];
	pre_checks?: Guard[];
	summary_template?: string;
}

/** PRESENT phase: configure review display. */
export interface GatePhasePresent {
	sections?: GatePresentSection[];
	context_queries?: string[];
}

/** COLLECT phase: configure verdict collection. */
export interface GatePhaseCollect {
	verdicts: GateVerdict[];
	require_rationale?: boolean;
	min_reviewers?: number;
}

/** EXECUTE phase: post-verdict actions. */
export interface GatePhaseExecute {
	actions?: Action[];
}

/** LEARN phase: post-gate learning. */
export interface GatePhaseLearn {
	on_fail?: {
		create_lesson?: boolean;
		track_recurrence?: boolean;
	};
	on_pass?: {
		track_cycle_time?: boolean;
	};
}

/** The five phases of a human gate sub-workflow. */
export interface GatePhases {
	gather?: GatePhaseGather;
	present?: GatePhasePresent;
	collect?: GatePhaseCollect;
	execute?: GatePhaseExecute;
	learn?: GatePhaseLearn;
}

/** A human gate definition — a structured sub-workflow for review/approval. */
export interface Gate {
	pattern: GatePattern;
	description?: string;
	phases: GatePhases;
	timeout?: GateTimeout;
}

// ---------------------------------------------------------------------------
// Contribution Points
// ---------------------------------------------------------------------------

/** A named slot in a workflow skeleton that stage-definition plugins fill. */
export interface ContributionPoint {
	name: string;
	stage: string;
	description?: string;
	required?: boolean;
	filled_by?: string;
}

// ---------------------------------------------------------------------------
// Variants and Selection Rules
// ---------------------------------------------------------------------------

/** A workflow variant that overrides the base workflow for specific scenarios. */
export interface WorkflowVariant {
	description?: string;
	skip_states?: string[];
	override_transitions?: Transition[];
	skip_gates?: string[];
}

/** Rule for automatically selecting a workflow variant. */
export interface SelectionRule {
	variant: string;
	conditions: Guard[];
	priority?: number;
}

// ---------------------------------------------------------------------------
// Workflow Definition
// ---------------------------------------------------------------------------

/** The complete workflow definition — a plugin-owned state machine for an artifact type. */
export interface WorkflowDefinition {
	name: string;
	version: string;
	artifact_type: string;
	plugin: string;
	description?: string;
	initial_state: string;
	contribution_points?: ContributionPoint[];
	states: Record<string, WorkflowState>;
	transitions: Transition[];
	gates?: Record<string, Gate>;
	variants?: Record<string, WorkflowVariant>;
	selection_rules?: SelectionRule[];
	migration?: Record<string, string>;
}

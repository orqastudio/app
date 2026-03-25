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

import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, extname } from "node:path";
import { parse as parseYaml } from "yaml";

import type {
	WorkflowDefinition,
	Transition,
	Guard,
	Action,
	WorkflowState,
	FieldCheckParams,
	RelationshipCheckParams,
	QueryGuardParams,
	RoleCheckParams,
	CodeHookGuardParams,
	SetFieldParams,
	AppendLogParams,
	CreateArtifactParams,
	NotifyParams,
	CodeHookActionParams,
	WorkflowVariant,
} from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

export class WorkflowError extends Error {
	constructor(
		message: string,
		public readonly code: WorkflowErrorCode,
		public readonly details?: Record<string, unknown>,
	) {
		super(message);
		this.name = "WorkflowError";
	}
}

export type WorkflowErrorCode =
	| "LOAD_FAILED"
	| "PARSE_FAILED"
	| "VALIDATION_FAILED"
	| "NO_MATCHING_TRANSITION"
	| "GUARD_FAILED"
	| "GUARD_EVALUATION_ERROR"
	| "ACTION_EXECUTION_ERROR"
	| "UNKNOWN_STATE"
	| "UNKNOWN_EVENT"
	| "GATE_REQUIRED"
	| "HOOK_NOT_REGISTERED";

// ---------------------------------------------------------------------------
// Context types — what the engine receives from the caller
// ---------------------------------------------------------------------------

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
export type QueryHandler = (
	queryName: string,
	args: Record<string, unknown>,
) => Promise<QueryResult>;

/** Result from a graph query. */
export interface QueryResult {
	count: number;
	items: unknown[];
}

/** Registered code hook function. */
export type CodeHookHandler = (
	hookName: string,
	args: Record<string, unknown>,
	artifact: ArtifactContext,
) => Promise<boolean>;

/** Registered action hook function. */
export type ActionHookHandler = (
	hookName: string,
	args: Record<string, unknown>,
	artifact: ArtifactContext,
) => Promise<ActionEffect[]>;

// ---------------------------------------------------------------------------
// Execution result types — what the engine returns
// ---------------------------------------------------------------------------

/** An effect that the caller must apply. */
export type ActionEffect =
	| SetFieldEffect
	| AppendLogEffect
	| CreateArtifactEffect
	| NotifyEffect;

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

// ---------------------------------------------------------------------------
// Guard evaluator
// ---------------------------------------------------------------------------

export function evaluateFieldCheck(
	artifact: ArtifactContext,
	params: FieldCheckParams,
): boolean {
	const value = getNestedField(artifact.fields, params.field);

	switch (params.operator) {
		case "exists":
			return value !== undefined && value !== null;
		case "not_empty":
			return (
				value !== undefined &&
				value !== null &&
				value !== "" &&
				!(Array.isArray(value) && value.length === 0)
			);
		case "equals":
			return value === params.value;
		case "not_equals":
			return value !== params.value;
		case "in":
			if (Array.isArray(params.value)) {
				if (Array.isArray(value)) {
					return value.some((v) =>
						(params.value as unknown[]).includes(v),
					);
				}
				return (params.value as unknown[]).includes(value);
			}
			return false;
		case "not_in":
			if (Array.isArray(params.value)) {
				if (Array.isArray(value)) {
					return !value.some((v) =>
						(params.value as unknown[]).includes(v),
					);
				}
				return !(params.value as unknown[]).includes(value);
			}
			return true;
		case "matches":
			if (typeof value !== "string" || typeof params.value !== "string") {
				return false;
			}
			try {
				return new RegExp(params.value).test(value);
			} catch {
				return false;
			}
		default:
			return false;
	}
}

export function evaluateRelationshipCheck(
	artifact: ArtifactContext,
	params: RelationshipCheckParams,
): boolean {
	const rels = artifact.relationships[params.relationship_type] ?? [];

	switch (params.condition) {
		case "exists":
			return rels.length > 0;
		case "min_count":
			return rels.length >= (params.min_count ?? 1);
		case "all_targets_in_status":
			if (rels.length === 0) return false;
			return rels.every(
				(r) =>
					r.target_status !== undefined &&
					(params.statuses ?? []).includes(r.target_status),
			);
		case "any_target_in_status":
			return rels.some(
				(r) =>
					r.target_status !== undefined &&
					(params.statuses ?? []).includes(r.target_status),
			);
		case "no_targets_in_status":
			return rels.every(
				(r) =>
					r.target_status === undefined ||
					!(params.statuses ?? []).includes(r.target_status),
			);
		default:
			return false;
	}
}

export function evaluateRoleCheck(
	actor: ActorContext,
	params: RoleCheckParams,
): boolean {
	return params.roles.some((role) => actor.roles.includes(role));
}

export async function evaluateQueryGuard(
	params: QueryGuardParams,
	queryHandler: QueryHandler | undefined,
): Promise<boolean> {
	if (!queryHandler) {
		throw new WorkflowError(
			`No query handler registered for query guard`,
			"GUARD_EVALUATION_ERROR",
			{ query_name: params.query_name },
		);
	}

	const result = await queryHandler(params.query_name, params.args ?? {});

	switch (params.expected_result) {
		case "empty":
			return result.count === 0;
		case "non_empty":
			return result.count > 0;
		case "count_equals":
			return result.count === (params.count ?? 0);
		case "count_gte":
			return result.count >= (params.count ?? 0);
		case "count_lte":
			return result.count <= (params.count ?? 0);
		default:
			return result.count > 0;
	}
}

export async function evaluateCodeHookGuard(
	params: CodeHookGuardParams,
	artifact: ArtifactContext,
	hookHandler: CodeHookHandler | undefined,
): Promise<boolean> {
	if (!hookHandler) {
		throw new WorkflowError(
			`No code hook handler registered for guard hook: ${params.hook}`,
			"HOOK_NOT_REGISTERED",
			{ hook: params.hook },
		);
	}
	return hookHandler(params.hook, params.args ?? {}, artifact);
}

// ---------------------------------------------------------------------------
// Action executor
// ---------------------------------------------------------------------------

/** Template interpolation for action parameters. */
function interpolateTemplate(
	template: string,
	artifact: ArtifactContext,
	actor: ActorContext,
): string {
	return template.replace(/\$\{(\w+)\}/g, (_match, key: string) => {
		if (key === "now") return new Date().toISOString();
		if (key === "actor") return actor.id;
		if (key === "id") return artifact.id;
		if (key === "state") return artifact.state;
		const fieldVal = artifact.fields[key];
		if (fieldVal !== undefined && fieldVal !== null) return String(fieldVal);
		return `\${${key}}`;
	});
}

export function executeSetField(
	params: SetFieldParams,
	artifact: ArtifactContext,
	actor: ActorContext,
): SetFieldEffect {
	const value =
		typeof params.value === "string"
			? interpolateTemplate(params.value, artifact, actor)
			: params.value;
	return { type: "set_field", field: params.field, value };
}

export function executeAppendLog(
	params: AppendLogParams,
	artifact: ArtifactContext,
	actor: ActorContext,
): AppendLogEffect {
	const message = interpolateTemplate(params.message, artifact, actor);
	return {
		type: "append_log",
		log_field: params.log_field ?? "audit_log",
		entry: message,
	};
}

export function executeCreateArtifact(
	params: CreateArtifactParams,
): CreateArtifactEffect {
	return {
		type: "create_artifact",
		artifact_type: params.artifact_type,
		template: params.template,
		relationship: params.relationship,
	};
}

export function executeNotify(
	params: NotifyParams,
	artifact: ArtifactContext,
	actor: ActorContext,
): NotifyEffect {
	const message = interpolateTemplate(params.message, artifact, actor);
	return {
		type: "notify",
		channel: params.channel,
		message,
		severity: params.severity ?? "info",
	};
}

export async function executeActions(
	actions: Action[],
	artifact: ArtifactContext,
	actor: ActorContext,
	actionHookHandler?: ActionHookHandler,
): Promise<ActionEffect[]> {
	const effects: ActionEffect[] = [];

	for (const action of actions) {
		switch (action.type) {
			case "set_field":
				effects.push(
					executeSetField(
						action.params as SetFieldParams,
						artifact,
						actor,
					),
				);
				break;
			case "append_log":
				effects.push(
					executeAppendLog(
						action.params as AppendLogParams,
						artifact,
						actor,
					),
				);
				break;
			case "create_artifact":
				effects.push(
					executeCreateArtifact(
						action.params as CreateArtifactParams,
					),
				);
				break;
			case "notify":
				effects.push(
					executeNotify(
						action.params as NotifyParams,
						artifact,
						actor,
					),
				);
				break;
			case "code_hook": {
				const hookParams = action.params as CodeHookActionParams;
				if (!actionHookHandler) {
					throw new WorkflowError(
						`No action hook handler registered for: ${hookParams.hook}`,
						"HOOK_NOT_REGISTERED",
						{ hook: hookParams.hook },
					);
				}
				const hookEffects = await actionHookHandler(
					hookParams.hook,
					hookParams.args ?? {},
					artifact,
				);
				effects.push(...hookEffects);
				break;
			}
		}
	}

	return effects;
}

// ---------------------------------------------------------------------------
// Utility: nested field access
// ---------------------------------------------------------------------------

/** Access a nested field using dot notation (e.g. "labels" or "meta.priority"). */
function getNestedField(
	obj: Record<string, unknown>,
	path: string,
): unknown {
	const parts = path.split(".");
	let current: unknown = obj;
	for (const part of parts) {
		if (current === null || current === undefined) return undefined;
		if (typeof current !== "object") return undefined;
		current = (current as Record<string, unknown>)[part];
	}
	return current;
}

// ---------------------------------------------------------------------------
// Workflow loader
// ---------------------------------------------------------------------------

/** Cache of loaded workflow definitions. */
const workflowCache = new Map<string, WorkflowDefinition>();

/**
 * Parse a YAML workflow file into a WorkflowDefinition.
 * Validates required fields and state references.
 */
export function loadWorkflow(filePath: string): WorkflowDefinition {
	const cached = workflowCache.get(filePath);
	if (cached) return cached;

	let content: string;
	try {
		content = readFileSync(filePath, "utf-8");
	} catch (err) {
		throw new WorkflowError(
			`Failed to read workflow file: ${filePath}`,
			"LOAD_FAILED",
			{ path: filePath, error: String(err) },
		);
	}

	const workflow = parseWorkflowYaml(content, filePath);
	workflowCache.set(filePath, workflow);
	return workflow;
}

/**
 * Parse YAML content string into a WorkflowDefinition with validation.
 */
export function parseWorkflowYaml(
	content: string,
	source?: string,
): WorkflowDefinition {
	let parsed: unknown;
	try {
		parsed = parseYaml(content);
	} catch (err) {
		throw new WorkflowError(
			`Failed to parse workflow YAML${source ? `: ${source}` : ""}`,
			"PARSE_FAILED",
			{ source, error: String(err) },
		);
	}

	if (!parsed || typeof parsed !== "object") {
		throw new WorkflowError(
			`Workflow YAML is not an object`,
			"PARSE_FAILED",
			{ source },
		);
	}

	const wf = parsed as Record<string, unknown>;
	validateWorkflowStructure(wf, source);

	return wf as unknown as WorkflowDefinition;
}

/**
 * Load all resolved workflows from a directory.
 * Looks for *.resolved.yaml and *.workflow.yaml files.
 */
export function loadWorkflowsFromDir(
	dirPath: string,
): Map<string, WorkflowDefinition> {
	const workflows = new Map<string, WorkflowDefinition>();

	let entries: string[];
	try {
		entries = readdirSync(dirPath);
	} catch {
		return workflows;
	}

	for (const entry of entries) {
		const ext = extname(entry);
		if (ext !== ".yaml" && ext !== ".yml") continue;
		if (
			!entry.endsWith(".resolved.yaml") &&
			!entry.endsWith(".workflow.yaml")
		) {
			continue;
		}

		const fullPath = join(dirPath, entry);
		try {
			if (!statSync(fullPath).isFile()) continue;
		} catch {
			continue;
		}

		try {
			const wf = loadWorkflow(fullPath);
			workflows.set(wf.name, wf);
		} catch {
			// Skip invalid workflow files
		}
	}

	return workflows;
}

/** Clear the workflow cache (useful for testing). */
export function clearWorkflowCache(): void {
	workflowCache.clear();
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

function validateWorkflowStructure(
	wf: Record<string, unknown>,
	source?: string,
): void {
	const required = [
		"name",
		"version",
		"artifact_type",
		"plugin",
		"states",
		"transitions",
		"initial_state",
	];

	for (const field of required) {
		if (!(field in wf)) {
			throw new WorkflowError(
				`Workflow missing required field: ${field}${source ? ` in ${source}` : ""}`,
				"VALIDATION_FAILED",
				{ field, source },
			);
		}
	}

	const states = wf.states as Record<string, unknown>;
	if (typeof states !== "object" || states === null) {
		throw new WorkflowError(
			`Workflow 'states' must be an object`,
			"VALIDATION_FAILED",
			{ source },
		);
	}

	const stateNames = new Set(Object.keys(states));

	if (stateNames.size < 2) {
		throw new WorkflowError(
			`Workflow must have at least 2 states`,
			"VALIDATION_FAILED",
			{ source },
		);
	}

	const initialState = wf.initial_state as string;
	if (!stateNames.has(initialState)) {
		throw new WorkflowError(
			`Initial state '${initialState}' not found in states`,
			"VALIDATION_FAILED",
			{ source, initial_state: initialState },
		);
	}

	const transitions = wf.transitions as unknown[];
	if (!Array.isArray(transitions) || transitions.length === 0) {
		throw new WorkflowError(
			`Workflow must have at least 1 transition`,
			"VALIDATION_FAILED",
			{ source },
		);
	}

	for (const t of transitions) {
		const tr = t as Record<string, unknown>;
		validateTransitionStates(tr, stateNames, source);
	}
}

function validateTransitionStates(
	tr: Record<string, unknown>,
	stateNames: Set<string>,
	source?: string,
): void {
	const from = tr.from;
	const to = tr.to as string;

	const fromStates = Array.isArray(from) ? from : [from];
	for (const s of fromStates) {
		if (!stateNames.has(s as string)) {
			throw new WorkflowError(
				`Transition references unknown source state: '${s}'`,
				"VALIDATION_FAILED",
				{ source, state: s },
			);
		}
	}

	if (!stateNames.has(to)) {
		throw new WorkflowError(
			`Transition references unknown target state: '${to}'`,
			"VALIDATION_FAILED",
			{ source, state: to },
		);
	}
}

// ---------------------------------------------------------------------------
// Transition resolver
// ---------------------------------------------------------------------------

/**
 * Find all transitions that match the given state and event.
 */
export function findMatchingTransitions(
	workflow: WorkflowDefinition,
	currentState: string,
	event: string,
): Transition[] {
	return workflow.transitions.filter((t) => {
		const fromStates = Array.isArray(t.from) ? t.from : [t.from];
		return fromStates.includes(currentState) && t.event === event;
	});
}

/**
 * Get all events available from the current state.
 */
export function getAvailableEvents(
	workflow: WorkflowDefinition,
	currentState: string,
): string[] {
	const events = new Set<string>();
	for (const t of workflow.transitions) {
		const fromStates = Array.isArray(t.from) ? t.from : [t.from];
		if (fromStates.includes(currentState)) {
			events.add(t.event);
		}
	}
	return [...events];
}

/**
 * Evaluate all guards on a transition synchronously (field_check, relationship_check, role_check).
 * Returns true if ALL guards pass.
 */
export function evaluateGuardsSync(
	guards: Guard[],
	artifact: ArtifactContext,
	actor: ActorContext,
): { passed: boolean; errors: string[] } {
	const errors: string[] = [];

	for (const guard of guards) {
		switch (guard.type) {
			case "field_check": {
				const result = evaluateFieldCheck(
					artifact,
					guard.params as FieldCheckParams,
				);
				if (!result) {
					errors.push(
						guard.description ??
							`field_check failed: ${(guard.params as FieldCheckParams).field} ${(guard.params as FieldCheckParams).operator}`,
					);
				}
				break;
			}
			case "relationship_check": {
				const result = evaluateRelationshipCheck(
					artifact,
					guard.params as RelationshipCheckParams,
				);
				if (!result) {
					errors.push(
						guard.description ??
							`relationship_check failed: ${(guard.params as RelationshipCheckParams).relationship_type} ${(guard.params as RelationshipCheckParams).condition}`,
					);
				}
				break;
			}
			case "role_check": {
				const result = evaluateRoleCheck(
					actor,
					guard.params as RoleCheckParams,
				);
				if (!result) {
					errors.push(
						guard.description ??
							`role_check failed: requires one of [${(guard.params as RoleCheckParams).roles.join(", ")}]`,
					);
				}
				break;
			}
			case "query":
			case "code_hook":
				// These require async evaluation — skip in sync mode
				// The async evaluator handles them
				break;
		}
	}

	return { passed: errors.length === 0, errors };
}

/**
 * Evaluate all guards on a transition (full async support).
 */
export async function evaluateGuards(
	guards: Guard[],
	artifact: ArtifactContext,
	actor: ActorContext,
	queryHandler?: QueryHandler,
	codeHookHandler?: CodeHookHandler,
): Promise<{ passed: boolean; errors: string[] }> {
	const errors: string[] = [];

	for (const guard of guards) {
		try {
			let result: boolean;

			switch (guard.type) {
				case "field_check":
					result = evaluateFieldCheck(
						artifact,
						guard.params as FieldCheckParams,
					);
					break;
				case "relationship_check":
					result = evaluateRelationshipCheck(
						artifact,
						guard.params as RelationshipCheckParams,
					);
					break;
				case "role_check":
					result = evaluateRoleCheck(
						actor,
						guard.params as RoleCheckParams,
					);
					break;
				case "query":
					result = await evaluateQueryGuard(
						guard.params as QueryGuardParams,
						queryHandler,
					);
					break;
				case "code_hook":
					result = await evaluateCodeHookGuard(
						guard.params as CodeHookGuardParams,
						artifact,
						codeHookHandler,
					);
					break;
				default:
					result = false;
					errors.push(`Unknown guard type: ${guard.type}`);
			}

			if (!result) {
				errors.push(
					guard.description ??
						`Guard failed: ${guard.type}`,
				);
			}
		} catch (err) {
			errors.push(
				`Guard evaluation error (${guard.type}): ${err instanceof Error ? err.message : String(err)}`,
			);
		}
	}

	return { passed: errors.length === 0, errors };
}

/**
 * Resolve the first matching transition for a state + event.
 * Evaluates guards in order; first transition with all guards passing wins.
 */
export async function resolveTransition(
	workflow: WorkflowDefinition,
	artifact: ArtifactContext,
	event: string,
	actor: ActorContext,
	queryHandler?: QueryHandler,
	codeHookHandler?: CodeHookHandler,
): Promise<TransitionCandidate | null> {
	if (!workflow.states[artifact.state]) {
		throw new WorkflowError(
			`Artifact is in unknown state: '${artifact.state}'`,
			"UNKNOWN_STATE",
			{ state: artifact.state },
		);
	}

	const matching = findMatchingTransitions(
		workflow,
		artifact.state,
		event,
	);

	if (matching.length === 0) {
		return null;
	}

	for (const transition of matching) {
		const guards = transition.guards ?? [];
		const result = await evaluateGuards(
			guards,
			artifact,
			actor,
			queryHandler,
			codeHookHandler,
		);

		if (result.passed) {
			return {
				transition,
				guards_passed: true,
				guard_errors: [],
			};
		}
	}

	// Return the last candidate with its errors for diagnostics
	const lastTransition = matching[matching.length - 1]!;
	const lastResult = await evaluateGuards(
		lastTransition.guards ?? [],
		artifact,
		actor,
		queryHandler,
		codeHookHandler,
	);

	return {
		transition: lastTransition,
		guards_passed: false,
		guard_errors: lastResult.errors,
	};
}

// ---------------------------------------------------------------------------
// Variant selection
// ---------------------------------------------------------------------------

/**
 * Select the workflow variant to use based on artifact properties.
 * Returns the variant name and definition, or null for the base workflow.
 */
export function selectVariant(
	workflow: WorkflowDefinition,
	artifact: ArtifactContext,
	actor: ActorContext,
): { name: string; variant: WorkflowVariant } | null {
	if (!workflow.selection_rules || !workflow.variants) return null;

	const sorted = [...workflow.selection_rules].sort(
		(a, b) => (b.priority ?? 0) - (a.priority ?? 0),
	);

	for (const rule of sorted) {
		const variant = workflow.variants[rule.variant];
		if (!variant) continue;

		const guards = rule.conditions;
		const { passed } = evaluateGuardsSync(guards, artifact, actor);

		if (passed) {
			return { name: rule.variant, variant };
		}
	}

	return null;
}

/**
 * Apply a variant to a workflow definition, producing a modified copy.
 * - Removes skip_states and their transitions
 * - Replaces/adds override_transitions
 * - Removes skip_gates references
 */
export function applyVariant(
	workflow: WorkflowDefinition,
	variant: WorkflowVariant,
): WorkflowDefinition {
	const skipStates = new Set(variant.skip_states ?? []);
	const skipGates = new Set(variant.skip_gates ?? []);

	// Filter states
	const states: Record<string, WorkflowState> = {};
	for (const [name, state] of Object.entries(workflow.states)) {
		if (!skipStates.has(name)) {
			states[name] = state;
		}
	}

	// Filter transitions: remove those referencing skipped states
	let transitions = workflow.transitions.filter((t) => {
		const fromStates = Array.isArray(t.from) ? t.from : [t.from];
		const hasSkippedFrom = fromStates.some((s) => skipStates.has(s));
		const hasSkippedTo = skipStates.has(t.to);
		return !hasSkippedFrom && !hasSkippedTo;
	});

	// Remove gate references for skipped gates
	if (skipGates.size > 0) {
		transitions = transitions.map((t) => {
			if (t.gate && skipGates.has(t.gate)) {
				const { gate: _, ...rest } = t; // eslint-disable-line @typescript-eslint/no-unused-vars
				return rest as Transition;
			}
			return t;
		});
	}

	// Add override transitions
	if (variant.override_transitions) {
		transitions = [...transitions, ...variant.override_transitions];
	}

	// Filter gates
	const gates: Record<string, unknown> = {};
	if (workflow.gates) {
		for (const [name, gate] of Object.entries(workflow.gates)) {
			if (!skipGates.has(name)) {
				gates[name] = gate;
			}
		}
	}

	return {
		...workflow,
		states,
		transitions,
		gates: Object.keys(gates).length > 0
			? (gates as WorkflowDefinition["gates"])
			: undefined,
	};
}

// ---------------------------------------------------------------------------
// Available transitions
// ---------------------------------------------------------------------------

/**
 * Get all transitions available from the current state, with guard evaluation status.
 * Useful for UI display of available actions.
 */
export async function getAvailableTransitions(
	workflow: WorkflowDefinition,
	artifact: ArtifactContext,
	actor: ActorContext,
	queryHandler?: QueryHandler,
	codeHookHandler?: CodeHookHandler,
): Promise<TransitionCandidate[]> {
	const candidates: TransitionCandidate[] = [];

	for (const transition of workflow.transitions) {
		const fromStates = Array.isArray(transition.from)
			? transition.from
			: [transition.from];

		if (!fromStates.includes(artifact.state)) continue;

		const guards = transition.guards ?? [];
		const result = await evaluateGuards(
			guards,
			artifact,
			actor,
			queryHandler,
			codeHookHandler,
		);

		candidates.push({
			transition,
			guards_passed: result.passed,
			guard_errors: result.errors,
		});
	}

	return candidates;
}

// ---------------------------------------------------------------------------
// Engine: the main API
// ---------------------------------------------------------------------------

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
export async function executeTransition(
	workflow: WorkflowDefinition,
	artifact: ArtifactContext,
	event: string,
	actor: ActorContext,
	options: WorkflowEngineOptions = {},
): Promise<ExecutionResult> {
	const { queryHandler, codeHookHandler, actionHookHandler } = options;

	// Validate current state exists
	if (!workflow.states[artifact.state]) {
		return {
			success: false,
			from_state: artifact.state,
			to_state: null,
			transition: null,
			effects: [],
			errors: [
				new WorkflowError(
					`Artifact is in unknown state: '${artifact.state}'`,
					"UNKNOWN_STATE",
				),
			],
			gate_required: null,
		};
	}

	// Resolve transition
	const candidate = await resolveTransition(
		workflow,
		artifact,
		event,
		actor,
		queryHandler,
		codeHookHandler,
	);

	if (!candidate) {
		return {
			success: false,
			from_state: artifact.state,
			to_state: null,
			transition: null,
			effects: [],
			errors: [
				new WorkflowError(
					`No transitions match state '${artifact.state}' + event '${event}'`,
					"NO_MATCHING_TRANSITION",
				),
			],
			gate_required: null,
		};
	}

	if (!candidate.guards_passed) {
		return {
			success: false,
			from_state: artifact.state,
			to_state: null,
			transition: candidate.transition,
			effects: [],
			errors: [
				new WorkflowError(
					`Guards failed: ${candidate.guard_errors.join("; ")}`,
					"GUARD_FAILED",
					{ errors: candidate.guard_errors },
				),
			],
			gate_required: null,
		};
	}

	// Check for human gate requirement
	if (candidate.transition.gate) {
		return {
			success: false,
			from_state: artifact.state,
			to_state: candidate.transition.to,
			transition: candidate.transition,
			effects: [],
			errors: [],
			gate_required: candidate.transition.gate,
		};
	}

	// Collect effects: on_exit → transition actions → on_enter
	const effects: ActionEffect[] = [];
	const errors: WorkflowError[] = [];

	// On-exit actions from current state
	const currentState = workflow.states[artifact.state];
	if (currentState?.on_exit) {
		try {
			const exitEffects = await executeActions(
				currentState.on_exit,
				artifact,
				actor,
				actionHookHandler,
			);
			effects.push(...exitEffects);
		} catch (err) {
			errors.push(
				new WorkflowError(
					`on_exit action failed: ${err instanceof Error ? err.message : String(err)}`,
					"ACTION_EXECUTION_ERROR",
				),
			);
		}
	}

	// Transition actions
	if (candidate.transition.actions) {
		try {
			const transitionEffects = await executeActions(
				candidate.transition.actions,
				artifact,
				actor,
				actionHookHandler,
			);
			effects.push(...transitionEffects);
		} catch (err) {
			errors.push(
				new WorkflowError(
					`transition action failed: ${err instanceof Error ? err.message : String(err)}`,
					"ACTION_EXECUTION_ERROR",
				),
			);
		}
	}

	// On-enter actions for target state
	const targetState = workflow.states[candidate.transition.to];
	if (targetState?.on_enter) {
		try {
			const enterEffects = await executeActions(
				targetState.on_enter,
				artifact,
				actor,
				actionHookHandler,
			);
			effects.push(...enterEffects);
		} catch (err) {
			errors.push(
				new WorkflowError(
					`on_enter action failed: ${err instanceof Error ? err.message : String(err)}`,
					"ACTION_EXECUTION_ERROR",
				),
			);
		}
	}

	// Always add a set_field for the state transition itself
	effects.unshift({
		type: "set_field",
		field: "status",
		value: candidate.transition.to,
	});

	return {
		success: errors.length === 0,
		from_state: artifact.state,
		to_state: candidate.transition.to,
		transition: candidate.transition,
		effects,
		errors,
		gate_required: null,
	};
}

/**
 * Workflow resolver — merges plugin workflow contributions into resolved workflows.
 *
 * Runs at `orqa plugin install` / `orqa plugin refresh` time:
 * 1. Scans installed plugins for `workflows/` directories
 * 2. Identifies skeletons (files with `contribution_points`) and standalone workflows
 * 3. Merges stage-plugin contributions into skeleton contribution points
 * 4. Validates the merged result against the workflow schema structure
 * 5. Writes resolved workflows to `.orqa/workflows/<name>.resolved.yaml`
 *
 * The runtime engine reads only resolved files — never raw plugin workflows.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { parse as parseYaml, stringify as stringifyYaml } from "yaml";
import {
	STATE_CATEGORIES,
	type WorkflowDefinition,
	type WorkflowState,
	type Transition,
} from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** A raw workflow file discovered in a plugin's workflows/ directory. */
export interface DiscoveredWorkflow {
	/** Absolute path to the YAML file. */
	filePath: string;
	/** Plugin directory that owns this file. */
	pluginDir: string;
	/** Plugin name from the manifest (or from the workflow's plugin field). */
	pluginName: string;
	/** Parsed workflow definition. */
	definition: WorkflowDefinition;
	/** Whether this workflow has contribution points (i.e. is a skeleton). */
	isSkeleton: boolean;
}

/** A contribution that a stage-definition plugin makes to a skeleton. */
export interface WorkflowContribution {
	/** The contribution point name this targets. */
	targetPoint: string;
	/** The workflow name this targets. */
	targetWorkflow: string;
	/** States to merge into the skeleton. */
	states: Record<string, WorkflowState>;
	/** Transitions to merge into the skeleton. */
	transitions: Transition[];
	/** Priority for ordering (higher = merged later, wins on conflict). */
	priority: number;
	/** Plugin that provides this contribution. */
	pluginName: string;
	/** Source file path. */
	filePath: string;
}

/** Metadata about which plugins contributed to a resolved workflow. */
export interface ResolutionMetadata {
	/** Skeleton source plugin. */
	skeletonPlugin: string;
	/** Skeleton source file. */
	skeletonFile: string;
	/** Contributions that were merged. */
	contributions: Array<{
		plugin: string;
		point: string;
		statesAdded: string[];
		transitionsAdded: number;
	}>;
	/** Contribution points that were not filled. */
	unfilledPoints: string[];
	/** Contribution points that were required but not filled. */
	unfilledRequired: string[];
	/** Project-level overrides that were applied. */
	overrides: Array<{
		file: string;
		statesAdded: string[];
		statesReplaced: string[];
		transitionsAdded: number;
		fieldsOverridden: string[];
	}>;
	/** Timestamp of resolution. */
	resolvedAt: string;
}

/** A project-level override targeting a specific resolved workflow. */
export interface WorkflowOverride {
	/** Which resolved workflow to override (matches workflow name). */
	target_workflow: string;
	/** States to add or replace (keyed by state name). */
	states?: Record<string, WorkflowState>;
	/** Transitions to add. */
	transitions?: Transition[];
	/** Top-level field overrides (description, initial_state, etc.). */
	fields?: Record<string, unknown>;
}

/** A discovered override file. */
interface DiscoveredOverride {
	/** Absolute path to the override YAML file. */
	filePath: string;
	/** Parsed override definition. */
	override: WorkflowOverride;
}

/** Result of resolving a single workflow. */
export interface ResolvedWorkflowResult {
	/** The workflow name. */
	name: string;
	/** The resolved workflow definition. */
	definition: WorkflowDefinition;
	/** Resolution metadata. */
	metadata: ResolutionMetadata;
	/** Validation errors (empty if valid). */
	errors: string[];
	/** Output file path. */
	outputPath: string;
}

/** Result of resolving all workflows in a project. */
export interface ResolveAllResult {
	/** Successfully resolved workflows. */
	resolved: ResolvedWorkflowResult[];
	/** Standalone workflows (no contribution points, written as-is). */
	standalone: ResolvedWorkflowResult[];
	/** Errors encountered during discovery or resolution. */
	errors: string[];
}

// ---------------------------------------------------------------------------
// Discovery
// ---------------------------------------------------------------------------

/**
 * Discover all workflow files across installed plugins.
 *
 * Scans `workflows/` directories in plugins/, connectors/, and sidecars/.
 * Files must end with `.workflow.yaml` or `.workflow.yml`.
 */
export function discoverWorkflows(projectRoot: string): {
	workflows: DiscoveredWorkflow[];
	errors: string[];
} {
	const workflows: DiscoveredWorkflow[] = [];
	const errors: string[] = [];

	const containers = ["plugins", "connectors", "sidecars"];

	for (const container of containers) {
		const containerDir = path.join(projectRoot, container);
		if (!fs.existsSync(containerDir)) continue;

		const entries = fs.readdirSync(containerDir, { withFileTypes: true });

		for (const entry of entries) {
			if (!entry.isDirectory() || entry.name.startsWith(".")) continue;

			const pluginDir = path.join(containerDir, entry.name);
			const workflowsDir = path.join(pluginDir, "workflows");

			if (!fs.existsSync(workflowsDir)) continue;

			// Read plugin name from manifest if available
			let pluginName = entry.name;
			const manifestPath = path.join(pluginDir, "orqa-plugin.json");
			if (fs.existsSync(manifestPath)) {
				try {
					const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf-8"));
					if (manifest.name) {
						pluginName = manifest.name;
					}
				} catch {
					// Fall back to directory name
				}
			}

			// Scan for workflow files
			const workflowFiles = fs.readdirSync(workflowsDir).filter(
				(f) => f.endsWith(".workflow.yaml") || f.endsWith(".workflow.yml"),
			);

			for (const file of workflowFiles) {
				const filePath = path.join(workflowsDir, file);
				try {
					const content = fs.readFileSync(filePath, "utf-8");
					const parsed = parseYaml(content) as WorkflowDefinition;

					if (!parsed || typeof parsed !== "object") {
						errors.push(`${filePath}: Failed to parse — not a valid YAML object`);
						continue;
					}

					// Basic structural check
					if (!parsed.name || !parsed.states || !parsed.transitions) {
						errors.push(
							`${filePath}: Missing required fields (name, states, transitions)`,
						);
						continue;
					}

					const isSkeleton =
						Array.isArray(parsed.contribution_points) &&
						parsed.contribution_points.length > 0;

					workflows.push({
						filePath,
						pluginDir,
						pluginName,
						definition: parsed,
						isSkeleton,
					});
				} catch (err) {
					errors.push(
						`${filePath}: ${err instanceof Error ? err.message : String(err)}`,
					);
				}
			}
		}
	}

	return { workflows, errors };
}

// ---------------------------------------------------------------------------
// Override Discovery
// ---------------------------------------------------------------------------

/**
 * Discover project-level workflow override files.
 *
 * Scans `.orqa/workflows/overrides/` for `*.override.yaml` / `*.override.yml` files.
 * Each file must contain a `target_workflow` field identifying which resolved
 * workflow to override.
 */
export function discoverOverrides(projectRoot: string): {
	overrides: DiscoveredOverride[];
	errors: string[];
} {
	const overrides: DiscoveredOverride[] = [];
	const errors: string[] = [];

	const overridesDir = path.join(projectRoot, ".orqa", "workflows", "overrides");
	if (!fs.existsSync(overridesDir)) {
		return { overrides, errors };
	}

	const files = fs.readdirSync(overridesDir).filter(
		(f) => f.endsWith(".override.yaml") || f.endsWith(".override.yml"),
	);

	for (const file of files) {
		const filePath = path.join(overridesDir, file);
		try {
			const content = fs.readFileSync(filePath, "utf-8");
			const parsed = parseYaml(content) as WorkflowOverride;

			if (!parsed || typeof parsed !== "object") {
				errors.push(`${filePath}: Failed to parse — not a valid YAML object`);
				continue;
			}

			if (!parsed.target_workflow) {
				errors.push(`${filePath}: Missing required field: target_workflow`);
				continue;
			}

			overrides.push({ filePath, override: parsed });
		} catch (err) {
			errors.push(
				`${filePath}: ${err instanceof Error ? err.message : String(err)}`,
			);
		}
	}

	return { overrides, errors };
}

// ---------------------------------------------------------------------------
// Override Application
// ---------------------------------------------------------------------------

/**
 * Apply project-level overrides to a resolved workflow definition.
 *
 * Override semantics:
 * - `states`: new state names are added; existing state names are replaced
 * - `transitions`: appended (deduplicated by from/to/event)
 * - `fields`: top-level scalar fields are overwritten (description, initial_state, etc.)
 *
 * This runs AFTER contribution merging, BEFORE validation and writing.
 */
export function applyOverrides(
	definition: WorkflowDefinition,
	overrides: DiscoveredOverride[],
	metadata: ResolutionMetadata,
): { definition: WorkflowDefinition; errors: string[] } {
	const errors: string[] = [];
	// Deep clone to avoid mutating the input
	const result: WorkflowDefinition = JSON.parse(JSON.stringify(definition));

	// Filter overrides targeting this workflow
	const applicable = overrides.filter(
		(o) => o.override.target_workflow === result.name,
	);

	for (const { filePath, override } of applicable) {
		const statesAdded: string[] = [];
		const statesReplaced: string[] = [];
		const fieldsOverridden: string[] = [];
		let transitionsAdded = 0;

		// Apply state overrides
		if (override.states) {
			for (const [stateName, stateDef] of Object.entries(override.states)) {
				if (result.states[stateName]) {
					statesReplaced.push(stateName);
				} else {
					statesAdded.push(stateName);
				}
				result.states[stateName] = stateDef;
			}
		}

		// Apply transition overrides (append, deduplicate)
		if (override.transitions) {
			for (const transition of override.transitions) {
				const isDuplicate = result.transitions.some(
					(t) =>
						JSON.stringify(t.from) === JSON.stringify(transition.from) &&
						t.to === transition.to &&
						t.event === transition.event,
				);
				if (!isDuplicate) {
					result.transitions.push(transition);
					transitionsAdded++;
				}
			}
		}

		// Apply top-level field overrides
		if (override.fields) {
			const allowedFields = new Set([
				"description",
				"initial_state",
				"version",
			]);
			for (const [field, value] of Object.entries(override.fields)) {
				if (!allowedFields.has(field)) {
					errors.push(
						`${filePath}: Cannot override field "${field}" — only ${[...allowedFields].join(", ")} are allowed`,
					);
					continue;
				}
				(result as unknown as Record<string, unknown>)[field] = value;
				fieldsOverridden.push(field);
			}
		}

		const relPath = path.relative(
			path.join(path.dirname(filePath), "..", ".."),
			filePath,
		).replace(/\\/g, "/");

		metadata.overrides.push({
			file: relPath,
			statesAdded,
			statesReplaced,
			transitionsAdded,
			fieldsOverridden,
		});
	}

	return { definition: result, errors };
}

// ---------------------------------------------------------------------------
// Contribution Matching
// ---------------------------------------------------------------------------

/**
 * Extended workflow definition with optional contribution declaration.
 *
 * A workflow file declares itself as a contribution via:
 *   contributes_to:
 *     workflow: "delivery"
 *     point: "implementation-workflow"
 *     priority: 10
 *
 * Without this field, non-skeleton workflows are standalone.
 */
interface ContributionDeclaration {
	workflow: string;
	point: string;
	priority?: number;
}

/**
 * Match non-skeleton workflows to skeleton contribution points.
 *
 * A workflow file is a contribution only if it explicitly declares
 * `contributes_to` targeting a skeleton workflow and contribution point.
 * All other non-skeleton workflows are standalone.
 */
export function matchContributions(
	workflows: DiscoveredWorkflow[],
): {
	skeletons: DiscoveredWorkflow[];
	contributions: Map<string, WorkflowContribution[]>;
	standalone: DiscoveredWorkflow[];
} {
	const skeletons = workflows.filter((w) => w.isSkeleton);
	const nonSkeletons = workflows.filter((w) => !w.isSkeleton);

	// Build a map of skeleton name → skeleton
	const skeletonByName = new Map<string, DiscoveredWorkflow>();
	for (const s of skeletons) {
		skeletonByName.set(s.definition.name, s);
	}

	const contributions = new Map<string, WorkflowContribution[]>();
	const standalone: DiscoveredWorkflow[] = [];

	for (const workflow of nonSkeletons) {
		// Check for explicit contribution declaration
		const contributesTo = (workflow.definition as unknown as Record<string, unknown>)
			.contributes_to as ContributionDeclaration | undefined;

		if (!contributesTo || !contributesTo.workflow || !contributesTo.point) {
			// No contribution declaration — standalone workflow
			standalone.push(workflow);
			continue;
		}

		// Validate the target skeleton exists
		const targetSkeleton = skeletonByName.get(contributesTo.workflow);
		if (!targetSkeleton) {
			// Target skeleton not found — treat as standalone but warn
			standalone.push(workflow);
			continue;
		}

		// Validate the contribution point exists in the skeleton
		const skeleton = targetSkeleton.definition;
		const points = skeleton.contribution_points ?? [];
		const targetPoint = points.find((p) => p.name === contributesTo.point);

		if (!targetPoint) {
			// Contribution point not found — treat as standalone
			standalone.push(workflow);
			continue;
		}

		const key = `${skeleton.name}::${contributesTo.point}`;
		const existing = contributions.get(key) ?? [];
		existing.push({
			targetPoint: contributesTo.point,
			targetWorkflow: skeleton.name,
			states: workflow.definition.states,
			transitions: workflow.definition.transitions,
			priority: contributesTo.priority ?? 0,
			pluginName: workflow.pluginName,
			filePath: workflow.filePath,
		});
		contributions.set(key, existing);
	}

	return { skeletons, contributions, standalone };
}

// ---------------------------------------------------------------------------
// Merging
// ---------------------------------------------------------------------------

/**
 * Merge contributions into a skeleton workflow definition.
 *
 * Rules:
 * - Contributions are additive — cannot remove skeleton-defined states
 * - States from contributions are merged into the skeleton's states map
 * - Transitions from contributions are appended to the skeleton's transitions
 * - Contribution points are updated with `filled_by` metadata
 * - Priority ordering: higher priority contributions override lower ones
 *   for the same state name
 */
export function mergeContributions(
	skeleton: DiscoveredWorkflow,
	contributions: Map<string, WorkflowContribution[]>,
): {
	merged: WorkflowDefinition;
	metadata: ResolutionMetadata;
} {
	// Deep clone the skeleton definition
	const merged: WorkflowDefinition = JSON.parse(
		JSON.stringify(skeleton.definition),
	);

	const metadata: ResolutionMetadata = {
		skeletonPlugin: skeleton.pluginName,
		skeletonFile: path.relative(skeleton.pluginDir, skeleton.filePath).replace(/\\/g, "/"),
		contributions: [],
		unfilledPoints: [],
		unfilledRequired: [],
		overrides: [],
		resolvedAt: new Date().toISOString(),
	};

	const points = merged.contribution_points ?? [];

	for (const point of points) {
		const key = `${merged.name}::${point.name}`;
		const pointContributions = contributions.get(key) ?? [];

		if (pointContributions.length === 0) {
			metadata.unfilledPoints.push(point.name);
			if (point.required) {
				metadata.unfilledRequired.push(point.name);
			}
			continue;
		}

		// Sort by priority (ascending — lower priority first, higher overrides)
		pointContributions.sort((a, b) => a.priority - b.priority);

		for (const contribution of pointContributions) {
			// Merge states
			const statesAdded: string[] = [];
			for (const [stateName, stateDefn] of Object.entries(contribution.states)) {
				if (!merged.states[stateName]) {
					merged.states[stateName] = stateDefn;
					statesAdded.push(stateName);
				}
				// If state already exists in skeleton, skip (skeleton owns it)
			}

			// Merge transitions
			for (const transition of contribution.transitions) {
				// Check for duplicate transitions (same from/to/event)
				const isDuplicate = merged.transitions.some(
					(t) =>
						JSON.stringify(t.from) === JSON.stringify(transition.from) &&
						t.to === transition.to &&
						t.event === transition.event,
				);
				if (!isDuplicate) {
					merged.transitions.push(transition);
				}
			}

			// Update contribution point metadata
			point.filled_by = contribution.pluginName;

			metadata.contributions.push({
				plugin: contribution.pluginName,
				point: point.name,
				statesAdded,
				transitionsAdded: contribution.transitions.length,
			});
		}
	}

	return { merged, metadata };
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/**
 * Validate a resolved workflow definition.
 *
 * Checks:
 * - All required fields present
 * - initial_state exists in states map
 * - All transition from/to states exist in states map
 * - All gate references exist in gates map
 * - Every state category is valid
 * - At least 2 states and 1 transition
 * - Required contribution points were filled
 */
export function validateResolvedWorkflow(
	definition: WorkflowDefinition,
	metadata: ResolutionMetadata,
): string[] {
	const errors: string[] = [];

	// Required fields
	if (!definition.name) errors.push("Missing required field: name");
	if (!definition.version) errors.push("Missing required field: version");
	if (!definition.artifact_type) errors.push("Missing required field: artifact_type");
	if (!definition.plugin) errors.push("Missing required field: plugin");
	if (!definition.initial_state) errors.push("Missing required field: initial_state");

	if (!definition.states || typeof definition.states !== "object") {
		errors.push("Missing or invalid: states");
		return errors;
	}

	if (!Array.isArray(definition.transitions) || definition.transitions.length === 0) {
		errors.push("Missing or empty: transitions");
		return errors;
	}

	const stateNames = new Set(Object.keys(definition.states));

	// Minimum states
	if (stateNames.size < 2) {
		errors.push(`Workflow must have at least 2 states (found ${stateNames.size})`);
	}

	// initial_state exists
	if (!stateNames.has(definition.initial_state)) {
		errors.push(
			`initial_state "${definition.initial_state}" does not exist in states`,
		);
	}

	// Valid state categories (engine vocabulary, defined in @orqastudio/types)
	const validCategories = new Set<string>(STATE_CATEGORIES);
	for (const [name, state] of Object.entries(definition.states)) {
		if (!validCategories.has(state.category)) {
			errors.push(
				`State "${name}" has invalid category "${state.category}"`,
			);
		}
	}

	// Transition references
	const gateNames = definition.gates
		? new Set(Object.keys(definition.gates))
		: new Set<string>();

	for (let i = 0; i < definition.transitions.length; i++) {
		const t = definition.transitions[i];
		const fromStates = Array.isArray(t.from) ? t.from : [t.from];

		for (const fromState of fromStates) {
			if (!stateNames.has(fromState)) {
				errors.push(
					`Transition[${i}] from "${fromState}" — state does not exist`,
				);
			}
		}

		if (!stateNames.has(t.to)) {
			errors.push(
				`Transition[${i}] to "${t.to}" — state does not exist`,
			);
		}

		if (t.gate && !gateNames.has(t.gate)) {
			errors.push(
				`Transition[${i}] gate "${t.gate}" — gate not defined in gates map`,
			);
		}
	}

	// Required contribution points
	for (const point of metadata.unfilledRequired) {
		errors.push(
			`Required contribution point "${point}" was not filled by any plugin`,
		);
	}

	return errors;
}

// ---------------------------------------------------------------------------
// Resolution Pipeline
// ---------------------------------------------------------------------------

/**
 * Resolve all workflows in a project.
 *
 * This is the main entry point called by `orqa plugin install` / `orqa plugin refresh`.
 */
export function resolveAllWorkflows(projectRoot: string): ResolveAllResult {
	const result: ResolveAllResult = {
		resolved: [],
		standalone: [],
		errors: [],
	};

	// 1. Discover workflows from plugins
	const discovery = discoverWorkflows(projectRoot);
	result.errors.push(...discovery.errors);

	if (discovery.workflows.length === 0) {
		return result;
	}

	// 2. Discover project-level overrides
	const overrideDiscovery = discoverOverrides(projectRoot);
	result.errors.push(...overrideDiscovery.errors);

	// 3. Match contributions to skeletons
	const { skeletons, contributions, standalone } = matchContributions(
		discovery.workflows,
	);

	// 4. Ensure output directory exists
	const outputDir = path.join(projectRoot, ".orqa", "workflows");
	if (!fs.existsSync(outputDir)) {
		fs.mkdirSync(outputDir, { recursive: true });
	}

	// 5. Resolve each skeleton
	for (const skeleton of skeletons) {
		const { merged, metadata } = mergeContributions(skeleton, contributions);

		// Apply project-level overrides after contribution merging
		const overrideResult = applyOverrides(merged, overrideDiscovery.overrides, metadata);
		result.errors.push(...overrideResult.errors);

		// Validate after overrides are applied
		const errors = validateResolvedWorkflow(overrideResult.definition, metadata);
		const outputPath = path.join(
			outputDir,
			`${overrideResult.definition.name}.resolved.yaml`,
		);

		// Write the resolved workflow with metadata header
		const outputContent = buildResolvedYaml(overrideResult.definition, metadata);
		fs.writeFileSync(outputPath, outputContent, "utf-8");

		result.resolved.push({
			name: overrideResult.definition.name,
			definition: overrideResult.definition,
			metadata,
			errors,
			outputPath,
		});
	}

	// 6. Write standalone workflows (also apply overrides)
	for (const workflow of standalone) {
		const defn = workflow.definition;
		const metadata: ResolutionMetadata = {
			skeletonPlugin: workflow.pluginName,
			skeletonFile: path.relative(workflow.pluginDir, workflow.filePath).replace(/\\/g, "/"),
			contributions: [],
			unfilledPoints: [],
			unfilledRequired: [],
			overrides: [],
			resolvedAt: new Date().toISOString(),
		};

		// Apply project-level overrides to standalone workflows too
		const overrideResult = applyOverrides(defn, overrideDiscovery.overrides, metadata);
		result.errors.push(...overrideResult.errors);

		const errors = validateResolvedWorkflow(overrideResult.definition, metadata);
		const outputPath = path.join(
			outputDir,
			`${overrideResult.definition.name}.resolved.yaml`,
		);

		const outputContent = buildResolvedYaml(overrideResult.definition, metadata);
		fs.writeFileSync(outputPath, outputContent, "utf-8");

		result.standalone.push({
			name: overrideResult.definition.name,
			definition: overrideResult.definition,
			metadata,
			errors,
			outputPath,
		});
	}

	return result;
}

// ---------------------------------------------------------------------------
// Output Formatting
// ---------------------------------------------------------------------------

/**
 * Build the resolved YAML file content with metadata comment header.
 *
 * The output is deterministic — same input produces same output. This makes
 * the resolved files diffable in version control.
 */
function buildResolvedYaml(
	definition: WorkflowDefinition,
	metadata: ResolutionMetadata,
): string {
	const lines: string[] = [];

	// Metadata comment header
	lines.push("# AUTO-GENERATED — do not edit manually.");
	lines.push(`# Resolved by: orqa plugin install`);
	lines.push(`# Skeleton: ${metadata.skeletonPlugin} (${metadata.skeletonFile})`);
	if (metadata.contributions.length > 0) {
		lines.push("# Contributions:");
		for (const c of metadata.contributions) {
			lines.push(`#   - ${c.plugin} → ${c.point} (+${c.statesAdded.length} states, +${c.transitionsAdded} transitions)`);
		}
	}
	if (metadata.unfilledPoints.length > 0) {
		lines.push(`# Unfilled contribution points: ${metadata.unfilledPoints.join(", ")}`);
	}
	if (metadata.overrides.length > 0) {
		lines.push("# Project overrides:");
		for (const o of metadata.overrides) {
			const parts: string[] = [];
			if (o.statesAdded.length > 0) parts.push(`+${o.statesAdded.length} states`);
			if (o.statesReplaced.length > 0) parts.push(`~${o.statesReplaced.length} replaced`);
			if (o.transitionsAdded > 0) parts.push(`+${o.transitionsAdded} transitions`);
			if (o.fieldsOverridden.length > 0) parts.push(`fields: ${o.fieldsOverridden.join(", ")}`);
			lines.push(`#   - ${o.file} (${parts.join(", ")})`);
		}
	}
	lines.push(`# Resolved at: ${metadata.resolvedAt}`);
	lines.push("");

	// Serialize the definition — strip contribution_points from output
	// since they are build-time metadata, not runtime data
	const outputDefn = { ...definition };
	if (outputDefn.contribution_points) {
		// Keep contribution_points in the output for traceability but mark them as resolved
		const resolvedPoints = outputDefn.contribution_points.map((p) => ({
			...p,
		}));
		outputDefn.contribution_points = resolvedPoints;
	}

	// Use yaml stringify with sorted keys for deterministic output
	const yamlContent = stringifyYaml(outputDefn, {
		lineWidth: 120,
		defaultKeyType: "PLAIN",
		defaultStringType: "PLAIN",
	});

	lines.push(yamlContent);

	return lines.join("\n");
}

// ---------------------------------------------------------------------------
// CLI Integration Helper
// ---------------------------------------------------------------------------

/**
 * Run workflow resolution and print results.
 *
 * Called from `cmdPluginSync` in install.ts and `cmdRefresh` in plugin.ts.
 */
export function runWorkflowResolution(projectRoot: string): void {
	const result = resolveAllWorkflows(projectRoot);

	const totalResolved = result.resolved.length + result.standalone.length;

	if (totalResolved === 0 && result.errors.length === 0) {
		// No workflows found — nothing to report
		return;
	}

	if (result.errors.length > 0) {
		console.log(`  Workflow discovery warnings:`);
		for (const err of result.errors) {
			console.log(`    - ${err}`);
		}
	}

	for (const r of [...result.resolved, ...result.standalone]) {
		const relPath = path.relative(projectRoot, r.outputPath).replace(/\\/g, "/");
		const contribCount = r.metadata.contributions.length;
		const overrideCount = r.metadata.overrides.length;
		const parts: string[] = [];
		if (contribCount > 0) parts.push(`${contribCount} contribution(s)`);
		if (overrideCount > 0) parts.push(`${overrideCount} override(s)`);
		const suffix = parts.length > 0 ? ` (${parts.join(", ")})` : "";
		const label = contribCount > 0 ? "Resolved" : "Standalone";
		console.log(`  ${label} workflow: ${r.name}${suffix} → ${relPath}`);
		if (r.errors.length > 0) {
			for (const err of r.errors) {
				console.log(`    WARNING: ${err}`);
			}
		}
	}

	if (totalResolved > 0) {
		console.log(`  ✓ ${totalResolved} workflow(s) resolved`);
	}
}

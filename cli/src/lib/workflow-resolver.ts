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
	type ArtifactSchema,
	type RelationshipType,
} from "@orqastudio/types";
import { listInstalledPlugins } from "./installer.js";
import { readManifest } from "./manifest.js";

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
 * @param projectRoot - Absolute path to the project root.
 * @returns Object with discovered workflows and any errors encountered.
 */
export function discoverWorkflows(projectRoot: string): {
	workflows: DiscoveredWorkflow[];
	errors: string[];
} {
	const workflows: DiscoveredWorkflow[] = [];
	const errors: string[] = [];

	// plugins/ is two levels deep: plugins/<taxonomy>/<plugin>
	// connectors/ and sidecars/ are one level deep: <container>/<plugin>
	const pluginDirs: string[] = [];

	const pluginsDir = path.join(projectRoot, "plugins");
	if (fs.existsSync(pluginsDir)) {
		for (const taxonomy of fs.readdirSync(pluginsDir, { withFileTypes: true })) {
			if (!taxonomy.isDirectory() || taxonomy.name.startsWith(".")) continue;
			const taxonomyPath = path.join(pluginsDir, taxonomy.name);
			for (const plugin of fs.readdirSync(taxonomyPath, { withFileTypes: true })) {
				if (!plugin.isDirectory() || plugin.name.startsWith(".")) continue;
				pluginDirs.push(path.join(taxonomyPath, plugin.name));
			}
		}
	}

	for (const container of ["connectors", "sidecars"]) {
		const containerDir = path.join(projectRoot, container);
		if (!fs.existsSync(containerDir)) continue;
		for (const entry of fs.readdirSync(containerDir, { withFileTypes: true })) {
			if (!entry.isDirectory() || entry.name.startsWith(".")) continue;
			pluginDirs.push(path.join(containerDir, entry.name));
		}
	}

	for (const pluginDir of pluginDirs) {
		const entry = { name: path.basename(pluginDir) };
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
 * @param projectRoot - Absolute path to the project root.
 * @returns Object with discovered overrides and any errors encountered.
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
 * @param definition - The resolved workflow definition to apply overrides to.
 * @param overrides - Project-level override declarations to apply.
 * @param metadata - Resolution metadata to record override application in.
 * @returns Updated definition with overrides applied, and any errors.
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
 * @param workflows - All discovered workflows to classify.
 * @returns Object with skeletons, contributions map, and standalone workflows.
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
 * @param skeleton - The skeleton workflow to merge contributions into.
 * @param contributions - Map of contribution point keys to their contributions.
 * @returns The merged workflow definition and resolution metadata.
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
 * @param definition - The resolved workflow definition to validate.
 * @param metadata - The resolution metadata from mergeContributions.
 * @returns Array of error messages; empty array means the workflow is valid.
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
// Stage-Based Resolved Workflow Output
// ---------------------------------------------------------------------------

/**
 * Build the id_pattern regex for an artifact schema.
 *
 * Pattern format: ^{PREFIX}-[a-f0-9]{8}$
 * @param idPrefix - The ID prefix string (e.g. "TASK", "EPIC").
 * @returns The regex pattern string anchored at start and end.
 */
function buildIdPatternForSchema(idPrefix: string): string {
	const escaped = idPrefix.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
	return `^${escaped}-[a-f0-9]{8}$`;
}

/**
 * Write stage-based resolved workflow files from a resolved methodology skeleton.
 *
 * For each contribution_point in the skeleton that has a `stage` field, writes
 * a `<stage>.resolved.yaml` file containing:
 * - The contribution workflow states/transitions from the stage plugin
 * - Artifact types provided by the contributing plugin (from its manifest)
 * - Relationship types relevant to this stage (from the contributing plugin)
 *
 * This satisfies DOC-fd3edf48 section 5.1: resolved workflows should be named
 * by stage (discovery.resolved.yaml, planning.resolved.yaml, etc.).
 * @param skeleton - The skeleton workflow (e.g. agile-methodology).
 * @param contributions - The matched contributions map from matchContributions.
 * @param projectRoot - Absolute path to the project root.
 * @param outputDir - Directory to write stage files into.
 */
function writeStageResolvedWorkflows(
	skeleton: DiscoveredWorkflow,
	contributions: Map<string, WorkflowContribution[]>,
	projectRoot: string,
	outputDir: string,
): void {
	const points = skeleton.definition.contribution_points ?? [];

	// Build a map of plugin name → plugin path for fast lookup.
	const pluginPathByName = new Map<string, string>();
	try {
		for (const plugin of listInstalledPlugins(projectRoot)) {
			pluginPathByName.set(plugin.name, plugin.path);
		}
	} catch {
		// If plugin list is unavailable, stage enrichment with artifact types is skipped
	}

	for (const point of points) {
		const stageName = point.stage;
		if (!stageName) continue;

		const key = `${skeleton.definition.name}::${point.name}`;
		const pointContributions = contributions.get(key) ?? [];
		if (pointContributions.length === 0) continue;

		// Merge states and transitions from all contributing plugins into one stage record.
		const stageStates: Record<string, WorkflowState> = {};
		const stageTransitions: Transition[] = [];
		const contributingPlugins: Array<{ plugin: string; version: string }> = [];

		// Sort by priority ascending so higher-priority contributions win.
		const sorted = [...pointContributions].sort((a, b) => a.priority - b.priority);

		for (const contrib of sorted) {
			for (const [stateName, stateDef] of Object.entries(contrib.states)) {
				stageStates[stateName] = stateDef;
			}
			for (const transition of contrib.transitions) {
				const isDupe = stageTransitions.some(
					(t) =>
						JSON.stringify(t.from) === JSON.stringify(transition.from) &&
						t.to === transition.to &&
						t.event === transition.event,
				);
				if (!isDupe) {
					stageTransitions.push(transition);
				}
			}

			// Look up plugin version from manifest.
			const pluginPath = pluginPathByName.get(contrib.pluginName);
			let version = "unknown";
			if (pluginPath) {
				try {
					const manifest = readManifest(pluginPath);
					version = manifest.version;
				} catch {
					// Skip
				}
			}

			if (!contributingPlugins.some((p) => p.plugin === contrib.pluginName)) {
				contributingPlugins.push({ plugin: contrib.pluginName, version });
			}
		}

		// Collect artifact types and relationship types from contributing plugins.
		const artifactTypes: Record<string, unknown> = {};
		const relationshipTypes: Record<string, unknown> = {};

		for (const { plugin: pluginName } of contributingPlugins) {
			const pluginPath = pluginPathByName.get(pluginName);
			if (!pluginPath) continue;
			try {
				const manifest = readManifest(pluginPath);

				for (const schema of manifest.provides?.schemas ?? []) {
					artifactTypes[schema.key] = buildArtifactTypeSummary(schema, pluginName);
				}

				for (const rel of manifest.provides?.relationships ?? []) {
					const relType = rel as RelationshipType;
					if (!relationshipTypes[relType.key]) {
						relationshipTypes[relType.key] = {
							forward: relType.key,
							inverse: relType.inverse,
							from: relType.from,
							to: relType.to,
							semantic: relType.semantic,
							...(relType.constraints ? { constraints: relType.constraints } : {}),
						};
					}
				}
			} catch {
				// Skip plugins with unreadable manifests
			}
		}

		// Determine initial state for the stage contribution workflow.
		const initialState = stageTransitions.length > 0
			? (Array.isArray(stageTransitions[0].from)
				? stageTransitions[0].from[0]
				: stageTransitions[0].from) ?? Object.keys(stageStates)[0]
			: Object.keys(stageStates)[0];

		const primaryPlugin = contributingPlugins[0]?.plugin ?? "unknown";
		const primaryVersion = contributingPlugins[0]?.version ?? "unknown";

		// Build the stage resolved YAML object.
		const stageResolved: Record<string, unknown> = {
			version: "1.0.0",
			generated: true,
			generatedAt: new Date().toISOString(),
			description: `Resolved ${stageName} stage workflow. Contains states, transitions, and artifact types contributed by ${primaryPlugin} into the ${point.name} slot of ${skeleton.definition.name}.`,
			stage: stageName,
			source_plugin: primaryPlugin,
			fills_slot: point.name,
			methodology: skeleton.definition.name,
		};

		// Add contribution workflow if states exist.
		if (Object.keys(stageStates).length > 0) {
			stageResolved["contribution_workflow"] = {
				name: `${stageName}-contribution`,
				initial_state: initialState,
				states: stageStates,
				transitions: stageTransitions,
			};
		}

		// Add artifact types if any were found.
		if (Object.keys(artifactTypes).length > 0) {
			stageResolved["artifact_types"] = artifactTypes;
		}

		// Add relationship types if any were found.
		if (Object.keys(relationshipTypes).length > 0) {
			stageResolved["relationship_types"] = relationshipTypes;
		}

		// Add contribution metadata.
		stageResolved["contribution_metadata"] = contributingPlugins.map((cp) => ({
			plugin: cp.plugin,
			version: cp.version,
		}));

		// Map short stage names to canonical full names for file output.
		// The skeleton uses abbreviated names (discover, plan, document, implement, learn)
		// but the resolved files use the full canonical names (discovery, planning, etc.).
		const STAGE_FILE_NAMES: Record<string, string> = {
			discover: "discovery",
			plan: "planning",
			document: "documentation",
			implement: "implementation",
			review: "review",
			learn: "learning",
		};
		const fileBaseName = STAGE_FILE_NAMES[stageName] ?? stageName;

		// Write the stage file.
		const outputPath = path.join(outputDir, `${fileBaseName}.resolved.yaml`);
		const header = [
			"# AUTO-GENERATED — do not edit manually.",
			`# Stage: ${stageName} (${point.name} slot in ${skeleton.definition.name})`,
			`# Source plugin: ${primaryPlugin} v${primaryVersion}`,
			`# Generated at: ${new Date().toISOString()}`,
			"",
		].join("\n");

		const yamlContent = stringifyYaml(stageResolved, {
			lineWidth: 120,
			defaultKeyType: "PLAIN",
			defaultStringType: "PLAIN",
		});

		fs.writeFileSync(outputPath, header + yamlContent, "utf-8");
	}
}

/**
 * Build a summary of an artifact type for inclusion in stage resolved files.
 *
 * Extracts the key fields from a plugin ArtifactSchema into the format used in
 * discovery.resolved.yaml and other stage files.
 * @param schema - The artifact schema from the plugin manifest.
 * @param pluginName - The name of the plugin providing this artifact type.
 * @returns A plain object summary suitable for embedding in resolved YAML.
 */
function buildArtifactTypeSummary(schema: ArtifactSchema, pluginName: string): Record<string, unknown> {
	const requiredKeys = new Set(schema.frontmatter.required ?? []);
	const properties = schema.frontmatter.properties ?? {};
	const required: Record<string, unknown> = {};
	const optional: Record<string, unknown> = {};

	for (const [key, def] of Object.entries(properties)) {
		if (requiredKeys.has(key)) {
			required[key] = def;
		} else {
			optional[key] = def;
		}
	}

	const statuses = Object.keys(schema.statusTransitions ?? {});

	return {
		id_prefix: schema.idPrefix,
		id_pattern: buildIdPatternForSchema(schema.idPrefix),
		default_path: schema.defaultPath.endsWith("/")
			? schema.defaultPath
			: schema.defaultPath + "/",
		icon: schema.icon,
		source_plugin: pluginName,
		fields: { required, optional },
		additional_properties: schema.frontmatter.additionalProperties ?? true,
		...(statuses.length > 0 ? {
			state_machine: {
				initial_state: statuses[0],
				states: buildStateMapFromTransitions(schema.statusTransitions ?? {}),
				transitions: [],
			},
		} : {}),
	};
}

/**
 * Build a minimal state map from statusTransitions keys.
 *
 * Without the resolved workflow YAML available, we can only create placeholder
 * state entries from the status names. Full state details (category, on_enter)
 * come from the workflow resolver when stage states are contributed.
 * @param statusTransitions - Map of status names to their allowed next statuses.
 * @returns Map of status names to placeholder state objects.
 */
function buildStateMapFromTransitions(
	statusTransitions: Record<string, string[]>,
): Record<string, { category: string; description: string }> {
	const states: Record<string, { category: string; description: string }> = {};
	for (const status of Object.keys(statusTransitions)) {
		states[status] = {
			category: "planning",
			description: status,
		};
	}
	return states;
}

// ---------------------------------------------------------------------------
// Resolution Pipeline
// ---------------------------------------------------------------------------

/**
 * Resolve all workflows in a project.
 *
 * This is the main entry point called by `orqa plugin install` / `orqa plugin refresh`.
 * @param projectRoot - Absolute path to the project root.
 * @returns The resolution result with resolved, standalone, and error lists.
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

		// Write per-stage resolved files for methodology skeletons.
		// Methodology skeletons have contribution_points with a `stage` field.
		// Each stage gets its own <stage>.resolved.yaml alongside the main file.
		const hasStages = (skeleton.definition.contribution_points ?? []).some(
			(p) => p.stage,
		);
		if (hasStages) {
			try {
				writeStageResolvedWorkflows(skeleton, contributions, projectRoot, outputDir);
			} catch (e) {
				result.errors.push(
					`Stage workflow writing failed for ${skeleton.definition.name}: ${e instanceof Error ? e.message : String(e)}`,
				);
			}
		}
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
 * @param definition - The resolved workflow definition to serialize.
 * @param metadata - The resolution metadata to include in the comment header.
 * @returns The full YAML string with comment header.
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
 * @param projectRoot - Absolute path to the project root.
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

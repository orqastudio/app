/**
 * Workflow resolver — merges plugin workflow contributions into resolved workflows.
 *
 * Runs at `orqa plugin install` / `orqa plugin refresh` time:
 * 1. Scans installed plugins for `workflows/` directories
 * 2. Identifies skeletons (files with `contribution_points`) and standalone workflows
 * 3. Merges stage-plugin contributions into skeleton contribution points
 * 4. Validates the merged result against the workflow schema structure
 * 5. Writes resolved workflows to `.orqa/workflows/<name>.resolved.json`
 *
 * The runtime engine reads only resolved files — never raw plugin workflows.
 */
import { type WorkflowDefinition, type WorkflowState, type Transition } from "@orqastudio/types";
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
/**
 * Discover all workflow files across installed plugins.
 *
 * Scans `workflows/` directories in plugins/, connectors/, and sidecars/.
 * Files must end with `.workflow.yaml` or `.workflow.yml`.
 * @param projectRoot - Absolute path to the project root.
 * @returns Object with discovered workflows and any errors encountered.
 */
export declare function discoverWorkflows(projectRoot: string): {
    workflows: DiscoveredWorkflow[];
    errors: string[];
};
/**
 * Discover project-level workflow override files.
 *
 * Scans `.orqa/workflows/overrides/` for `*.override.yaml` / `*.override.yml` files.
 * Each file must contain a `target_workflow` field identifying which resolved
 * workflow to override.
 * @param projectRoot - Absolute path to the project root.
 * @returns Object with discovered overrides and any errors encountered.
 */
export declare function discoverOverrides(projectRoot: string): {
    overrides: DiscoveredOverride[];
    errors: string[];
};
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
export declare function applyOverrides(definition: WorkflowDefinition, overrides: DiscoveredOverride[], metadata: ResolutionMetadata): {
    definition: WorkflowDefinition;
    errors: string[];
};
/**
 * Match non-skeleton workflows to skeleton contribution points.
 *
 * A workflow file is a contribution only if it explicitly declares
 * `contributes_to` targeting a skeleton workflow and contribution point.
 * All other non-skeleton workflows are standalone.
 * @param workflows - All discovered workflows to classify.
 * @returns Object with skeletons, contributions map, and standalone workflows.
 */
export declare function matchContributions(workflows: DiscoveredWorkflow[]): {
    skeletons: DiscoveredWorkflow[];
    contributions: Map<string, WorkflowContribution[]>;
    standalone: DiscoveredWorkflow[];
};
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
export declare function mergeContributions(skeleton: DiscoveredWorkflow, contributions: Map<string, WorkflowContribution[]>): {
    merged: WorkflowDefinition;
    metadata: ResolutionMetadata;
};
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
export declare function validateResolvedWorkflow(definition: WorkflowDefinition, metadata: ResolutionMetadata): string[];
/**
 * Resolve all workflows in a project.
 *
 * This is the main entry point called by `orqa plugin install` / `orqa plugin refresh`.
 * @param projectRoot - Absolute path to the project root.
 * @returns The resolution result with resolved, standalone, and error lists.
 */
export declare function resolveAllWorkflows(projectRoot: string): ResolveAllResult;
/**
 * Run workflow resolution and print results.
 *
 * Called from `cmdPluginSync` in install.ts and `cmdRefresh` in plugin.ts.
 * @param projectRoot - Absolute path to the project root.
 */
export declare function runWorkflowResolution(projectRoot: string): void;
export {};
//# sourceMappingURL=workflow-resolver.d.ts.map
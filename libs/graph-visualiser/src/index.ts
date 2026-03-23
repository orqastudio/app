// Visualiser (reactive Svelte 5 class)
export { GraphVisualiser } from "./visualiser.svelte.js";

// Pure analysis functions (no Svelte dependency)
export { computeGraphHealth, computeBackboneArtifacts, computeKnowledgeGaps, traceChain, computeImpact } from "./analysis.js";

// Element builders
export { buildAnalysisElements, buildVisualizationElements } from "./elements.js";

// Colors
export { ARTIFACT_TYPE_COLORS } from "./colors.js";

// Types
export type { NodePosition, GraphHealth, BackboneArtifact, KnowledgeGaps, ImpactResult, GraphDataSource } from "./types.js";

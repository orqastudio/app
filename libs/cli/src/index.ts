/**
 * @orqastudio/cli — library exports for programmatic use.
 *
 * Used by connectors, plugins, and other consumers that need
 * plugin management, validation, graph browsing, or version management
 * without spawning a subprocess.
 */

// Symlink utilities
export {
	createSymlink,
	ensureSymlink,
	verifySymlink,
	removeSymlink,
	type SymlinkOptions,
	type SymlinkResult,
	type SymlinkVerification,
} from "./lib/symlink.js";

// Plugin management
export { installPlugin, uninstallPlugin, listInstalledPlugins } from "./lib/installer.js";
export { fetchRegistry } from "./lib/registry.js";
export { readLockfile, writeLockfile } from "./lib/lockfile.js";
export { readManifest, validateManifest } from "./lib/manifest.js";

// Graph browsing (daemon-backed)
export { scanArtifactGraph, queryGraph, getGraphStats, type GraphNode, type GraphQueryOptions, type GraphStats } from "./lib/graph.js";

// Daemon client
export { callDaemonGraph, isDaemonRunning, type DaemonArtifactNode, type DaemonArtifactRef, type DaemonHealthResponse } from "./lib/daemon-client.js";

// Version management
export {
	readCanonicalVersion,
	writeCanonicalVersion,
	syncVersions,
	checkVersionDrift,
} from "./lib/version-sync.js";

// Repo maintenance
export { auditLicenses, DEFAULT_LICENSE_POLICY, type LicenseAuditResult, type LicensePolicy } from "./lib/license.js";
export { auditReadmes, generateReadmeTemplate, type ReadmeAuditResult } from "./lib/readme.js";

// Prompt pipeline
export {
	generatePrompt,
	estimateTokens,
	DEFAULT_TOKEN_BUDGETS,
	type PromptPipelineOptions,
	type PromptResult,
	type ResolvedSection,
} from "./lib/prompt-pipeline.js";

// Knowledge retrieval
export {
	retrieveKnowledge,
	queryOnDemandEntries,
	countOnDemandEntries,
	generateOnDemandPreamble,
	type KnowledgeQueryOptions,
	type RetrievedKnowledge,
} from "./lib/knowledge-retrieval.js";

// Token tracking
export {
	TokenTracker,
	recordRequest,
	recordAgentComplete,
	recordSessionSummary,
	readMetricEvents,
	filterEvents,
	computeTrends,
	getMetricsPath,
	type RequestMetrics,
	type AgentMetrics,
	type SessionMetrics,
	type MetricEvent,
	type TrendMetrics,
} from "./lib/token-tracker.js";

// Budget enforcement
export {
	BudgetEnforcer,
	estimateCost,
	inferModelTier,
	suggestDowngrade,
	DEFAULT_BUDGETS,
	COST_PER_MTOK,
	MODEL_TIERS,
	type BudgetConfig,
	type BudgetCheckResult,
	type BudgetSeverity,
} from "./lib/budget-enforcer.js";

// Prompt registry
export {
	buildPromptRegistry,
	readPromptRegistry,
	runPromptRegistryBuild,
	queryKnowledge,
	querySections,
	type PromptRegistry,
	type RegistryKnowledgeEntry,
	type RegistryPromptSection,
} from "./lib/prompt-registry.js";

// Gate engine
export {
	startGate,
	submitVerdict,
	getOpenGates,
	getGateSession,
	clearGateSessions,
	setAiRecommendation,
	computeCycleTime,
	type GatePhase,
	type GateSession,
	type GateVerdictInput,
	type GateResult,
	type GateEngineOptions,
	type LessonAction,
	type CycleTimeData,
	type PreCheckResult,
	type ReviewerVerdict,
} from "./lib/gate-engine.js";

// Agent spawner
export {
	createAgentConfig,
	selectModelTier,
	isValidRole,
	modelTierLabel,
	serializeFindings,
	parseFindingsHeader,
	UNIVERSAL_ROLES,
	DEFAULT_MODEL_TIERS,
	ROLE_TOOL_CONSTRAINTS,
	type UniversalRole,
	type ModelTier,
	type TaskComplexity,
	type ToolConstraint,
	type TaskContext,
	type AgentSpawnConfig,
	type CreateAgentParams,
	type FindingsHeader,
	type FindingsDocument,
} from "./lib/agent-spawner.js";

/**
 * \@orqastudio/cli — library exports for programmatic use.
 *
 * Used by connectors, plugins, and other consumers that need
 * plugin management, validation, graph browsing, or version management
 * without spawning a subprocess.
 */
export { createSymlink, ensureSymlink, verifySymlink, removeSymlink, type SymlinkOptions, type SymlinkResult, type SymlinkVerification, } from "./lib/symlink.js";
export { installPlugin, uninstallPlugin, listInstalledPlugins } from "./lib/installer.js";
export { fetchRegistry } from "./lib/registry.js";
export { readLockfile, writeLockfile } from "./lib/lockfile.js";
export { readManifest, validateManifest } from "./lib/manifest.js";
export { scanArtifactGraph, queryGraph, getGraphStats, type GraphNode, type GraphQueryOptions, type GraphStats } from "./lib/graph.js";
export { callDaemonGraph, isDaemonRunning, type DaemonArtifactNode, type DaemonArtifactRef, type DaemonHealthResponse } from "./lib/daemon-client.js";
export { readCanonicalVersion, writeCanonicalVersion, syncVersions, checkVersionDrift, } from "./lib/version-sync.js";
export { auditLicenses, DEFAULT_LICENSE_POLICY, type LicenseAuditResult, type LicensePolicy } from "./lib/license.js";
export { auditReadmes, generateReadmeTemplate, type ReadmeAuditResult } from "./lib/readme.js";
export { TokenTracker, recordRequest, recordAgentComplete, recordSessionSummary, readMetricEvents, filterEvents, computeTrends, getMetricsPath, type RequestMetrics, type AgentMetrics, type SessionMetrics, type MetricEvent, type TrendMetrics, } from "./lib/token-tracker.js";
export { BudgetEnforcer, estimateCost, inferModelTier, suggestDowngrade, DEFAULT_BUDGETS, COST_PER_MTOK, MODEL_TIERS, type BudgetConfig, type BudgetCheckResult, type BudgetSeverity, } from "./lib/budget-enforcer.js";
export { createAgentConfig, selectModelTier, isValidRole, modelTierLabel, serializeFindings, parseFindingsHeader, UNIVERSAL_ROLES, DEFAULT_MODEL_TIERS, DEFAULT_TOKEN_BUDGETS, ROLE_TOOL_CONSTRAINTS, type UniversalRole, type ModelTier, type TaskComplexity, type ToolConstraint, type TaskContext, type AgentSpawnConfig, type CreateAgentParams, type FindingsHeader, type FindingsDocument, } from "./lib/agent-spawner.js";
//# sourceMappingURL=index.d.ts.map
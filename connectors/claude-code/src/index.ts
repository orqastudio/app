/**
 * @orqastudio/claude-code-cli — Claude Code governance integration library.
 *
 * This package is BOTH:
 * 1. A Claude Code plugin (has .claude-plugin/plugin.json + hooks + skills + commands)
 * 2. A reusable library that other plugins can depend on
 *
 * Governance process is applied via connector-generated artifacts (hooks, skills,
 * commands) that map directly to the OrqaStudio daemon and CLI.
 */

// Re-export graph browsing from @orqastudio/cli (now daemon-backed)
export { scanArtifactGraph, queryGraph, getGraphStats } from "@orqastudio/cli";
export type { GraphNode, GraphQueryOptions, GraphStats } from "@orqastudio/cli";

// Re-export daemon client for direct daemon access
export { callDaemonGraph, isDaemonRunning } from "@orqastudio/cli";
export type { DaemonArtifactNode, DaemonArtifactRef, DaemonHealthResponse } from "@orqastudio/cli";

// Plugin management re-exports
export { installPlugin, uninstallPlugin, listInstalledPlugins } from "@orqastudio/cli";

// Local modules
export { runConnectorSetup, type ConnectorSetupResult } from "./connector-setup.js";

// Connector generation pipeline
export { generatePlugin, type GenerateResult } from "./generator.js";
export { watchAndRegenerate } from "./watcher.js";

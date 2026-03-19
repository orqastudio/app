/**
 * @orqastudio/claude-code-cli — Claude Code governance integration library.
 *
 * This package is BOTH:
 * 1. A Claude Code plugin (has .claude-plugin/plugin.json + hooks + skills + commands)
 * 2. A reusable library that other plugins can depend on
 *
 * It bridges the `.claude/` directory with OrqaStudio's artifact system,
 * so the same governance process applies via CLI as through the app.
 */
// Re-export graph browsing from @orqastudio/cli
export { scanArtifactGraph, queryGraph, getGraphStats } from "@orqastudio/cli";
// Plugin management re-exports
export { installPlugin, uninstallPlugin, listInstalledPlugins } from "@orqastudio/cli";
// Local modules
export { RuleEngine } from "./rule-engine.js";
export { PromptInjector } from "./prompt-injector.js";
export { ArtifactBridge } from "./artifact-bridge.js";
//# sourceMappingURL=index.js.map
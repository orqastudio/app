/**
 * Connector Setup — Claude Code-specific post-install setup.
 *
 * This module owns all Claude Code-specific directory wiring:
 * - .claude/agents/ — merged directory of core + plugin agent symlinks
 * - .claude/rules   — symlink to .orqa/process/rules/
 * - .lsp.json       — aggregated LSP server configurations
 * - .mcp.json       — aggregated MCP server configurations
 *
 * It is called by the Claude Code connector after plugin installation,
 * and can also be run standalone to repair a broken install.
 *
 * The CLI installer (installer.ts) has no knowledge of .claude/ — this
 * module is the single source of truth for that directory's structure.
 */
export interface ConnectorSetupResult {
    symlinkAgents: "created" | "skipped" | "exists" | "replaced";
    symlinkRules: "created" | "skipped" | "exists" | "replaced";
    pluginAgentCount: number;
    lspCount: number;
    mcpCount: number;
}
/**
 * Run post-install setup for the Claude Code connector:
 * 1. Build .claude/agents/ as a merged directory containing symlinks to:
 *    - All core agents from app/.orqa/process/agents/ (or .orqa/process/agents/)
 *    - All plugin agents declared via provides.agents in installed plugin manifests
 *    Plugin agents are keyed by their manifest `key` field (e.g. "rust-specialist").
 *    Core agents take precedence: a plugin cannot shadow a core agent filename.
 * 2. Create .claude/rules → .orqa/process/rules/ symlink
 * 3. Aggregate lspServers/mcpServers from all plugins/connectors → .lsp.json/.mcp.json
 *    written into the connector's plugin directory.
 *
 * Called automatically by installPlugin when the installed plugin is the Claude Code connector.
 * Can also be called standalone to repair a broken install.
 *
 * NOTE: .claude/CLAUDE.md is NOT managed here — it is a Claude Code project artifact
 * maintained directly, not derived from any source file.
 */
export declare function runConnectorSetup(projectRoot: string, connectorPluginDir: string): ConnectorSetupResult;
//# sourceMappingURL=connector-setup.d.ts.map
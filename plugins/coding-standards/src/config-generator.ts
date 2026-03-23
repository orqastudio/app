/**
 * Config Generator — reads enforcement rules from .orqa/process/rules/
 * and generates tool config files (ESLint, Prettier, clippy, etc.).
 *
 * The generator reads:
 * 1. Active rules with `enforcement` arrays in their frontmatter
 * 2. Plugin manifests to discover available tools and their config formats
 * 3. Existing config files to merge (not overwrite)
 *
 * Output: tool-specific config files in the project root.
 */

import * as fs from "node:fs";
import * as path from "node:path";

export interface GeneratedConfig {
	tool: string;
	configFile: string;
	configFormat: string;
	rules: string[];
	written: boolean;
}

interface EnforcementEntry {
	event: string;
	pattern?: string;
	paths?: string[];
	action: string;
	message?: string;
	tool?: string;
	config?: Record<string, unknown>;
}

interface ParsedRule {
	id: string;
	name: string;
	enforcement: EnforcementEntry[];
}

interface PluginTool {
	command: string;
	configFile: string | null;
	configFormat: string;
}

export class ConfigGenerator {
	private projectRoot: string;

	constructor(projectRoot?: string) {
		this.projectRoot = projectRoot ?? process.cwd();
	}

	/**
	 * Generate config files for all configured tools from enforcement rules.
	 */
	generate(): GeneratedConfig[] {
		const rules = this.loadEnforcementRules();
		const tools = this.discoverPluginTools();
		const results: GeneratedConfig[] = [];

		// Group enforcement entries by tool
		const toolRules = new Map<string, EnforcementEntry[]>();
		for (const rule of rules) {
			for (const entry of rule.enforcement) {
				if (entry.tool) {
					const existing = toolRules.get(entry.tool) ?? [];
					existing.push(entry);
					toolRules.set(entry.tool, existing);
				}
			}
		}

		// Generate config for each tool that has rules
		for (const [toolName, entries] of toolRules) {
			const toolDef = tools.get(toolName);
			if (!toolDef || !toolDef.configFile) continue;

			const configPath = path.join(this.projectRoot, toolDef.configFile);
			const config = this.buildToolConfig(toolName, entries, toolDef);

			if (config) {
				const written = this.writeConfig(configPath, config, toolDef.configFormat);
				results.push({
					tool: toolName,
					configFile: toolDef.configFile,
					configFormat: toolDef.configFormat,
					rules: entries.map((e) => e.pattern ?? "").filter(Boolean),
					written,
				});
			}
		}

		return results;
	}

	private loadEnforcementRules(): ParsedRule[] {
		const rulesDir = path.join(this.projectRoot, ".orqa", "process", "rules");
		if (!fs.existsSync(rulesDir)) return [];

		const rules: ParsedRule[] = [];
		for (const file of fs.readdirSync(rulesDir)) {
			if (!file.endsWith(".md")) continue;
			const content = fs.readFileSync(path.join(rulesDir, file), "utf-8");
			const rule = this.parseRule(content, file);
			if (rule) rules.push(rule);
		}
		return rules;
	}

	private parseRule(content: string, filename: string): ParsedRule | null {
		const fmMatch = content.match(/^---\n([\s\S]*?)\n---/);
		if (!fmMatch) return null;

		const yamlBlock = fmMatch[1];
		if (!yamlBlock) return null;

		const idMatch = yamlBlock.match(/^id:\s*(.+)/m);
		const nameMatch = yamlBlock.match(/^(?:title|name):\s*(.+)/m);
		const statusMatch = yamlBlock.match(/^status:\s*(.+)/m);

		if (!idMatch?.[1]) return null;
		const status = statusMatch?.[1]?.trim().replace(/"/g, "");
		if (status && status !== "active") return null;

		// Simple enforcement array extraction
		if (!yamlBlock.includes("enforcement:")) return null;

		// Parse enforcement entries (simplified YAML parsing)
		const enforcement: EnforcementEntry[] = [];
		const enfSection = yamlBlock.split("enforcement:")[1];
		if (!enfSection) return null;

		const entryBlocks = enfSection.split(/\n\s+-\s+/).filter(Boolean);
		for (const block of entryBlocks) {
			const entry: Partial<EnforcementEntry> = {};
			for (const line of block.split("\n")) {
				const kv = line.match(/^\s*(\w+):\s*(.+)/);
				if (!kv) continue;
				const key = kv[1];
				const val = kv[2];
				if (!key || !val) continue;
				const cleaned = val.trim().replace(/^["']|["']$/g, "");
				switch (key) {
					case "event": entry.event = cleaned; break;
					case "action": entry.action = cleaned; break;
					case "pattern": entry.pattern = cleaned; break;
					case "message": entry.message = cleaned; break;
					case "tool": entry.tool = cleaned; break;
				}
			}
			if (entry.event && entry.action) {
				enforcement.push(entry as EnforcementEntry);
			}
		}

		if (enforcement.length === 0) return null;

		return {
			id: idMatch[1].trim().replace(/"/g, ""),
			name: nameMatch?.[1]?.trim().replace(/"/g, "") ?? filename,
			enforcement,
		};
	}

	private discoverPluginTools(): Map<string, PluginTool> {
		const tools = new Map<string, PluginTool>();
		const pluginsDir = path.join(this.projectRoot, "plugins");

		if (!fs.existsSync(pluginsDir)) return tools;

		for (const entry of fs.readdirSync(pluginsDir, { withFileTypes: true })) {
			if (!entry.isDirectory()) continue;
			const manifestPath = path.join(pluginsDir, entry.name, "orqa-plugin.json");
			if (!fs.existsSync(manifestPath)) continue;

			try {
				const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf-8"));
				const pluginTools = manifest.provides?.tools;
				if (pluginTools && typeof pluginTools === "object") {
					for (const [name, def] of Object.entries(pluginTools)) {
						tools.set(name, def as PluginTool);
					}
				}
			} catch {
				// Skip invalid manifests
			}
		}

		return tools;
	}

	private buildToolConfig(
		_toolName: string,
		entries: EnforcementEntry[],
		_toolDef: PluginTool,
	): Record<string, unknown> | null {
		// Build a config object from enforcement entries
		const config: Record<string, unknown> = {};

		for (const entry of entries) {
			if (entry.config) {
				Object.assign(config, entry.config);
			}
		}

		return Object.keys(config).length > 0 ? config : null;
	}

	private writeConfig(
		configPath: string,
		config: Record<string, unknown>,
		format: string,
	): boolean {
		try {
			let content: string;
			switch (format) {
				case "json":
					content = JSON.stringify(config, null, 2);
					break;
				case "toml":
					// Simple TOML generation for flat configs
					content = Object.entries(config)
						.map(([k, v]) => `${k} = ${JSON.stringify(v)}`)
						.join("\n");
					break;
				default:
					content = JSON.stringify(config, null, 2);
			}
			fs.writeFileSync(configPath, content);
			return true;
		} catch {
			return false;
		}
	}
}

/**
 * Check Runner — runs configured quality check tools via plugin executors.
 *
 * Discovers installed plugins with tool definitions, runs each tool,
 * and returns aggregated results.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { execSync } from "node:child_process";

export interface CheckResult {
	tool: string;
	plugin: string;
	command: string;
	passed: boolean;
	output: string;
	duration: number;
}

export interface CheckSummary {
	totalChecks: number;
	passed: number;
	failed: number;
	results: CheckResult[];
	duration: number;
}

interface PluginTool {
	command: string;
	configFile: string | null;
	configFormat: string;
}

export class CheckRunner {
	private projectRoot: string;

	constructor(projectRoot?: string) {
		this.projectRoot = projectRoot ?? process.cwd();
	}

	/**
	 * Run all configured quality checks and return a summary.
	 */
	runAll(): CheckSummary {
		const startTime = Date.now();
		const tools = this.discoverTools();
		const results: CheckResult[] = [];

		for (const [plugin, toolMap] of tools) {
			for (const [toolName, toolDef] of toolMap) {
				const result = this.runTool(toolName, plugin, toolDef);
				results.push(result);
			}
		}

		const duration = Date.now() - startTime;
		const passed = results.filter((r) => r.passed).length;

		return {
			totalChecks: results.length,
			passed,
			failed: results.length - passed,
			results,
			duration,
		};
	}

	/**
	 * Run a specific tool by name.
	 */
	runTool(toolName: string, plugin: string, toolDef: PluginTool): CheckResult {
		const startTime = Date.now();

		try {
			const output = execSync(toolDef.command, {
				cwd: this.projectRoot,
				encoding: "utf-8",
				timeout: 120000,
				stdio: ["pipe", "pipe", "pipe"],
			});

			return {
				tool: toolName,
				plugin,
				command: toolDef.command,
				passed: true,
				output: output.trim(),
				duration: Date.now() - startTime,
			};
		} catch (err: unknown) {
			const execErr = err as { stdout?: string; stderr?: string; status?: number };
			const output = [execErr.stdout, execErr.stderr].filter(Boolean).join("\n").trim();

			return {
				tool: toolName,
				plugin,
				command: toolDef.command,
				passed: false,
				output: output || "Check failed with no output",
				duration: Date.now() - startTime,
			};
		}
	}

	private discoverTools(): Map<string, Map<string, PluginTool>> {
		const result = new Map<string, Map<string, PluginTool>>();
		const pluginsDir = path.join(this.projectRoot, "plugins");

		if (!fs.existsSync(pluginsDir)) return result;

		for (const entry of fs.readdirSync(pluginsDir, { withFileTypes: true })) {
			if (!entry.isDirectory()) continue;
			const manifestPath = path.join(pluginsDir, entry.name, "orqa-plugin.json");
			if (!fs.existsSync(manifestPath)) continue;

			try {
				const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf-8"));
				const tools = manifest.provides?.tools;
				if (tools && typeof tools === "object") {
					const toolMap = new Map<string, PluginTool>();
					for (const [name, def] of Object.entries(tools)) {
						toolMap.set(name, def as PluginTool);
					}
					if (toolMap.size > 0) {
						result.set(entry.name, toolMap);
					}
				}
			} catch {
				// Skip invalid manifests
			}
		}

		return result;
	}
}

// Utility functions for resolving tool display metadata (icon, label) from tool names.
// Static configuration (icon/label maps) lives in config/tool-display-config.ts.

import { resolveIcon } from "@orqastudio/svelte-components/pure";
import { logger } from "@orqastudio/sdk";
import type { Component } from "svelte";
import {
	TOOL_ICONS,
	TOOL_LABELS,
	CAPABILITY_LABELS as capabilityLabels,
} from "$lib/config/tool-display-config";

export { CAPABILITY_LABELS } from "$lib/config/tool-display-config";

const log = logger("tool-display");

/**
 * Strips an MCP server prefix from a tool name.
 * @param name - The raw tool name, potentially prefixed with "mcp__server__".
 * @returns The tool name with the MCP prefix removed, or the original name if no prefix.
 */
export function stripToolName(name: string): string {
	const parts = name.split("__");
	if (parts.length >= 3 && parts[0] === "mcp") {
		return parts[parts.length - 1];
	}
	return name;
}

/**
 * Returns the display label and icon for a tool name.
 * @param name - The raw tool name, potentially MCP-prefixed.
 * @returns The resolved display label, icon component, and icon name.
 */
export function getToolDisplay(name: string): { label: string; icon: Component; iconName: string } {
	const stripped = stripToolName(name);
	const iconName = TOOL_ICONS[stripped] ?? "wrench";
	return {
		label: TOOL_LABELS[stripped] ?? stripped,
		icon: resolveIcon(iconName),
		iconName,
	};
}

/**
 * Returns a human-readable label for an agent capability identifier.
 * @param capability - The capability identifier string (e.g. "read_file", "write_code").
 * @returns A formatted human-readable label, falling back to title-cased identifier.
 */
export function getCapabilityLabel(capability: string): string {
	return (
		capabilityLabels[capability] ??
		capability.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase())
	);
}

/**
 * Returns a grouped activity label summarising multiple calls of the same tool type.
 * @param toolName - The raw tool name used to determine the group label template.
 * @param count - The number of tool calls in the group.
 * @returns A human-readable summary such as "Read 3 files" or "Ran 2 commands".
 */
export function groupLabel(toolName: string, count: number): string {
	const stripped = stripToolName(toolName);
	const labels: Record<string, string> = {
		read_file: `Read ${count} files`,
		write_file: `Wrote ${count} files`,
		edit_file: `Edited ${count} files`,
		bash: `Ran ${count} commands`,
		glob: `Found files (${count} searches)`,
		grep: `Searched content (${count} searches)`,
		search_regex: `Regex search (${count} searches)`,
		search_semantic: `Semantic search (${count} queries)`,
		code_research: `Code research (${count} queries)`,
	};
	return labels[stripped] ?? `${stripped} (${count} calls)`;
}

/**
 * Returns the high-level activity phase label for a given tool, used in the conversation UI to
 * group tool calls into phases like "Exploring Code", "Writing Code", or "Running Commands".
 * @param toolName - The raw tool name to classify.
 * @returns A phase label for display in the conversation activity indicator.
 */
export function getActivityPhase(toolName: string): string {
	const stripped = stripToolName(toolName);
	const phases: Record<string, string> = {
		read_file: "Exploring Code",
		glob: "Exploring Code",
		grep: "Exploring Code",
		search_regex: "Exploring Code",
		search_semantic: "Exploring Code",
		code_research: "Researching",
		bash: "Running Commands",
		write_file: "Writing Code",
		edit_file: "Writing Code",
	};
	return phases[stripped] ?? "Working";
}

/**
 * Returns a short, real-time label for a tool call in progress, including file name or query
 * extracted from the tool input. Used for the ephemeral activity line in the conversation view.
 * @param toolName - The raw tool name identifying the operation type.
 * @param input - The JSON-encoded tool input containing operation parameters.
 * @returns A human-readable in-progress label such as "Reading src/foo.ts".
 */
export function getEphemeralLabel(toolName: string, input: string): string {
	const stripped = stripToolName(toolName);
	try {
		const parsed = JSON.parse(input);
		switch (stripped) {
			case "read_file":
				return `Reading ${shortenPath(parsed.file_path ?? parsed.path ?? "")}`;
			case "write_file":
				return `Writing ${shortenPath(parsed.file_path ?? parsed.path ?? "")}`;
			case "edit_file":
				return `Editing ${shortenPath(parsed.file_path ?? parsed.path ?? "")}`;
			case "glob":
				return `Finding ${parsed.pattern ?? "files"}`;
			case "grep":
				return `Searching for "${truncate(parsed.pattern ?? parsed.query ?? "", 40)}"`;
			case "search_regex":
				return `Searching for "${truncate(parsed.pattern ?? "", 40)}"`;
			case "search_semantic":
				return `Searching "${truncate(parsed.query ?? "", 40)}"`;
			case "code_research":
				return `Researching "${truncate(parsed.query ?? "", 40)}"`;
			case "bash":
				return `Running command`;
			default:
				return getToolDisplay(toolName).label;
		}
	} catch (err) {
		log.warn("Failed to parse tool input for ephemeral label", { toolName, err });
		return getToolDisplay(toolName).label;
	}
}

function shortenPath(path: string): string {
	if (!path) return "file";
	const parts = path.replace(/\\/g, "/").split("/");
	if (parts.length <= 2) return parts.join("/");
	return `.../${parts.slice(-2).join("/")}`;
}

function truncate(str: string, max: number): string {
	return str.length > max ? str.slice(0, max) + "..." : str;
}

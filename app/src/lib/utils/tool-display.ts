import { resolveIcon } from "@orqastudio/svelte-components/pure";
import { logger } from "@orqastudio/sdk";
import type { Component } from "svelte";

const log = logger("tool-display");

/**
 * Strips an MCP server prefix from a tool name.
 */
export function stripToolName(name: string): string {
	const parts = name.split("__");
	if (parts.length >= 3 && parts[0] === "mcp") {
		return parts[parts.length - 1];
	}
	return name;
}

const TOOL_ICONS: Record<string, string> = {
	read_file: "file-text",
	write_file: "file-text",
	edit_file: "pencil",
	bash: "terminal",
	glob: "folder",
	grep: "search",
	search_regex: "search",
	search_semantic: "brain",
	code_research: "book-open",
};

const TOOL_LABELS: Record<string, string> = {
	read_file: "Read File",
	write_file: "Write File",
	edit_file: "Edit File",
	bash: "Run Command",
	glob: "Find Files",
	grep: "Search Content",
	search_regex: "Regex Search",
	search_semantic: "Semantic Search",
	code_research: "Code Research",
};

/**
 * Returns the display label and icon for a tool name.
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
 * Human-friendly labels for agent capability identifiers.
 */
export const CAPABILITY_LABELS: Record<string, string> = {
	file_read: "Read Files",
	file_write: "Create Files",
	file_edit: "Edit Files",
	file_search: "Find Files",
	content_search: "Search Content",
	code_search_regex: "Regex Code Search",
	code_search_semantic: "Semantic Code Search",
	code_research: "Code Research",
	shell_execute: "Run Commands",
	skill_load: "Load Knowledge",
	web_fetch: "Fetch URLs",
	web_search: "Web Search",
	notebook_edit: "Edit Notebooks",
};

export function getCapabilityLabel(capability: string): string {
	return CAPABILITY_LABELS[capability] ?? capability.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
}

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

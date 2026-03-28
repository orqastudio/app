// Static display configuration for Claude Code tool names.
// Maps tool name keys (after MCP prefix stripping) to Lucide icon names and human-readable labels.
// Consumed by tool-display.ts utility functions.

/** Maps stripped tool names to Lucide icon names. */
export const TOOL_ICONS: Record<string, string> = {
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

/** Maps stripped tool names to human-readable display labels. */
export const TOOL_LABELS: Record<string, string> = {
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

/** Human-friendly labels for agent capability identifiers. */
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

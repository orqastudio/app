import WrenchIcon from "@lucide/svelte/icons/wrench";
import FileTextIcon from "@lucide/svelte/icons/file-text";
import FilePenIcon from "@lucide/svelte/icons/file-pen";
import PencilIcon from "@lucide/svelte/icons/pencil";
import TerminalIcon from "@lucide/svelte/icons/terminal";
import FolderSearchIcon from "@lucide/svelte/icons/folder-search";
import FileSearchIcon from "@lucide/svelte/icons/file-search";
import RegexIcon from "@lucide/svelte/icons/regex";
import BrainIcon from "@lucide/svelte/icons/brain";
import BookOpenIcon from "@lucide/svelte/icons/book-open";

/**
 * Strips an MCP server prefix from a tool name.
 *
 * MCP tool names follow the pattern `mcp__<server>__<tool>` (two or more
 * double-underscore segments after the leading "mcp" segment). This function
 * returns just the final `<tool>` segment. If the name does not match the
 * pattern it is returned unchanged.
 *
 * Examples:
 *   mcp__orqa_studio_tools__read_file  → read_file
 *   mcp__chunkhound__search_regex      → search_regex
 *   read_file                          → read_file
 */
export function stripToolName(name: string): string {
	const parts = name.split("__");
	if (parts.length >= 3 && parts[0] === "mcp") {
		return parts[parts.length - 1];
	}
	return name;
}

export const TOOL_DISPLAY: Record<string, { label: string; icon: typeof WrenchIcon }> = {
	read_file: { label: "Read File", icon: FileTextIcon },
	write_file: { label: "Write File", icon: FilePenIcon },
	edit_file: { label: "Edit File", icon: PencilIcon },
	bash: { label: "Run Command", icon: TerminalIcon },
	glob: { label: "Find Files", icon: FolderSearchIcon },
	grep: { label: "Search Content", icon: FileSearchIcon },
	search_regex: { label: "Regex Search", icon: RegexIcon },
	search_semantic: { label: "Semantic Search", icon: BrainIcon },
	code_research: { label: "Code Research", icon: BookOpenIcon },
};

/**
 * Returns the display label and icon for a tool name.
 *
 * Automatically strips any MCP server prefix before performing the lookup so
 * that `mcp__chunkhound__search_regex` resolves to the same entry as
 * `search_regex`. Falls back to the stripped name and the generic wrench icon
 * when no entry is found in `TOOL_DISPLAY`.
 */
export function getToolDisplay(name: string): { label: string; icon: typeof WrenchIcon } {
	const stripped = stripToolName(name);
	return TOOL_DISPLAY[stripped] ?? { label: stripped, icon: WrenchIcon };
}

/**
 * Returns a short group label for N consecutive completed calls of the same tool.
 *
 * Examples:
 *   groupLabel("read_file", 3)   → "Read 3 files"
 *   groupLabel("bash", 2)        → "Ran 2 commands"
 *   groupLabel("search_regex", 4) → "Regex search (4 searches)"
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
 * Maps a tool name to an activity phase label shown during streaming.
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
 * Returns a short ephemeral description for a tool call in progress.
 * e.g. "Reading src/main.rs" or "Searching for 'handleClick'"
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
	} catch {
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

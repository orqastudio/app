/**
 * Shared frontmatter parsing for CLI commands.
 *
 * Uses the `yaml` package for proper YAML parsing (handles nested objects,
 * numbers, booleans, etc.) — unlike the SDK's lightweight regex parser which
 * is designed for UI display.
 */

import { readFileSync, writeFileSync } from "node:fs";
import { parse as parseYaml, stringify as stringifyYaml } from "yaml";

/**
 * Parse YAML frontmatter from a content string.
 * Returns [frontmatter, body] or null if no valid frontmatter found.
 * @param content - The file content string to parse.
 * @returns Tuple of [frontmatter, body] or null if no valid frontmatter found.
 */
export function parseFrontmatterFromContent(
	content: string,
): [Record<string, unknown>, string] | null {
	if (!content.startsWith("---\n")) return null;
	const fmEnd = content.indexOf("\n---", 4);
	if (fmEnd === -1) return null;
	try {
		const fm = parseYaml(content.substring(4, fmEnd)) as Record<
			string,
			unknown
		>;
		if (!fm || typeof fm !== "object") return null;
		return [fm, content.substring(fmEnd + 4)];
	} catch {
		return null;
	}
}

/**
 * Parse YAML frontmatter from a file path.
 * Returns just the frontmatter object, or null on failure.
 * @param filePath - Absolute path to the file to parse.
 * @returns The parsed frontmatter object, or null if parsing fails.
 */
export function parseFrontmatterFromFile(
	filePath: string,
): Record<string, unknown> | null {
	let content: string;
	try {
		content = readFileSync(filePath, "utf-8");
	} catch {
		return null;
	}
	const result = parseFrontmatterFromContent(content);
	return result ? result[0] : null;
}

/**
 * Write frontmatter back to a file using proper YAML serialisation.
 * @param filePath - Absolute path to the file to write.
 * @param fm - The frontmatter object to serialise.
 * @param body - The body content to write after the frontmatter.
 */
export function writeFrontmatter(
	filePath: string,
	fm: Record<string, unknown>,
	body: string,
): void {
	const yamlText = stringifyYaml(fm, { lineWidth: 0 }).trimEnd();
	writeFileSync(filePath, `---\n${yamlText}\n---${body}`);
}

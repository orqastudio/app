/**
 * Shared frontmatter parsing for CLI commands.
 *
 * Uses the `yaml` package for proper YAML parsing (handles nested objects,
 * numbers, booleans, etc.) — unlike the SDK's lightweight regex parser which
 * is designed for UI display.
 */
/**
 * Parse YAML frontmatter from a content string.
 * Returns [frontmatter, body] or null if no valid frontmatter found.
 * @param content - The file content string to parse.
 * @returns Tuple of [frontmatter, body] or null if no valid frontmatter found.
 */
export declare function parseFrontmatterFromContent(content: string): [Record<string, unknown>, string] | null;
/**
 * Parse YAML frontmatter from a file path.
 * Returns just the frontmatter object, or null on failure.
 * @param filePath - Absolute path to the file to parse.
 * @returns The parsed frontmatter object, or null if parsing fails.
 */
export declare function parseFrontmatterFromFile(filePath: string): Record<string, unknown> | null;
/**
 * Write frontmatter back to a file using proper YAML serialisation.
 * @param filePath - Absolute path to the file to write.
 * @param fm - The frontmatter object to serialise.
 * @param body - The body content to write after the frontmatter.
 */
export declare function writeFrontmatter(filePath: string, fm: Record<string, unknown>, body: string): void;
//# sourceMappingURL=frontmatter.d.ts.map
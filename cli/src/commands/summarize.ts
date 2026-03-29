/**
 * Summarize command — generates structured summaries for knowledge artifacts.
 *
 * orqa summarize <path>         Summarize a single knowledge artifact
 * orqa summarize --all          Summarize all knowledge artifacts in .orqa/
 * orqa summarize --check        Check which artifacts are missing summaries
 *
 * Generates template-based summaries (100-150 token target) and writes them
 * to the artifact's YAML frontmatter `summary` field.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import {
	parseFrontmatterFromContent,
	writeFrontmatter,
} from "../lib/frontmatter.js";
import { getRoot } from "../lib/root.js";

/**
 * Estimate token count for a string using a simple character-based heuristic.
 * Approximation: ~4 characters per token, consistent with GPT/Claude tokenizers.
 * @param text - The string to estimate tokens for.
 * @returns Estimated token count.
 */
function estimateTokens(text: string): number {
	return Math.ceil(text.length / 4);
}

const USAGE = `
Usage: orqa summarize <path|--all|--check> [options]

Generate structured summaries for knowledge artifacts. Summaries are written
to the artifact's YAML frontmatter 'summary' field (100-150 token target).

Arguments:
  <path>         Path to a single knowledge artifact (.md file)

Options:
  --all          Summarize all knowledge artifacts missing summaries
  --check        Report which artifacts are missing summaries (no changes)
  --force        Overwrite existing summaries (default: skip if present)
  --help, -h     Show this help message
`.trim();

/** Summary target range in tokens. */
const SUMMARY_MIN_TOKENS = 80;
const SUMMARY_MAX_TOKENS = 170;

// ---------------------------------------------------------------------------
// Summary Generation
// ---------------------------------------------------------------------------

/**
 * Generate a structured summary from artifact content.
 *
 * Template-based extraction: pulls the title, first paragraph, and heading
 * structure to create a concise summary in the format:
 *   Purpose: <what this knowledge covers>
 *   Key points: <3-5 bullet points>
 *   When to use: <triggers/conditions>
 * @param title - The artifact title.
 * @param body - The artifact body content (markdown).
 * @param description - Optional frontmatter description field.
 * @returns Generated summary string.
 */
function generateSummary(
	title: string,
	body: string,
	description?: string,
): string {
	const parts: string[] = [];

	// Purpose — from title and first meaningful paragraph
	const purpose = extractPurpose(title, body, description);
	parts.push(`Purpose: ${purpose}`);

	// Key points — from headings and first sentences
	const keyPoints = extractKeyPoints(body);
	if (keyPoints.length > 0) {
		const bullets = keyPoints.map((p) => `- ${p}`).join("\n");
		parts.push(`Key points:\n${bullets}`);
	}

	// When to use — from description or content cues
	const whenToUse = extractWhenToUse(body, description);
	if (whenToUse) {
		parts.push(`When to use: ${whenToUse}`);
	}

	let summary = parts.join("\n");

	// Trim if over budget
	const tokens = estimateTokens(summary);
	if (tokens > SUMMARY_MAX_TOKENS) {
		summary = trimToTokenBudget(summary, SUMMARY_MAX_TOKENS);
	}

	return summary;
}

/**
 * Extract the purpose from title, first paragraph, and description.
 * @param title - The artifact title.
 * @param body - The artifact body content.
 * @param description - Optional frontmatter description.
 * @returns A one-line purpose string.
 */
function extractPurpose(
	title: string,
	body: string,
	description?: string,
): string {
	// Use description if available and concise
	if (description) {
		const firstLine = description.split("\n")[0].trim();
		if (firstLine.length > 10 && firstLine.length < 200) {
			return firstLine;
		}
	}

	// Fall back to first non-heading paragraph
	const lines = body.split("\n");
	for (const line of lines) {
		const trimmed = line.trim();
		if (
			trimmed.length > 20 &&
			!trimmed.startsWith("#") &&
			!trimmed.startsWith("-") &&
			!trimmed.startsWith("|") &&
			!trimmed.startsWith("```")
		) {
			return trimmed.length > 150
				? trimmed.substring(0, 147) + "..."
				: trimmed;
		}
	}

	return title;
}

/**
 * Extract key points from headings and their first sentences.
 * @param body - The artifact body content.
 * @returns Array of key point strings (up to 5).
 */
function extractKeyPoints(body: string): string[] {
	const points: string[] = [];
	const lines = body.split("\n");
	const maxPoints = 5;

	for (let i = 0; i < lines.length && points.length < maxPoints; i++) {
		const line = lines[i].trim();

		// Match ## or ### headings (skip # which is the title)
		if (/^#{2,3}\s+/.test(line)) {
			const heading = line.replace(/^#{2,3}\s+/, "").trim();

			// Look for the first meaningful line after the heading
			for (let j = i + 1; j < lines.length && j < i + 4; j++) {
				const nextLine = lines[j].trim();
				if (
					nextLine.length > 15 &&
					!nextLine.startsWith("#") &&
					!nextLine.startsWith("```") &&
					!nextLine.startsWith("|")
				) {
					const point =
						nextLine.length > 80
							? nextLine.substring(0, 77) + "..."
							: nextLine;
					points.push(`${heading}: ${point}`);
					break;
				}
			}

			// If no meaningful line found, just use the heading
			if (
				points.length === 0 ||
				!points[points.length - 1].startsWith(heading)
			) {
				points.push(heading);
			}
		}
	}

	return points;
}

/**
 * Extract "when to use" from description or body content cues.
 * @param body - The artifact body content.
 * @param description - Optional frontmatter description.
 * @returns A "when to use" string, or null if not found.
 */
function extractWhenToUse(body: string, description?: string): string | null {
	// Check description for "Use when:" pattern
	if (description) {
		const useWhenMatch = description.match(
			/use when[:\s]+(.+?)(?:\n|$)/i,
		);
		if (useWhenMatch) {
			return useWhenMatch[1].trim();
		}
	}

	// Check body for "when to use" or "applies to" sections
	const lines = body.split("\n");
	for (let i = 0; i < lines.length; i++) {
		const line = lines[i].trim().toLowerCase();
		if (
			line.includes("when to use") ||
			line.includes("applies to") ||
			line.includes("use this when")
		) {
			// Return the next meaningful line
			for (let j = i + 1; j < lines.length && j < i + 3; j++) {
				const next = lines[j].trim();
				if (next.length > 10 && !next.startsWith("#")) {
					return next.length > 100
						? next.substring(0, 97) + "..."
						: next;
				}
			}
		}
	}

	return null;
}

/**
 * Trim text to fit within a token budget.
 * @param text - The text to trim.
 * @param maxTokens - Maximum number of tokens allowed.
 * @returns Trimmed text within the token budget.
 */
function trimToTokenBudget(text: string, maxTokens: number): string {
	const maxChars = maxTokens * 4;
	if (text.length <= maxChars) return text;

	// Trim at last newline within budget
	const truncated = text.substring(0, maxChars);
	const lastNewline = truncated.lastIndexOf("\n");
	if (lastNewline > maxChars * 0.6) {
		return truncated.substring(0, lastNewline);
	}
	return truncated;
}

// ---------------------------------------------------------------------------
// File Processing
// ---------------------------------------------------------------------------

interface SummarizeResult {
	path: string;
	id: string;
	title: string;
	action: "created" | "skipped" | "error";
	tokens?: number;
	error?: string;
}

/**
 * Process a single artifact file: read, generate summary, write back.
 * @param filePath - Path to the artifact .md file.
 * @param force - Whether to overwrite an existing summary.
 * @returns Result indicating whether the summary was created or skipped.
 */
function summarizeFile(
	filePath: string,
	force: boolean,
): SummarizeResult {
	const absPath = path.resolve(filePath);

	let content: string;
	try {
		content = fs.readFileSync(absPath, "utf-8");
	} catch (err) {
		return {
			path: absPath,
			id: "",
			title: "",
			action: "error",
			error: `Cannot read: ${err instanceof Error ? err.message : String(err)}`,
		};
	}

	const parsed = parseFrontmatterFromContent(content);
	if (!parsed) {
		return {
			path: absPath,
			id: "",
			title: "",
			action: "error",
			error: "No valid YAML frontmatter found",
		};
	}

	const [fm, body] = parsed;
	const id = (fm.id as string) ?? "";
	const title = (fm.title as string) ?? path.basename(absPath, ".md");

	// Skip if summary exists and --force not set
	if (fm.summary && !force) {
		return { path: absPath, id, title, action: "skipped" };
	}

	const description =
		typeof fm.description === "string" ? fm.description : undefined;
	const summary = generateSummary(title, body, description);
	const tokens = estimateTokens(summary);

	fm.summary = summary;
	writeFrontmatter(absPath, fm, body);

	return { path: absPath, id, title, action: "created", tokens };
}

/**
 * Find all knowledge artifact files in .orqa/ directories.
 * @param projectRoot - Absolute path to the project root.
 * @returns Array of absolute paths to knowledge artifact .md files.
 */
function findKnowledgeArtifacts(projectRoot: string): string[] {
	const results: string[] = [];
	const dirs = [
		path.join(projectRoot, ".orqa", "documentation", "knowledge"),
		path.join(projectRoot, "app", ".orqa", "documentation", "knowledge"),
	];

	for (const dir of dirs) {
		if (!fs.existsSync(dir)) continue;

		let entries: fs.Dirent[];
		try {
			entries = fs.readdirSync(dir, { withFileTypes: true });
		} catch {
			continue;
		}

		for (const entry of entries) {
			if (entry.isFile() && entry.name.endsWith(".md")) {
				results.push(path.join(dir, entry.name));
			}
		}
	}

	return results;
}

/**
 * Check which artifacts are missing summaries (no changes made).
 * @param projectRoot - Absolute path to the project root.
 */
function checkMissingSummaries(projectRoot: string): void {
	const files = findKnowledgeArtifacts(projectRoot);
	let missing = 0;
	let hasSummary = 0;
	let belowTarget = 0;

	for (const filePath of files) {
		let content: string;
		try {
			content = fs.readFileSync(filePath, "utf-8");
		} catch {
			continue;
		}

		const parsed = parseFrontmatterFromContent(content);
		if (!parsed) continue;

		const [fm] = parsed;
		const id = (fm.id as string) ?? path.basename(filePath, ".md");
		const title = (fm.title as string) ?? "";

		if (!fm.summary) {
			console.log(`  MISSING  ${id}  ${title}`);
			missing++;
		} else {
			const tokens = estimateTokens(fm.summary as string);
			if (tokens < SUMMARY_MIN_TOKENS) {
				console.log(
					`  SHORT    ${id}  ${title} (${tokens} tokens, target ${SUMMARY_MIN_TOKENS}-${SUMMARY_MAX_TOKENS})`,
				);
				belowTarget++;
			}
			hasSummary++;
		}
	}

	console.log(
		`\n${files.length} artifacts: ${hasSummary} with summary, ${missing} missing, ${belowTarget} below target`,
	);

	if (missing > 0) {
		console.log(`\nRun 'orqa summarize --all' to generate missing summaries.`);
	}
}

// ---------------------------------------------------------------------------
// CLI Entry Point
// ---------------------------------------------------------------------------

/**
 * Dispatch the summarize command: generate or check summaries for knowledge artifacts.
 * @param args - CLI arguments after "summarize".
 */
export async function runSummarizeCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	const force = args.includes("--force");
	const doAll = args.includes("--all");
	const doCheck = args.includes("--check");
	const root = getRoot();

	if (doCheck) {
		console.log("Checking knowledge artifact summaries...\n");
		checkMissingSummaries(root);
		return;
	}

	if (doAll) {
		console.log("Generating summaries for knowledge artifacts...\n");
		const files = findKnowledgeArtifacts(root);

		if (files.length === 0) {
			console.log("No knowledge artifacts found.");
			return;
		}

		let created = 0;
		let skipped = 0;
		let errors = 0;

		for (const file of files) {
			const result = summarizeFile(file, force);
			const rel = path.relative(root, result.path);

			switch (result.action) {
				case "created":
					console.log(
						`  CREATED  ${result.id}  ${result.title} (${result.tokens} tokens)`,
					);
					created++;
					break;
				case "skipped":
					skipped++;
					break;
				case "error":
					console.error(`  ERROR    ${rel}: ${result.error}`);
					errors++;
					break;
			}
		}

		console.log(
			`\n${files.length} artifacts: ${created} summarized, ${skipped} skipped (existing), ${errors} errors`,
		);
		return;
	}

	// Single file mode
	const filePath = args.find((a) => !a.startsWith("--"));
	if (!filePath) {
		console.error("Provide a file path or use --all / --check.");
		console.error(USAGE);
		process.exit(1);
	}

	const result = summarizeFile(filePath, force);

	switch (result.action) {
		case "created":
			console.log(
				`Summary generated for ${result.id} (${result.tokens} tokens)`,
			);
			break;
		case "skipped":
			console.log(
				`${result.id} already has a summary. Use --force to overwrite.`,
			);
			break;
		case "error":
			console.error(`Error: ${result.error}`);
			process.exit(1);
	}
}

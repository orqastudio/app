export interface FrontmatterResult {
	metadata: Record<string, string | string[]>;
	body: string;
}

/**
 * Parse YAML-like frontmatter delimited by `---` from a content string.
 * Handles simple key-value pairs, array values (lines starting with `  - `),
 * and multiline string values (using `|` or `>` block scalar indicators).
 *
 * This is a lightweight regex-based parser -- not a full YAML parser.
 */
export function parseFrontmatter(content: string): FrontmatterResult {
	// Normalise CRLF line endings to LF so the regex anchors ($) and
	// continuation-line checks work correctly regardless of the file's
	// line-ending style on disk (Windows files often use CRLF).
	const trimmed = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n").trimStart();
	if (!trimmed.startsWith("---")) {
		return { metadata: {}, body: content };
	}

	// Find the closing `---` delimiter (must be on its own line)
	const closingIndex = trimmed.indexOf("\n---", 3);
	if (closingIndex === -1) {
		return { metadata: {}, body: content };
	}

	const frontmatterBlock = trimmed.slice(4, closingIndex); // skip opening "---\n"
	const body = trimmed.slice(closingIndex + 4).replace(/^\n/, ""); // skip closing "---\n"

	const metadata: Record<string, string | string[]> = {};
	const lines = frontmatterBlock.split("\n");

	let currentKey: string | null = null;
	let currentArrayValues: string[] | null = null;
	let currentMultilineValue: string[] | null = null;

	function flushCurrent() {
		if (currentKey !== null && currentArrayValues !== null) {
			metadata[currentKey] = currentArrayValues;
			currentKey = null;
			currentArrayValues = null;
		}
		if (currentKey !== null && currentMultilineValue !== null) {
			metadata[currentKey] = currentMultilineValue.join("\n").trim();
			currentKey = null;
			currentMultilineValue = null;
		}
	}

	for (const line of lines) {
		// Array item: starts with `  - ` (or `- ` with any leading whitespace)
		const arrayMatch = line.match(/^\s+-\s+(.+)$/);
		if (arrayMatch?.[1] && currentKey !== null && currentArrayValues !== null) {
			currentArrayValues.push(arrayMatch[1].trim());
			continue;
		}

		// Continuation of multiline string value (indented line that is not an array item)
		if (
			currentKey !== null &&
			currentMultilineValue !== null &&
			(line.startsWith("  ") || line.startsWith("\t")) &&
			!arrayMatch
		) {
			currentMultilineValue.push(line.trimStart());
			continue;
		}

		// New key-value pair
		const kvMatch = line.match(/^([a-zA-Z_][\w-]*)\s*:\s*(.*)$/);
		if (kvMatch?.[1] && kvMatch[2] !== undefined) {
			flushCurrent();

			const key = kvMatch[1];
			let value = kvMatch[2].trim();

			// Block scalar indicator (| or >) — start multiline collection
			if (value === "|" || value === ">") {
				currentKey = key;
				currentMultilineValue = [];
				continue;
			}

			// Inline array: [item1, item2, ...]
			if (value.startsWith("[") && value.endsWith("]")) {
				const inner = value.slice(1, -1);
				metadata[key] = inner
					.split(",")
					.map((s) => s.trim())
					.filter((s) => s.length > 0);
				continue;
			}

			// Empty value — could be followed by array items
			if (value === "") {
				currentKey = key;
				currentArrayValues = [];
				continue;
			}

			// Strip surrounding quotes if present
			if (
				(value.startsWith('"') && value.endsWith('"')) ||
				(value.startsWith("'") && value.endsWith("'"))
			) {
				value = value.slice(1, -1);
			}

			metadata[key] = value;
		}
	}

	flushCurrent();

	return { metadata, body };
}

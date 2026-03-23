import { describe, it, expect } from "vitest";
import { parseFrontmatter } from "../src/utils/frontmatter.js";

describe("parseFrontmatter", () => {
	it("parses valid frontmatter with simple key-value pairs", () => {
		const content = `---
id: EPIC-048
title: My Epic
status: draft
---
# Body content here`;

		const result = parseFrontmatter(content);
		expect(result.metadata).toEqual({
			id: "EPIC-048",
			title: "My Epic",
			status: "draft",
		});
		expect(result.body).toBe("# Body content here");
	});

	it("returns empty metadata and full body when no frontmatter present", () => {
		const content = "# Just a heading\n\nSome body text.";
		const result = parseFrontmatter(content);
		expect(result.metadata).toEqual({});
		expect(result.body).toBe(content);
	});

	it("returns empty metadata when frontmatter is not closed", () => {
		const content = `---
id: EPIC-001
title: Unclosed
`;
		const result = parseFrontmatter(content);
		expect(result.metadata).toEqual({});
		expect(result.body).toBe(content);
	});

	it("handles malformed YAML gracefully (non-key-value lines ignored)", () => {
		const content = `---
id: TASK-001
this is not valid yaml
status: todo
---
Body`;

		const result = parseFrontmatter(content);
		expect(result.metadata).toEqual({
			id: "TASK-001",
			status: "todo",
		});
		expect(result.body).toBe("Body");
	});

	it("parses YAML-style array values (dash-prefixed items)", () => {
		const content = `---
id: EPIC-042
skills:
  - planning
  - composability
  - architecture
---
Body`;

		const result = parseFrontmatter(content);
		expect(result.metadata.skills).toEqual(["planning", "composability", "architecture"]);
	});

	it("parses inline array values [item1, item2]", () => {
		const content = `---
id: TASK-005
tags: [frontend, svelte, urgent]
---
Body`;

		const result = parseFrontmatter(content);
		expect(result.metadata.tags).toEqual(["frontend", "svelte", "urgent"]);
	});

	it("handles block scalar indicator (pipe |)", () => {
		const content = `---
id: AD-001
description: |
  This is a multiline
  description value
  spanning three lines
---
Body`;

		const result = parseFrontmatter(content);
		expect(result.metadata.description).toBe(
			"This is a multiline\ndescription value\nspanning three lines"
		);
	});

	it("handles block scalar indicator (folded >)", () => {
		const content = `---
id: AD-002
description: >
  Folded multiline
  description here
---
Body`;

		const result = parseFrontmatter(content);
		expect(result.metadata.description).toBe("Folded multiline\ndescription here");
	});

	it("strips surrounding quotes from values", () => {
		const content = `---
title: "Quoted Title"
label: 'Single Quoted'
---
Body`;

		const result = parseFrontmatter(content);
		expect(result.metadata.title).toBe("Quoted Title");
		expect(result.metadata.label).toBe("Single Quoted");
	});

	it("handles CRLF line endings", () => {
		const content = "---\r\nid: TASK-001\r\nstatus: done\r\n---\r\nBody";
		const result = parseFrontmatter(content);
		expect(result.metadata).toEqual({
			id: "TASK-001",
			status: "done",
		});
		expect(result.body).toBe("Body");
	});

	it("handles empty array when no items follow", () => {
		const content = `---
id: EPIC-010
depends-on:
status: draft
---
Body`;

		const result = parseFrontmatter(content);
		// Empty value followed by a new key — the empty value becomes an empty array
		expect(result.metadata["depends-on"]).toEqual([]);
		expect(result.metadata.status).toBe("draft");
	});

	it("handles content with leading whitespace before frontmatter", () => {
		const content = `  ---
id: TASK-003
---
Body`;

		const result = parseFrontmatter(content);
		expect(result.metadata.id).toBe("TASK-003");
	});
});

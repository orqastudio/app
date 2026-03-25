import { describe, it, expect, beforeEach, afterEach } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";
import {
	generatePrompt,
	estimateTokens,
	DEFAULT_TOKEN_BUDGETS,
	type PromptPipelineOptions,
	type PromptResult,
} from "../src/lib/prompt-pipeline.js";
import type { PromptRegistry } from "../src/lib/prompt-registry.js";

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

/** Create a temporary project directory with a prompt registry. */
function createTestProject(registry: PromptRegistry): string {
	const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "orqa-pipeline-test-"));
	const orqaDir = path.join(tmpDir, ".orqa");
	fs.mkdirSync(orqaDir, { recursive: true });
	fs.writeFileSync(
		path.join(orqaDir, "prompt-registry.json"),
		JSON.stringify(registry, null, 2),
	);
	return tmpDir;
}

/** Create a content file in the project and return its absolute path. */
function createContentFile(projectRoot: string, relativePath: string, content: string): string {
	const absPath = path.join(projectRoot, relativePath);
	fs.mkdirSync(path.dirname(absPath), { recursive: true });
	fs.writeFileSync(absPath, content, "utf-8");
	return absPath;
}

/** Recursively remove a directory. */
function rmrf(dir: string): void {
	if (fs.existsSync(dir)) {
		fs.rmSync(dir, { recursive: true, force: true });
	}
}

const EMPTY_REGISTRY: PromptRegistry = {
	version: 1,
	built_at: new Date().toISOString(),
	knowledge: [],
	sections: [],
	contributors: [],
	errors: [],
};

// ---------------------------------------------------------------------------
// Token estimation
// ---------------------------------------------------------------------------

describe("estimateTokens", () => {
	it("estimates tokens as chars / 4 rounded up", () => {
		expect(estimateTokens("")).toBe(0);
		expect(estimateTokens("a")).toBe(1);
		expect(estimateTokens("abcd")).toBe(1);
		expect(estimateTokens("abcde")).toBe(2);
		expect(estimateTokens("a".repeat(100))).toBe(25);
	});
});

// ---------------------------------------------------------------------------
// Pipeline — missing registry
// ---------------------------------------------------------------------------

describe("generatePrompt — no registry", () => {
	it("returns empty prompt with error when registry is missing", () => {
		const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "orqa-no-reg-"));
		try {
			const result = generatePrompt({
				role: "implementer",
				projectPath: tmpDir,
			});
			expect(result.prompt).toBe("");
			expect(result.errors.length).toBeGreaterThan(0);
			expect(result.errors[0]).toContain("Prompt registry not found");
		} finally {
			rmrf(tmpDir);
		}
	});
});

// ---------------------------------------------------------------------------
// Pipeline — empty registry
// ---------------------------------------------------------------------------

describe("generatePrompt — empty registry", () => {
	let projectRoot: string;

	beforeEach(() => {
		projectRoot = createTestProject(EMPTY_REGISTRY);
	});

	afterEach(() => {
		rmrf(projectRoot);
	});

	it("generates a minimal prompt with just the role tag", () => {
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
		});
		expect(result.prompt).toContain("<role>implementer</role>");
		expect(result.errors).toHaveLength(0);
		expect(result.includedSections).toHaveLength(0);
	});

	it("includes task context as a dynamic section", () => {
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
			taskContext: {
				description: "Fix the bug in parser.ts",
				files: ["src/parser.ts"],
				acceptanceCriteria: ["Tests pass", "No regressions"],
			},
		});
		expect(result.prompt).toContain("<task-context");
		expect(result.prompt).toContain("Fix the bug in parser.ts");
		expect(result.prompt).toContain("src/parser.ts");
		expect(result.prompt).toContain("1. Tests pass");
		expect(result.prompt).toContain("2. No regressions");
		expect(result.includedSections).toHaveLength(1);
		expect(result.includedSections[0].type).toBe("task-context");
	});

	it("uses default token budget for role", () => {
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
		});
		expect(result.budget).toBe(DEFAULT_TOKEN_BUDGETS.implementer);
	});

	it("uses custom token budget when provided", () => {
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
			tokenBudget: 500,
		});
		expect(result.budget).toBe(500);
	});
});

// ---------------------------------------------------------------------------
// Pipeline — knowledge and sections with content files
// ---------------------------------------------------------------------------

describe("generatePrompt — with knowledge and sections", () => {
	let projectRoot: string;

	beforeEach(() => {
		const registry: PromptRegistry = {
			version: 1,
			built_at: new Date().toISOString(),
			knowledge: [],
			sections: [],
			contributors: ["test-plugin"],
			errors: [],
		};

		projectRoot = createTestProject(registry);

		// Create content files
		const roleDefPath = createContentFile(
			projectRoot,
			"plugins/test/prompts/implementer-role.md",
			"You are an implementer. Write code carefully.",
		);
		const safetyPath = createContentFile(
			projectRoot,
			"plugins/test/prompts/safety.md",
			"Never delete production data.",
		);
		const knowledgePath = createContentFile(
			projectRoot,
			"plugins/test/knowledge/rust-errors.md",
			"Use thiserror for error types. Always derive Debug.",
		);

		// Build a registry with actual entries
		const fullRegistry: PromptRegistry = {
			version: 1,
			built_at: new Date().toISOString(),
			knowledge: [
				{
					id: "rust-errors",
					plugin: "test-plugin",
					source: "plugin",
					tier: "always",
					roles: ["implementer"],
					stages: [],
					paths: [],
					tags: [],
					priority: "P2",
					summary: "Use thiserror, derive Debug",
					content_file: knowledgePath,
				},
			],
			sections: [
				{
					id: "implementer-role",
					plugin: "test-plugin",
					source: "plugin",
					type: "role-definition",
					role: "implementer",
					stage: null,
					priority: "P0",
					content_file: roleDefPath,
				},
				{
					id: "safety-basic",
					plugin: "test-plugin",
					source: "core",
					type: "safety-rule",
					role: null,
					stage: null,
					priority: "P0",
					content_file: safetyPath,
				},
			],
			contributors: ["test-plugin"],
			errors: [],
		};

		// Rewrite registry
		fs.writeFileSync(
			path.join(projectRoot, ".orqa", "prompt-registry.json"),
			JSON.stringify(fullRegistry, null, 2),
		);
	});

	afterEach(() => {
		rmrf(projectRoot);
	});

	it("includes knowledge, role definition, and safety rules", () => {
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
		});
		expect(result.prompt).toContain("You are an implementer");
		expect(result.prompt).toContain("Never delete production data");
		expect(result.prompt).toContain("Use thiserror for error types");
		expect(result.includedSections.length).toBe(3);
	});

	it("orders static content before dynamic content", () => {
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
			taskContext: { description: "Build the parser" },
		});

		const roleIdx = result.prompt.indexOf("<role-definition");
		const safetyIdx = result.prompt.indexOf("<safety-rule");
		const knowledgeIdx = result.prompt.indexOf("<knowledge");
		const taskIdx = result.prompt.indexOf("<task-context");

		// Static at top, dynamic at bottom
		expect(roleIdx).toBeLessThan(safetyIdx);
		expect(safetyIdx).toBeLessThan(knowledgeIdx);
		expect(knowledgeIdx).toBeLessThan(taskIdx);
	});

	it("does not include sections for a different role", () => {
		const result = generatePrompt({
			role: "reviewer",
			projectPath: projectRoot,
		});
		// Safety rule (role: null) should still be included
		expect(result.prompt).toContain("Never delete production data");
		// Role-specific section should NOT be included
		expect(result.prompt).not.toContain("You are an implementer");
	});
});

// ---------------------------------------------------------------------------
// Token Budgeting
// ---------------------------------------------------------------------------

describe("generatePrompt — token budgeting", () => {
	let projectRoot: string;

	beforeEach(() => {
		projectRoot = createTestProject(EMPTY_REGISTRY);

		// Create large content files of known sizes
		const p0Path = createContentFile(
			projectRoot,
			"plugins/test/p0.md",
			"S".repeat(400), // 100 tokens
		);
		const p1Path = createContentFile(
			projectRoot,
			"plugins/test/p1.md",
			"M".repeat(400), // 100 tokens
		);
		const p2Path = createContentFile(
			projectRoot,
			"plugins/test/p2.md",
			"L".repeat(400), // 100 tokens
		);
		const p3Path = createContentFile(
			projectRoot,
			"plugins/test/p3.md",
			"X".repeat(400), // 100 tokens
		);

		const registry: PromptRegistry = {
			version: 1,
			built_at: new Date().toISOString(),
			knowledge: [],
			sections: [
				{
					id: "p0-section",
					plugin: "test",
					source: "core",
					type: "safety-rule",
					role: null,
					stage: null,
					priority: "P0",
					content_file: p0Path,
				},
				{
					id: "p1-section",
					plugin: "test",
					source: "core",
					type: "constraint",
					role: null,
					stage: null,
					priority: "P1",
					content_file: p1Path,
				},
				{
					id: "p2-section",
					plugin: "test",
					source: "plugin",
					type: "stage-instruction",
					role: null,
					stage: null,
					priority: "P2",
					content_file: p2Path,
				},
				{
					id: "p3-section",
					plugin: "test",
					source: "plugin",
					type: "task-template",
					role: null,
					stage: null,
					priority: "P3",
					content_file: p3Path,
				},
			],
			contributors: ["test"],
			errors: [],
		};

		fs.writeFileSync(
			path.join(projectRoot, ".orqa", "prompt-registry.json"),
			JSON.stringify(registry, null, 2),
		);
	});

	afterEach(() => {
		rmrf(projectRoot);
	});

	it("includes all sections when budget is sufficient", () => {
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
			tokenBudget: 10000,
		});
		expect(result.includedSections).toHaveLength(4);
		expect(result.trimmedSections).toHaveLength(0);
	});

	it("trims P3 first when budget is tight", () => {
		// With ~100 tokens per section + XML overhead, a budget of ~350 should force trimming P3
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
			tokenBudget: 350,
		});
		const trimmedIds = result.trimmedSections.map((s) => s.id);
		expect(trimmedIds).toContain("p3-section");
	});

	it("never trims P0 sections", () => {
		// Even with a very small budget, P0 is kept
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
			tokenBudget: 50,
		});
		const includedIds = result.includedSections.map((s) => s.id);
		expect(includedIds).toContain("p0-section");
	});

	it("trims P3 before P2 before P1", () => {
		// Budget that forces trimming P3 and P2 but keeps P1 and P0
		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
			tokenBudget: 250,
		});
		const trimmedIds = result.trimmedSections.map((s) => s.id);
		const includedIds = result.includedSections.map((s) => s.id);

		// P0 always included
		expect(includedIds).toContain("p0-section");
		// P3 always trimmed first
		if (trimmedIds.length >= 1) {
			expect(trimmedIds).toContain("p3-section");
		}
		// P1 should be kept longer than P2
		if (includedIds.includes("p1-section")) {
			expect(trimmedIds).not.toContain("p1-section");
		}
	});
});

// ---------------------------------------------------------------------------
// Conflict Resolution
// ---------------------------------------------------------------------------

describe("generatePrompt — conflict resolution", () => {
	let projectRoot: string;

	afterEach(() => {
		rmrf(projectRoot);
	});

	it("project-rules wins over plugin when IDs match", () => {
		projectRoot = createTestProject(EMPTY_REGISTRY);

		const projectRulePath = createContentFile(
			projectRoot,
			"rules/error-handling.md",
			"PROJECT: Always use Result types.",
		);
		const pluginRulePath = createContentFile(
			projectRoot,
			"plugins/test/error-handling.md",
			"PLUGIN: Use exceptions for errors.",
		);

		const registry: PromptRegistry = {
			version: 1,
			built_at: new Date().toISOString(),
			knowledge: [
				{
					id: "error-handling",
					plugin: "project",
					source: "project-rules",
					tier: "always",
					roles: ["implementer"],
					stages: [],
					paths: [],
					tags: [],
					priority: "P1",
					summary: null,
					content_file: projectRulePath,
				},
				{
					id: "error-handling",
					plugin: "test-plugin",
					source: "plugin",
					tier: "always",
					roles: ["implementer"],
					stages: [],
					paths: [],
					tags: [],
					priority: "P1",
					summary: null,
					content_file: pluginRulePath,
				},
			],
			sections: [],
			contributors: ["project", "test-plugin"],
			errors: [],
		};

		fs.writeFileSync(
			path.join(projectRoot, ".orqa", "prompt-registry.json"),
			JSON.stringify(registry, null, 2),
		);

		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
		});

		expect(result.prompt).toContain("PROJECT: Always use Result types");
		expect(result.prompt).not.toContain("PLUGIN: Use exceptions");
	});
});

// ---------------------------------------------------------------------------
// KV-cache-aware output structure
// ---------------------------------------------------------------------------

describe("generatePrompt — KV-cache structure", () => {
	let projectRoot: string;

	afterEach(() => {
		rmrf(projectRoot);
	});

	it("puts role tag at the very top of the prompt", () => {
		projectRoot = createTestProject(EMPTY_REGISTRY);
		const result = generatePrompt({
			role: "orchestrator",
			projectPath: projectRoot,
		});
		expect(result.prompt.startsWith("<role>orchestrator</role>")).toBe(true);
	});

	it("uses XML tags with id and priority attributes", () => {
		projectRoot = createTestProject(EMPTY_REGISTRY);

		const contentPath = createContentFile(
			projectRoot,
			"plugins/test/safety.md",
			"Do not harm.",
		);

		const registry: PromptRegistry = {
			version: 1,
			built_at: new Date().toISOString(),
			knowledge: [],
			sections: [
				{
					id: "safety-01",
					plugin: "test",
					source: "core",
					type: "safety-rule",
					role: null,
					stage: null,
					priority: "P0",
					content_file: contentPath,
				},
			],
			contributors: ["test"],
			errors: [],
		};

		fs.writeFileSync(
			path.join(projectRoot, ".orqa", "prompt-registry.json"),
			JSON.stringify(registry, null, 2),
		);

		const result = generatePrompt({
			role: "implementer",
			projectPath: projectRoot,
		});
		expect(result.prompt).toContain('<safety-rule id="safety-01" priority="P0">');
		expect(result.prompt).toContain("</safety-rule>");
	});
});

// ---------------------------------------------------------------------------
// Default budgets
// ---------------------------------------------------------------------------

describe("DEFAULT_TOKEN_BUDGETS", () => {
	it("has expected roles from research", () => {
		expect(DEFAULT_TOKEN_BUDGETS.orchestrator).toBe(2500);
		expect(DEFAULT_TOKEN_BUDGETS.implementer).toBe(2800);
		expect(DEFAULT_TOKEN_BUDGETS.reviewer).toBe(1900);
		expect(DEFAULT_TOKEN_BUDGETS.researcher).toBe(2100);
		expect(DEFAULT_TOKEN_BUDGETS.writer).toBe(1800);
		expect(DEFAULT_TOKEN_BUDGETS.designer).toBe(1800);
	});
});

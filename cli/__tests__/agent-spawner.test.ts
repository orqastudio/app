import { describe, it, expect, beforeEach, afterEach } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";
import {
	selectModelTier,
	createAgentConfig,
	isValidRole,
	modelTierLabel,
	serializeFindings,
	parseFindingsHeader,
	UNIVERSAL_ROLES,
	DEFAULT_MODEL_TIERS,
	ROLE_TOOL_CONSTRAINTS,
	type UniversalRole,
	type FindingsDocument,
} from "../src/lib/agent-spawner.js";
import type { PromptRegistry } from "../src/lib/prompt-registry.js";

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

function createTestProject(registry: PromptRegistry): string {
	const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "orqa-spawner-test-"));
	const orqaDir = path.join(tmpDir, ".orqa");
	fs.mkdirSync(orqaDir, { recursive: true });
	fs.writeFileSync(
		path.join(orqaDir, "prompt-registry.json"),
		JSON.stringify(registry, null, 2),
	);
	return tmpDir;
}

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
// Model Tier Selection
// ---------------------------------------------------------------------------

describe("selectModelTier", () => {
	it("returns default tiers for each role", () => {
		expect(selectModelTier("orchestrator")).toBe("opus");
		expect(selectModelTier("planner")).toBe("opus");
		expect(selectModelTier("implementer")).toBe("sonnet");
		expect(selectModelTier("reviewer")).toBe("sonnet");
		expect(selectModelTier("researcher")).toBe("sonnet");
		expect(selectModelTier("writer")).toBe("sonnet");
		expect(selectModelTier("designer")).toBe("sonnet");
		expect(selectModelTier("governance_steward")).toBe("sonnet");
	});

	it("upgrades implementer to opus for complex tasks", () => {
		expect(selectModelTier("implementer", "complex")).toBe("opus");
	});

	it("does NOT upgrade other roles for complex tasks", () => {
		expect(selectModelTier("reviewer", "complex")).toBe("sonnet");
		expect(selectModelTier("writer", "complex")).toBe("sonnet");
		expect(selectModelTier("researcher", "complex")).toBe("sonnet");
	});

	it("keeps orchestrator as opus regardless of complexity", () => {
		expect(selectModelTier("orchestrator", "simple")).toBe("opus");
		expect(selectModelTier("orchestrator", "complex")).toBe("opus");
	});

	it("respects custom overrides", () => {
		expect(
			selectModelTier("implementer", "simple", { implementer: "haiku" }),
		).toBe("haiku");
		expect(
			selectModelTier("orchestrator", "simple", { orchestrator: "sonnet" }),
		).toBe("sonnet");
	});

	it("overrides take precedence over complexity upgrade", () => {
		expect(
			selectModelTier("implementer", "complex", { implementer: "haiku" }),
		).toBe("haiku");
	});
});

// ---------------------------------------------------------------------------
// Universal Roles
// ---------------------------------------------------------------------------

describe("UNIVERSAL_ROLES", () => {
	it("contains exactly 8 roles", () => {
		expect(UNIVERSAL_ROLES).toHaveLength(8);
	});

	it("every role has default model tiers", () => {
		for (const role of UNIVERSAL_ROLES) {
			expect(DEFAULT_MODEL_TIERS[role]).toBeDefined();
		}
	});

	it("every role has tool constraints", () => {
		for (const role of UNIVERSAL_ROLES) {
			expect(ROLE_TOOL_CONSTRAINTS[role]).toBeDefined();
			expect(ROLE_TOOL_CONSTRAINTS[role].length).toBeGreaterThan(0);
		}
	});
});

// ---------------------------------------------------------------------------
// Tool Constraints
// ---------------------------------------------------------------------------

describe("ROLE_TOOL_CONSTRAINTS", () => {
	it("orchestrator cannot edit, bash, or web search", () => {
		const constraints = ROLE_TOOL_CONSTRAINTS.orchestrator;
		expect(constraints.find((c) => c.tool === "Edit")?.allowed).toBe(false);
		expect(constraints.find((c) => c.tool === "Bash")?.allowed).toBe(false);
		expect(constraints.find((c) => c.tool === "WebSearch")?.allowed).toBe(false);
	});

	it("implementer can edit source code and run shell", () => {
		const constraints = ROLE_TOOL_CONSTRAINTS.implementer;
		const edit = constraints.find((c) => c.tool === "Edit");
		expect(edit?.allowed).toBe(true);
		expect(edit?.artifactScope).toContain("source-code");
		expect(constraints.find((c) => c.tool === "Bash")?.allowed).toBe(true);
		expect(constraints.find((c) => c.tool === "WebSearch")?.allowed).toBe(false);
	});

	it("reviewer cannot edit but can run shell for checks", () => {
		const constraints = ROLE_TOOL_CONSTRAINTS.reviewer;
		expect(constraints.find((c) => c.tool === "Edit")?.allowed).toBe(false);
		const bash = constraints.find((c) => c.tool === "Bash");
		expect(bash?.allowed).toBe(true);
		expect(bash?.artifactScope).toContain("checks-only");
	});

	it("researcher can web search but not edit code or run shell", () => {
		const constraints = ROLE_TOOL_CONSTRAINTS.researcher;
		expect(constraints.find((c) => c.tool === "WebSearch")?.allowed).toBe(true);
		expect(constraints.find((c) => c.tool === "Bash")?.allowed).toBe(false);
	});

	it("writer can edit documentation only", () => {
		const constraints = ROLE_TOOL_CONSTRAINTS.writer;
		const edit = constraints.find((c) => c.tool === "Edit");
		expect(edit?.allowed).toBe(true);
		expect(edit?.artifactScope).toContain("documentation");
		expect(constraints.find((c) => c.tool === "Bash")?.allowed).toBe(false);
	});

	it("governance_steward can edit .orqa/ only", () => {
		const constraints = ROLE_TOOL_CONSTRAINTS.governance_steward;
		const edit = constraints.find((c) => c.tool === "Edit");
		expect(edit?.allowed).toBe(true);
		expect(edit?.artifactScope).toContain(".orqa/");
		expect(constraints.find((c) => c.tool === "Bash")?.allowed).toBe(false);
	});

	it("all roles can read, glob, and grep", () => {
		for (const role of UNIVERSAL_ROLES) {
			const constraints = ROLE_TOOL_CONSTRAINTS[role];
			expect(constraints.find((c) => c.tool === "Read")?.allowed).toBe(true);
			expect(constraints.find((c) => c.tool === "Glob")?.allowed).toBe(true);
			expect(constraints.find((c) => c.tool === "Grep")?.allowed).toBe(true);
		}
	});
});

// ---------------------------------------------------------------------------
// Findings Serialization / Parsing
// ---------------------------------------------------------------------------

describe("serializeFindings / parseFindingsHeader", () => {
	const doc: FindingsDocument = {
		header: {
			status: "complete",
			summary: "Implemented the agent spawner module with all 8 roles.",
			changedFiles: [
				"cli/src/lib/agent-spawner.ts",
				"cli/src/index.ts",
			],
			followUps: ["Add integration tests with real registry"],
		},
		body: "## Details\n\nFull implementation details here...",
	};

	it("round-trips through serialize and parse", () => {
		const serialized = serializeFindings(doc);
		const parsed = parseFindingsHeader(serialized);

		expect(parsed).not.toBeNull();
		expect(parsed!.status).toBe("complete");
		expect(parsed!.summary).toBe(doc.header.summary);
		expect(parsed!.changedFiles).toEqual(doc.header.changedFiles);
		expect(parsed!.followUps).toEqual(doc.header.followUps);
	});

	it("handles empty arrays", () => {
		const emptyDoc: FindingsDocument = {
			header: {
				status: "blocked",
				summary: "Could not proceed.",
				changedFiles: [],
				followUps: [],
			},
			body: "",
		};

		const serialized = serializeFindings(emptyDoc);
		const parsed = parseFindingsHeader(serialized);

		expect(parsed).not.toBeNull();
		expect(parsed!.status).toBe("blocked");
		expect(parsed!.changedFiles).toEqual([]);
		expect(parsed!.followUps).toEqual([]);
	});

	it("handles quotes in summary", () => {
		const quotedDoc: FindingsDocument = {
			header: {
				status: "partial",
				summary: 'Fixed the "broken" parser logic.',
				changedFiles: [],
				followUps: [],
			},
			body: "",
		};

		const serialized = serializeFindings(quotedDoc);
		const parsed = parseFindingsHeader(serialized);

		expect(parsed).not.toBeNull();
		expect(parsed!.summary).toBe('Fixed the "broken" parser logic.');
	});

	it("returns null for malformed content", () => {
		expect(parseFindingsHeader("no frontmatter here")).toBeNull();
		expect(parseFindingsHeader("---\nbad: yaml\n---")).toBeNull();
	});
});

// ---------------------------------------------------------------------------
// createAgentConfig
// ---------------------------------------------------------------------------

describe("createAgentConfig", () => {
	let projectDir: string;

	beforeEach(() => {
		projectDir = createTestProject(EMPTY_REGISTRY);
	});

	afterEach(() => {
		rmrf(projectDir);
	});

	it("creates a config with correct role and default model tier", () => {
		const config = createAgentConfig({
			role: "implementer",
			taskDescription: "Fix the parser bug",
			projectPath: projectDir,
		});

		expect(config.role).toBe("implementer");
		expect(config.modelTier).toBe("sonnet");
		expect(config.toolConstraints).toBe(ROLE_TOOL_CONSTRAINTS.implementer);
		expect(config.taskContext.description).toBe("Fix the parser bug");
	});

	it("upgrades implementer to opus for complex tasks", () => {
		const config = createAgentConfig({
			role: "implementer",
			taskDescription: "Multi-file refactoring",
			complexity: "complex",
			projectPath: projectDir,
		});

		expect(config.modelTier).toBe("opus");
	});

	it("generates a prompt via the pipeline", () => {
		const config = createAgentConfig({
			role: "reviewer",
			workflowStage: "review",
			taskDescription: "Review the PR",
			projectPath: projectDir,
		});

		// With an empty registry, the prompt still contains role + task context
		expect(config.prompt).toContain("reviewer");
		expect(config.promptResult).toBeDefined();
	});

	it("sets findings path when team context is provided", () => {
		const config = createAgentConfig({
			role: "writer",
			taskDescription: "Write the API docs",
			projectPath: projectDir,
			teamName: "docs-team",
			taskId: "42",
		});

		expect(config.findingsPath).toBe(".state/team/docs-team/task-42.md");
		expect(config.taskContext.teamName).toBe("docs-team");
		expect(config.taskContext.taskId).toBe("42");
	});

	it("leaves findings path null without team context", () => {
		const config = createAgentConfig({
			role: "researcher",
			taskDescription: "Research options",
			projectPath: projectDir,
		});

		expect(config.findingsPath).toBeNull();
	});

	it("passes acceptance criteria to the prompt pipeline", () => {
		const config = createAgentConfig({
			role: "implementer",
			taskDescription: "Add the new feature",
			acceptanceCriteria: ["Tests pass", "No regressions"],
			projectPath: projectDir,
		});

		expect(config.taskContext.acceptanceCriteria).toEqual([
			"Tests pass",
			"No regressions",
		]);
		// Task context should appear in the prompt
		expect(config.prompt).toContain("Add the new feature");
	});

	it("applies custom model tier overrides", () => {
		const config = createAgentConfig({
			role: "reviewer",
			taskDescription: "Critical security review",
			modelTierOverrides: { reviewer: "opus" },
			projectPath: projectDir,
		});

		expect(config.modelTier).toBe("opus");
	});
});

// ---------------------------------------------------------------------------
// Validation Helpers
// ---------------------------------------------------------------------------

describe("isValidRole", () => {
	it("returns true for valid roles", () => {
		expect(isValidRole("orchestrator")).toBe(true);
		expect(isValidRole("implementer")).toBe(true);
		expect(isValidRole("governance_steward")).toBe(true);
	});

	it("returns false for invalid roles", () => {
		expect(isValidRole("admin")).toBe(false);
		expect(isValidRole("")).toBe(false);
		expect(isValidRole("ORCHESTRATOR")).toBe(false);
	});
});

describe("modelTierLabel", () => {
	it("returns labels for each tier", () => {
		expect(modelTierLabel("opus")).toContain("Opus");
		expect(modelTierLabel("sonnet")).toContain("Sonnet");
		expect(modelTierLabel("haiku")).toContain("Haiku");
	});
});

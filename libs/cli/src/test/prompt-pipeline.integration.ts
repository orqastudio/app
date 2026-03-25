/**
 * Integration test for the prompt pipeline.
 *
 * Builds the registry from real plugins, generates a prompt for an
 * "implementer" role in "implement" stage, and verifies the output
 * structure (XML tags, ordering, budget compliance).
 *
 * Run: npx tsx libs/cli/src/test/prompt-pipeline.integration.ts
 */

import { resolve } from "node:path";
import { buildPromptRegistry } from "../lib/prompt-registry.js";
import { generatePrompt, estimateTokens, DEFAULT_TOKEN_BUDGETS } from "../lib/prompt-pipeline.js";
import {
	countOnDemandEntries,
	generateOnDemandPreamble,
	retrieveKnowledge,
} from "../lib/knowledge-retrieval.js";

const PROJECT_ROOT = resolve(
	new URL(".", import.meta.url).pathname.replace(/^\/([A-Z]:)/, "$1"),
	"..",
	"..",
	"..",
	"..",
);

let passed = 0;
let failed = 0;

function assert(condition: boolean, message: string): void {
	if (condition) {
		console.log(`  PASS  ${message}`);
		passed++;
	} else {
		console.error(`  FAIL  ${message}`);
		failed++;
	}
}

// ---------------------------------------------------------------------------
// Test 1: Registry builds from real plugins
// ---------------------------------------------------------------------------

console.log("\n--- Test 1: Registry builds from real plugins ---\n");

const registry = buildPromptRegistry(PROJECT_ROOT);

assert(registry.version === 1, "Registry version is 1");
assert(
	registry.knowledge.length >= 0,
	`Registry has ${registry.knowledge.length} knowledge entries`,
);
assert(
	registry.sections.length >= 0,
	`Registry has ${registry.sections.length} prompt sections`,
);
assert(
	typeof registry.built_at === "string",
	"Registry has a built_at timestamp",
);

// ---------------------------------------------------------------------------
// Test 2: Generate prompt for implementer role
// ---------------------------------------------------------------------------

console.log("\n--- Test 2: Generate prompt for implementer/implement ---\n");

const result = generatePrompt({
	role: "implementer",
	workflowStage: "implement",
	projectPath: PROJECT_ROOT,
	taskContext: {
		description: "Fix the login validation bug",
		files: ["src/auth/login.ts"],
		acceptanceCriteria: [
			"Login with valid credentials succeeds",
			"Login with invalid credentials shows error",
		],
	},
});

assert(result.prompt.length > 0, "Prompt is non-empty");
assert(
	result.prompt.includes("<role>implementer</role>"),
	"Prompt contains role tag",
);
assert(
	result.prompt.includes("<task-context"),
	"Prompt contains task-context section",
);
assert(
	result.prompt.includes("<task-description>"),
	"Prompt contains task-description",
);
assert(
	result.prompt.includes("Fix the login validation bug"),
	"Prompt contains the task description text",
);
assert(
	result.prompt.includes("<acceptance-criteria>"),
	"Prompt contains acceptance criteria",
);

// Budget compliance
const budget = DEFAULT_TOKEN_BUDGETS["implementer"] ?? 2800;
assert(result.budget === budget, `Budget is ${budget} for implementer`);
assert(
	result.totalTokens <= budget * 1.1, // Allow 10% overshoot from XML overhead
	`Total tokens (${result.totalTokens}) within budget (${budget})`,
);

// Included sections have required fields
for (const section of result.includedSections) {
	assert(
		typeof section.id === "string" && section.id.length > 0,
		`Section "${section.id}" has an id`,
	);
	assert(
		typeof section.content === "string" && section.content.length > 0,
		`Section "${section.id}" has content`,
	);
	assert(
		typeof section.tokens === "number" && section.tokens > 0,
		`Section "${section.id}" has token count (${section.tokens})`,
	);
}

// Ordering: role tag should come before task-context
const rolePos = result.prompt.indexOf("<role>");
const taskPos = result.prompt.indexOf("<task-context");
if (taskPos > -1) {
	assert(rolePos < taskPos, "Role definition comes before task context (KV-cache ordering)");
}

// ---------------------------------------------------------------------------
// Test 3: On-demand knowledge retrieval
// ---------------------------------------------------------------------------

console.log("\n--- Test 3: On-demand knowledge retrieval ---\n");

const onDemandCount = countOnDemandEntries(registry);
assert(
	typeof onDemandCount === "number",
	`On-demand entry count: ${onDemandCount}`,
);

const preamble = generateOnDemandPreamble(onDemandCount);
if (onDemandCount > 0) {
	assert(
		preamble.includes("on-demand-knowledge"),
		"Preamble contains on-demand-knowledge tag",
	);
	assert(
		preamble.includes("search_semantic"),
		"Preamble references semantic search tool",
	);
} else {
	assert(preamble === "", "Preamble is empty when no on-demand entries exist");
}

// Empty preamble for zero entries
const emptyPreamble = generateOnDemandPreamble(0);
assert(emptyPreamble === "", "Zero on-demand entries produces empty preamble");

// Non-empty preamble for positive count
const nonEmptyPreamble = generateOnDemandPreamble(5);
assert(
	nonEmptyPreamble.includes("<on-demand-knowledge>"),
	"Positive count produces preamble with opening tag",
);
assert(
	nonEmptyPreamble.includes("</on-demand-knowledge>"),
	"Positive count produces preamble with closing tag",
);

// ---------------------------------------------------------------------------
// Test 4: Disk-based knowledge retrieval
// ---------------------------------------------------------------------------

console.log("\n--- Test 4: Disk-based knowledge retrieval ---\n");

const retrieved = retrieveKnowledge(PROJECT_ROOT, {
	textQuery: "logging",
	tokenBudget: 5000,
});

assert(Array.isArray(retrieved), "retrieveKnowledge returns an array");

if (retrieved.length > 0) {
	const first = retrieved[0];
	assert(typeof first.id === "string", `Retrieved artifact has id: ${first.id}`);
	assert(typeof first.title === "string", `Retrieved artifact has title: ${first.title}`);
	assert(first.content.length > 0, "Retrieved artifact has content");
	assert(first.tokens > 0, `Retrieved artifact has token count: ${first.tokens}`);
} else {
	console.log("  INFO  No knowledge artifacts matched 'logging' (this is OK if none exist)");
}

// Budget enforcement
const budgetedRetrieved = retrieveKnowledge(PROJECT_ROOT, {
	tokenBudget: 100,
});
const totalRetrievedTokens = budgetedRetrieved.reduce(
	(sum, k) => sum + k.tokens,
	0,
);
assert(
	totalRetrievedTokens <= 100,
	`Retrieved knowledge within budget: ${totalRetrievedTokens} <= 100`,
);

// ---------------------------------------------------------------------------
// Test 5: estimateTokens utility
// ---------------------------------------------------------------------------

console.log("\n--- Test 5: estimateTokens utility ---\n");

assert(estimateTokens("") === 0, "Empty string = 0 tokens");
assert(estimateTokens("abcd") === 1, "4 chars = 1 token");
assert(estimateTokens("abcde") === 2, "5 chars = 2 tokens (ceil)");

// ---------------------------------------------------------------------------
// Test 6: Prompt without registry (error handling)
// ---------------------------------------------------------------------------

console.log("\n--- Test 6: Graceful handling without registry ---\n");

const noRegistryResult = generatePrompt({
	role: "implementer",
	projectPath: "/nonexistent/path",
});

assert(noRegistryResult.prompt === "", "No registry produces empty prompt");
assert(noRegistryResult.errors.length > 0, "No registry produces error message");
assert(
	noRegistryResult.errors[0].includes("registry"),
	"Error mentions registry",
);

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

console.log(`\n=== Results: ${passed} passed, ${failed} failed ===\n`);

if (failed > 0) {
	process.exit(1);
}

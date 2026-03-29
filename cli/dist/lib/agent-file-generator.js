/**
 * Agent file generator — produces .claude/agents/*.md files from the prompt
 * pipeline at install time.
 *
 * Each generated agent file contains:
 *   - YAML frontmatter: name, description (Claude Code agent fields)
 *   - Body: role definition, completion standard, tool constraints,
 *     knowledge references, critical rules
 *
 * The completion enforcement block is baked directly into the agent file
 * body so it is always present — hooks only inject dynamic context at runtime.
 *
 * Called from the install pipeline alongside workflow resolution and
 * prompt registry building.
 */
import * as fs from "node:fs";
import * as path from "node:path";
import { generatePrompt, } from "./prompt-pipeline.js";
import { ROLE_TOOL_CONSTRAINTS, } from "./agent-spawner.js";
/**
 * Roles to generate agent files for.
 *
 * We generate for the six "universal" roles that appear in .claude/agents/.
 * Orchestrator and designer are excluded — orchestrator is the CLAUDE.md itself,
 * designer is not a standard agent file.
 */
const AGENT_ROLES = {
    implementer: {
        fileName: "implementer",
        displayName: "implementer",
        description: "Implements code changes. Reads task, reads knowledge, writes code, runs checks. Does NOT self-certify — reviewer verifies.",
        roleSummary: "You are an Implementer. You write, edit, and test code.",
    },
    reviewer: {
        fileName: "reviewer",
        displayName: "reviewer",
        description: "Reviews code and artifacts for quality, correctness, and compliance. Produces PASS/FAIL verdicts. Does NOT fix issues — reports them.",
        roleSummary: "You are a Reviewer. You verify work against acceptance criteria.",
    },
    researcher: {
        fileName: "researcher",
        displayName: "researcher",
        description: "Investigates questions, gathers information, analyses patterns. Produces findings, not changes. Read-only access to codebase.",
        roleSummary: "You are a Researcher. You investigate and report findings.",
    },
    writer: {
        fileName: "writer",
        displayName: "writer",
        description: "Creates and edits documentation. Does NOT write source code or run shell commands.",
        roleSummary: "You are a Writer. You create and maintain documentation and knowledge artifacts.",
    },
    governance_steward: {
        fileName: "governance-steward",
        displayName: "governance-steward",
        description: "Creates and maintains .orqa/ governance artifacts — epics, tasks, rules, decisions, lessons. Ensures graph integrity.",
        roleSummary: "You are a Governance Steward. You maintain the artifact graph.",
    },
    planner: {
        fileName: "planner",
        displayName: "planner",
        description: "Designs approaches, maps dependencies, produces implementation plans. Read-only — does not implement or modify files.",
        roleSummary: "You are a Planner. You design approaches and map dependencies.",
    },
};
// ---------------------------------------------------------------------------
// Completion Enforcement (baked into agent files, not hook-injected)
// ---------------------------------------------------------------------------
const COMPLETION_ENFORCEMENT = `## Completion Standard (NON-NEGOTIABLE)

You MUST complete ALL acceptance criteria in your delegation prompt. You may NOT:
- Defer any acceptance criterion to a follow-up task
- Mark work as "done" with outstanding items listed as "future work"
- Skip an acceptance criterion because it seems hard or low-priority
- Silently omit criteria from your findings

If you cannot complete a criterion, you MUST report it as a FAILURE — not a deferral. The orchestrator will then decide whether to re-scope, re-assign, or escalate. Only the user can approve deferring work from the approved plan.`;
// ---------------------------------------------------------------------------
// Tool Constraint Formatting
// ---------------------------------------------------------------------------
/**
 * Format tool constraints into a human-readable markdown section.
 */
function formatToolConstraints(constraints) {
    const allowed = constraints.filter((c) => c.allowed);
    const denied = constraints.filter((c) => !c.allowed);
    const lines = ["## Tool Access", ""];
    if (allowed.length > 0) {
        for (const c of allowed) {
            const scope = c.artifactScope
                ? ` (${c.artifactScope.join(", ")})`
                : "";
            lines.push(`- ${c.tool}${scope}`);
        }
    }
    if (denied.length > 0) {
        lines.push("");
        const deniedNames = denied.map((c) => c.tool).join(", ");
        lines.push(`No access to: ${deniedNames}`);
    }
    return lines.join("\n");
}
// ---------------------------------------------------------------------------
// Output Template Formatting
// ---------------------------------------------------------------------------
/** Role-specific output format templates. */
const OUTPUT_TEMPLATES = {
    implementer: `## Output

Write findings to the path specified in your delegation prompt (\`.state/team/<name>/task-<id>.md\`):

\`\`\`
## What Was Done
[Files modified, changes made]

## What Was NOT Done
[Gaps, deferred items, or "Nothing — all complete"]

## Evidence
[Actual command output from checks]

## Follow-ups
[Anything the orchestrator needs to address]
\`\`\``,
    reviewer: `## Output

Write verdict to the findings path specified in your delegation prompt:

\`\`\`
## Verdict: PASS / FAIL

## Acceptance Criteria
- [x] Criterion 1 — PASS: [evidence]
- [ ] Criterion 2 — FAIL: [what's wrong]

## Issues Found
[Specific problems with file paths and line numbers]

## Lessons
[Any patterns worth logging as IMPL entries]
\`\`\``,
    researcher: `## Output

Write findings to the path specified in your delegation prompt:

\`\`\`
## Question
[What was investigated]

## Findings
[Structured findings with evidence and file references]

## Recommendations
[What should be done based on findings]

## Open Questions
[Anything that needs further investigation — with justification for why it couldn't be resolved]
\`\`\``,
    writer: `## Output

Write findings to the path specified in your delegation prompt:

\`\`\`
## What Was Written
[Files created/modified]

## Cross-References Updated
[Any links or references that were added/fixed]

## Accuracy Notes
[What was verified, what needs further review]
\`\`\``,
    governance_steward: `## Output

Write findings to the path specified in your delegation prompt:

\`\`\`
## What Was Created/Modified
[Artifact IDs and paths]

## Relationships Added
[Forward edges with semantics]

## Integrity Notes
[Any graph issues found or resolved]
\`\`\``,
    planner: `## Output

Write plan to the path specified in your delegation prompt:

\`\`\`
## Approach
[Proposed design with rationale]

## Dependencies
[What must exist before implementation]

## Risks
[What could go wrong]

## Task Breakdown
[Suggested tasks with explicit, verifiable acceptance criteria]
\`\`\``,
};
// ---------------------------------------------------------------------------
// Role-Specific Boundaries
// ---------------------------------------------------------------------------
/** Role-specific boundary sections. */
const ROLE_BOUNDARIES = {
    implementer: `## Boundaries

- You ONLY modify source code files (\`libs/\`, \`plugins/\`, \`ui/\`, \`backend/\`, \`sidecar/\`, \`tools/\`)
- You do NOT modify governance artifacts (\`.orqa/\`) — delegate to governance-steward
- You do NOT modify documentation — delegate to writer
- You do NOT review your own work — a reviewer verifies separately

## Before Starting

1. Read the task artifact (path provided in your delegation prompt)
2. Read the epic for broader context
3. Read any knowledge files specified in your delegation prompt
4. Understand acceptance criteria before writing any code

## Quality Checks

Before reporting completion, run relevant checks:
- Rust: \`cargo build\`, \`cargo clippy -- -D warnings\`, \`cargo test\`
- Frontend: \`npx svelte-check\`, \`npx eslint\`, \`npm run test\`
- Both: \`make check\` if touching both layers`,
    reviewer: `## Boundaries

- You do NOT edit source code or artifacts — you report findings
- You CAN run shell commands (build, test, lint, type-check)
- If you find issues, report them clearly. The implementer fixes them.

## Before Starting

1. Read the task artifact and its acceptance criteria
2. Read the epic for design context
3. Read the implementer's findings file

## Verification Process

For each acceptance criterion:
1. Check it independently with evidence
2. Mark PASS or FAIL with specific reasoning
3. Do not soften a FAIL — one unmet criterion = FAIL verdict
4. If the implementer deferred ANY acceptance criterion, that is an automatic FAIL
5. "Deferred to follow-up" is NOT an acceptable completion state — flag it explicitly`,
    researcher: `## Boundaries

- You do NOT modify any files — you produce findings only
- You CAN search the web for external references
- You CAN read any file in the codebase
- Your output goes in the findings file specified in your delegation prompt

## Before Starting

1. Read the research question/scope from your delegation prompt
2. Read any referenced artifacts or documentation
3. Plan your investigation before starting`,
    writer: `## Boundaries

- You ONLY modify documentation files (\`.orqa/documentation/\`, \`.orqa/process/knowledge/\`, plugin knowledge directories)
- You do NOT modify source code
- You do NOT run shell commands

## Before Starting

1. Read the writing task from your delegation prompt
2. Read existing documentation in the target area
3. Read any referenced artifacts for accuracy`,
    governance_steward: `## Boundaries

- You ONLY modify files under \`.orqa/\` and plugin governance content
- You do NOT modify source code
- You do NOT run shell commands
- You ensure relationship integrity — every forward edge has correct semantics

## Before Starting

1. Read the governance task from your delegation prompt
2. Read relevant schema files for the artifact type you're modifying
3. Check existing artifacts to avoid duplicates

## Key Rules

- Artifact IDs: PREFIX + first 8 hex of MD5(title)
- Relationships: backward-only storage (task->epic, not epic->task)
- Status values: must match the schema for that artifact type
- Narrow from/to constraints on relationships — specificity is the point`,
    planner: `## Boundaries

- You do NOT modify any files — you produce plans only
- You analyse the codebase, research, and artifacts to design approaches
- Your output goes in the findings file specified in your delegation prompt

## Before Starting

1. Read the planning question/scope from your delegation prompt
2. Read the relevant epic and research documents
3. Read existing architecture decisions`,
};
// ---------------------------------------------------------------------------
// Agent File Generation
// ---------------------------------------------------------------------------
/**
 * Generate a single agent markdown file.
 *
 * Combines:
 * 1. YAML frontmatter (name, description)
 * 2. Role heading and summary
 * 3. Role-specific boundaries and before-starting checklist
 * 4. Tool constraints from the agent spawner
 * 5. Completion enforcement (baked in, not hook-injected)
 * 6. Prompt pipeline content (knowledge references, critical rules)
 * 7. Output template
 */
function generateAgentFileContent(role, metadata, promptResult) {
    const parts = [];
    // YAML frontmatter
    parts.push("---");
    parts.push(`name: ${metadata.displayName}`);
    parts.push(`description: "${metadata.description}"`);
    parts.push("---");
    parts.push("");
    // Role heading
    parts.push(`# ${metadata.displayName.charAt(0).toUpperCase() + metadata.displayName.slice(1).replace(/-./g, (m) => " " + m[1].toUpperCase())}`);
    parts.push("");
    parts.push(metadata.roleSummary);
    parts.push("");
    // Role-specific boundaries
    const boundaries = ROLE_BOUNDARIES[role === "governance_steward" ? "governance_steward" : role];
    if (boundaries) {
        parts.push(boundaries);
        parts.push("");
    }
    // Tool constraints
    const constraints = ROLE_TOOL_CONSTRAINTS[role];
    if (constraints) {
        parts.push(formatToolConstraints(constraints));
        parts.push("");
    }
    // Completion enforcement (baked in)
    parts.push(COMPLETION_ENFORCEMENT);
    parts.push("");
    // Include prompt pipeline content (knowledge references, critical rules)
    // if the registry produced any sections
    if (promptResult.prompt && promptResult.includedSections.length > 0) {
        // Filter to knowledge and constraint sections — role-definition is already
        // in the boundaries section above, and task-context is dynamic
        const knowledgeSections = promptResult.includedSections.filter((s) => s.type === "knowledge" ||
            s.type === "constraint" ||
            s.type === "safety-rule");
        if (knowledgeSections.length > 0) {
            parts.push("## Knowledge References");
            parts.push("");
            parts.push("The following knowledge is available. Read the full files when working in these areas:");
            parts.push("");
            for (const section of knowledgeSections) {
                // Only include the ID and first line as a reference — not full content.
                // Full content is retrieved on-demand at runtime via the knowledge-injector hook.
                const titleMatch = section.content.match(/^title:\s*"?(.+?)"?\s*$/m);
                const firstLine = titleMatch ? titleMatch[1] : (section.content.split("\n").find((l) => l.trim() && !l.startsWith("---") && !l.startsWith("id:") && !l.startsWith("type:") && !l.startsWith("title:"))?.trim() ?? "");
                parts.push(`- **${section.id}** (${section.source}, ${section.priority}): ${firstLine.slice(0, 120)}`);
            }
            parts.push("");
        }
    }
    // Output template
    const outputTemplate = OUTPUT_TEMPLATES[role === "governance_steward" ? "governance_steward" : role];
    if (outputTemplate) {
        parts.push(outputTemplate);
        parts.push("");
    }
    return parts.join("\n");
}
// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------
/**
 * Generate .claude/agents/*.md files for all universal agent roles.
 *
 * For each role:
 *   1. Calls generatePrompt() to get the composed prompt from the pipeline
 *   2. Combines role metadata, tool constraints, completion enforcement,
 *      and pipeline content into a single agent markdown file
 *   3. Writes to .claude/agents/<role>.md
 *
 * @param projectPath - The project root directory
 * @returns Summary of generated files and any errors
 */
export function generateAgentFiles(projectPath) {
    const agentsDir = path.join(projectPath, ".claude", "agents");
    const generated = [];
    const errors = [];
    // Ensure .claude/agents/ directory exists
    if (!fs.existsSync(agentsDir)) {
        fs.mkdirSync(agentsDir, { recursive: true });
    }
    for (const [roleKey, metadata] of Object.entries(AGENT_ROLES)) {
        const role = roleKey;
        // Map the role to the prompt pipeline's role string
        const promptRole = role === "governance_steward" ? "governance-steward" : role;
        // Generate prompt via the five-stage pipeline (no task context — these
        // are static agent definitions, not runtime prompts)
        let promptResult;
        try {
            promptResult = generatePrompt({
                role: promptRole,
                projectPath,
            });
        }
        catch (err) {
            errors.push(`Failed to generate prompt for ${role}: ${err instanceof Error ? err.message : String(err)}`);
            // Generate the file anyway with empty pipeline content
            promptResult = {
                prompt: "",
                totalTokens: 0,
                budget: 0,
                includedSections: [],
                trimmedSections: [],
                errors: [],
            };
        }
        // Collect pipeline errors
        if (promptResult.errors.length > 0) {
            for (const e of promptResult.errors) {
                errors.push(`${role}: ${e}`);
            }
        }
        // Generate the file content
        const content = generateAgentFileContent(role, metadata, promptResult);
        // Write to disk
        const filePath = path.join(agentsDir, `${metadata.fileName}.md`);
        try {
            fs.writeFileSync(filePath, content, "utf-8");
            generated.push(filePath);
        }
        catch (err) {
            errors.push(`Failed to write ${filePath}: ${err instanceof Error ? err.message : String(err)}`);
        }
    }
    return { generated, errors };
}
/**
 * Run agent file generation and print results.
 *
 * Called from cmdPluginSync in install.ts and cmdRefresh in plugin.ts.
 */
export function runAgentFileGeneration(projectRoot) {
    const result = generateAgentFiles(projectRoot);
    if (result.errors.length > 0) {
        console.log("  Agent file generation warnings:");
        for (const err of result.errors) {
            console.log(`    - ${err}`);
        }
    }
    if (result.generated.length > 0) {
        console.log(`  Agent files: ${result.generated.length} agent(s) generated in .claude/agents/`);
    }
}
//# sourceMappingURL=agent-file-generator.js.map
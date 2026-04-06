/**
 * Agent spawner — creates agent configurations for ephemeral task-scoped workers.
 *
 * Implements the three-layer taxonomy from RES-d6e8ab11 section 4:
 *   Universal Role + Stage Context + Domain Knowledge = Effective Agent
 *
 * Each agent spawns fresh for a single task (ephemeral, task-scoped).
 * The spawner:
 *   1. Selects a model tier based on role and task complexity
 *   2. Attaches tool constraints for the role
 *   3. Sets a token budget for the agent
 *
 * Prompt generation belongs in the Rust engine (prompt crate). Callers
 * that need a generated prompt should call the daemon's /prompt/generate
 * endpoint and pass the result to the agent directly.
 */
/** All valid universal roles. */
export const UNIVERSAL_ROLES = [
    "orchestrator",
    "implementer",
    "reviewer",
    "researcher",
    "planner",
    "writer",
    "designer",
    "governance_steward",
];
/** Default model tier per role (from RES-d6e8ab11 section 4). */
export const DEFAULT_MODEL_TIERS = {
    orchestrator: "opus",
    planner: "opus",
    implementer: "sonnet",
    reviewer: "sonnet",
    researcher: "sonnet",
    writer: "sonnet",
    designer: "sonnet",
    governance_steward: "sonnet",
};
/**
 * Default token budgets per model tier.
 * Used when no explicit budget is provided to createAgentConfig.
 */
export const DEFAULT_TOKEN_BUDGETS = {
    opus: 4000,
    sonnet: 2500,
    haiku: 1500,
};
/**
 * Select the model tier for a given role and complexity.
 *
 * Rules:
 * - Implementer is upgraded to opus for complex tasks
 * - All other roles use their default tier regardless of complexity
 * - Custom overrides can be provided to change the defaults
 * @param role - The universal role being assigned.
 * @param complexity - Task complexity classification.
 * @param overrides - Optional per-role model tier overrides.
 * @returns The selected model tier.
 */
export function selectModelTier(role, complexity = "simple", overrides) {
    // Check overrides first
    if (overrides?.[role]) {
        return overrides[role];
    }
    // Upgrade implementer to opus for complex tasks
    if (role === "implementer" && complexity === "complex") {
        return "opus";
    }
    return DEFAULT_MODEL_TIERS[role];
}
/** Tool constraint sets per universal role. */
export const ROLE_TOOL_CONSTRAINTS = {
    orchestrator: [
        { tool: "Edit", allowed: false },
        { tool: "Bash", allowed: false },
        { tool: "WebSearch", allowed: false },
        { tool: "Read", allowed: true },
        { tool: "Glob", allowed: true },
        { tool: "Grep", allowed: true },
        { tool: "SendMessage", allowed: true },
        { tool: "TaskCreate", allowed: true },
        { tool: "TaskUpdate", allowed: true },
    ],
    implementer: [
        { tool: "Edit", allowed: true, artifactScope: ["source-code"] },
        { tool: "Write", allowed: true, artifactScope: ["source-code"] },
        { tool: "Bash", allowed: true },
        { tool: "Read", allowed: true },
        { tool: "Glob", allowed: true },
        { tool: "Grep", allowed: true },
        { tool: "WebSearch", allowed: false },
    ],
    reviewer: [
        { tool: "Edit", allowed: false },
        { tool: "Write", allowed: false },
        { tool: "Bash", allowed: true, artifactScope: ["checks-only"] },
        { tool: "Read", allowed: true },
        { tool: "Glob", allowed: true },
        { tool: "Grep", allowed: true },
        { tool: "WebSearch", allowed: false },
    ],
    researcher: [
        { tool: "Edit", allowed: false },
        { tool: "Write", allowed: true, artifactScope: ["research-artifact"] },
        { tool: "Bash", allowed: false },
        { tool: "Read", allowed: true },
        { tool: "Glob", allowed: true },
        { tool: "Grep", allowed: true },
        { tool: "WebSearch", allowed: true },
    ],
    planner: [
        { tool: "Edit", allowed: false },
        { tool: "Write", allowed: false },
        { tool: "Bash", allowed: false },
        { tool: "Read", allowed: true },
        { tool: "Glob", allowed: true },
        { tool: "Grep", allowed: true },
        { tool: "WebSearch", allowed: false },
    ],
    writer: [
        { tool: "Edit", allowed: true, artifactScope: ["documentation"] },
        { tool: "Write", allowed: true, artifactScope: ["documentation"] },
        { tool: "Bash", allowed: false },
        { tool: "Read", allowed: true },
        { tool: "Glob", allowed: true },
        { tool: "Grep", allowed: true },
        { tool: "WebSearch", allowed: false },
    ],
    designer: [
        { tool: "Edit", allowed: true, artifactScope: ["ui-component"] },
        { tool: "Write", allowed: true, artifactScope: ["ui-component"] },
        { tool: "Bash", allowed: false },
        { tool: "Read", allowed: true },
        { tool: "Glob", allowed: true },
        { tool: "Grep", allowed: true },
        { tool: "WebSearch", allowed: false },
    ],
    governance_steward: [
        { tool: "Edit", allowed: true, artifactScope: [".orqa/"] },
        { tool: "Write", allowed: true, artifactScope: [".orqa/"] },
        { tool: "Bash", allowed: false },
        { tool: "Read", allowed: true },
        { tool: "Glob", allowed: true },
        { tool: "Grep", allowed: true },
        { tool: "WebSearch", allowed: false },
    ],
};
/**
 * Serialize a findings document to markdown format.
 *
 * The header is in YAML frontmatter, the body follows as markdown.
 * This format lets the orchestrator read just the frontmatter (~200 tokens)
 * without loading the full body.
 * @param doc - The findings document to serialize.
 * @returns Markdown string with YAML frontmatter header and body.
 */
export function serializeFindings(doc) {
    const changedFilesYaml = doc.header.changedFiles.length > 0
        ? doc.header.changedFiles.map((f) => `  - "${f}"`).join("\n")
        : "  []";
    const followUpsYaml = doc.header.followUps.length > 0
        ? doc.header.followUps.map((f) => `  - "${f}"`).join("\n")
        : "  []";
    return [
        "---",
        `status: "${doc.header.status}"`,
        `summary: "${doc.header.summary.replace(/"/g, '\\"')}"`,
        "changed_files:",
        changedFilesYaml,
        "follow_ups:",
        followUpsYaml,
        "---",
        "",
        doc.body,
    ].join("\n");
}
/**
 * Parse the header from a findings markdown document.
 * Extracts only the YAML frontmatter section (~200 tokens).
 * @param content - Findings markdown string with YAML frontmatter.
 * @returns Parsed findings header, or null if the frontmatter is missing or invalid.
 */
export function parseFindingsHeader(content) {
    const fmMatch = content.match(/^---\n([\s\S]*?)\n---/);
    if (!fmMatch)
        return null;
    const fmText = fmMatch[1];
    const statusMatch = fmText.match(/^status:\s*"?(complete|blocked|partial)"?/m);
    const summaryMatch = fmText.match(/^summary:\s*"((?:[^"\\]|\\.)*)"/m);
    if (!statusMatch || !summaryMatch)
        return null;
    const changedFiles = [];
    const followUps = [];
    // Parse changed_files list
    const cfSection = fmText.match(/changed_files:\n((?:\s+-\s+"[^"]*"\n?)*)/);
    if (cfSection) {
        const items = cfSection[1].matchAll(/\s+-\s+"([^"]*)"/g);
        for (const item of items) {
            changedFiles.push(item[1]);
        }
    }
    // Parse follow_ups list
    const fuSection = fmText.match(/follow_ups:\n((?:\s+-\s+"[^"]*"\n?)*)/);
    if (fuSection) {
        const items = fuSection[1].matchAll(/\s+-\s+"([^"]*)"/g);
        for (const item of items) {
            followUps.push(item[1]);
        }
    }
    return {
        status: statusMatch[1],
        summary: summaryMatch[1].replace(/\\"/g, '"'),
        changedFiles,
        followUps,
    };
}
/**
 * Create an agent spawn configuration.
 *
 * Combines model tier selection, tool constraints, and task context into a
 * complete configuration that a connector or integration can use to spawn an
 * agent. Prompt generation is NOT done here — callers should call the daemon's
 * /prompt/generate endpoint to obtain the prompt and pass it to the agent.
 * @param params - Configuration parameters for the agent.
 * @returns Complete agent spawn configuration.
 */
export function createAgentConfig(params) {
    const { role, taskDescription, files, acceptanceCriteria, complexity = "simple", tokenBudget, modelTierOverrides, teamName, taskId, } = params;
    // Select model tier
    const modelTier = selectModelTier(role, complexity, modelTierOverrides);
    // Build task context
    const taskContext = {
        description: taskDescription,
        files,
        acceptanceCriteria,
        teamName,
        taskId,
    };
    // Get tool constraints for this role
    const toolConstraints = ROLE_TOOL_CONSTRAINTS[role];
    // Compute findings path if team context is provided
    const findingsPath = teamName && taskId ? `.state/team/${teamName}/task-${taskId}.md` : null;
    // Default token budget per role tier if not specified
    const effectiveBudget = tokenBudget ?? DEFAULT_TOKEN_BUDGETS[modelTier];
    return {
        role,
        modelTier,
        toolConstraints,
        tokenBudget: effectiveBudget,
        taskContext,
        findingsPath,
    };
}
// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------
/**
 * Validate that a string is a valid universal role.
 * @param role - The string to check.
 * @returns True if the string is a valid UniversalRole.
 */
export function isValidRole(role) {
    return UNIVERSAL_ROLES.includes(role);
}
/**
 * Get a human-readable label for a model tier.
 * @param tier - The model tier.
 * @returns Human-readable label string.
 */
export function modelTierLabel(tier) {
    switch (tier) {
        case "opus":
            return "Claude Opus (highest capability)";
        case "sonnet":
            return "Claude Sonnet (balanced)";
        case "haiku":
            return "Claude Haiku (fastest/cheapest)";
    }
}
//# sourceMappingURL=agent-spawner.js.map
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
/**
 * The 8 universal roles from the agent architecture.
 * These define behavioral boundaries and capability sets.
 */
export type UniversalRole = "orchestrator" | "implementer" | "reviewer" | "researcher" | "planner" | "writer" | "designer" | "governance_steward";
/** All valid universal roles. */
export declare const UNIVERSAL_ROLES: readonly UniversalRole[];
/** Available model tiers, ordered by capability/cost. */
export type ModelTier = "opus" | "sonnet" | "haiku";
/** Default model tier per role (from RES-d6e8ab11 section 4). */
export declare const DEFAULT_MODEL_TIERS: Record<UniversalRole, ModelTier>;
/**
 * Default token budgets per model tier.
 * Used when no explicit budget is provided to createAgentConfig.
 */
export declare const DEFAULT_TOKEN_BUDGETS: Record<ModelTier, number>;
/** Task complexity classification. */
export type TaskComplexity = "simple" | "complex";
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
export declare function selectModelTier(role: UniversalRole, complexity?: TaskComplexity, overrides?: Partial<Record<UniversalRole, ModelTier>>): ModelTier;
/**
 * A tool constraint declaration for an agent role.
 *
 * These are declarative — the connector/integration uses them to configure
 * actual tool permissions when spawning the agent.
 */
export interface ToolConstraint {
    /** Tool name or pattern (e.g. "Edit", "Bash", "WebSearch"). */
    tool: string;
    /** Whether this tool is allowed for the role. */
    allowed: boolean;
    /** Artifact types/scopes the tool can operate on (if allowed). */
    artifactScope?: string[];
}
/** Tool constraint sets per universal role. */
export declare const ROLE_TOOL_CONSTRAINTS: Record<UniversalRole, ToolConstraint[]>;
/**
 * Structured findings header (~200 tokens).
 * This is what the orchestrator reads — not the full body.
 */
export interface FindingsHeader {
    /** Completion status of the task. */
    status: "complete" | "blocked" | "partial";
    /** 1-2 sentence summary of what was done. */
    summary: string;
    /** Files that were created or modified. */
    changedFiles: string[];
    /** Follow-up items that need attention. */
    followUps: string[];
}
/** Full findings document written to .state/team/<team>/task-<id>.md. */
export interface FindingsDocument {
    /** Structured header for orchestrator consumption. */
    header: FindingsHeader;
    /** Full details — only read by reviewer agents, not orchestrator. */
    body: string;
}
/**
 * Serialize a findings document to markdown format.
 *
 * The header is in YAML frontmatter, the body follows as markdown.
 * This format lets the orchestrator read just the frontmatter (~200 tokens)
 * without loading the full body.
 * @param doc - The findings document to serialize.
 * @returns Markdown string with YAML frontmatter header and body.
 */
export declare function serializeFindings(doc: FindingsDocument): string;
/**
 * Parse the header from a findings markdown document.
 * Extracts only the YAML frontmatter section (~200 tokens).
 * @param content - Findings markdown string with YAML frontmatter.
 * @returns Parsed findings header, or null if the frontmatter is missing or invalid.
 */
export declare function parseFindingsHeader(content: string): FindingsHeader | null;
/** Task context passed to the agent spawner. */
export interface TaskContext {
    /** Task description. */
    description: string;
    /** Relevant file paths for knowledge injection. */
    files?: string[];
    /** Acceptance criteria the agent must meet. */
    acceptanceCriteria?: string[];
    /** Team name for findings output. */
    teamName?: string;
    /** Task ID for findings output. */
    taskId?: string;
}
/** Complete agent spawn configuration. */
export interface AgentSpawnConfig {
    /** Universal role assigned to this agent. */
    role: UniversalRole;
    /** Selected model tier. */
    modelTier: ModelTier;
    /** Tool constraints for this role. */
    toolConstraints: ToolConstraint[];
    /** Token budget for this agent's prompt. */
    tokenBudget: number;
    /** Task context that was used. */
    taskContext: TaskContext;
    /** Path where findings should be written. */
    findingsPath: string | null;
}
/** Parameters for creating an agent configuration. */
export interface CreateAgentParams {
    /** Universal role for the agent. */
    role: UniversalRole;
    /** Current workflow stage (e.g. "implement", "review"). */
    workflowStage?: string;
    /** Task description and context. */
    taskDescription: string;
    /** Relevant file paths. */
    files?: string[];
    /** Acceptance criteria. */
    acceptanceCriteria?: string[];
    /** Task complexity override. */
    complexity?: TaskComplexity;
    /** Project root directory. */
    projectPath: string;
    /** Custom token budget (overrides role default). */
    tokenBudget?: number;
    /** Custom model tier overrides. */
    modelTierOverrides?: Partial<Record<UniversalRole, ModelTier>>;
    /** Team name for findings path. */
    teamName?: string;
    /** Task ID for findings path. */
    taskId?: string;
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
export declare function createAgentConfig(params: CreateAgentParams): AgentSpawnConfig;
/**
 * Validate that a string is a valid universal role.
 * @param role - The string to check.
 * @returns True if the string is a valid UniversalRole.
 */
export declare function isValidRole(role: string): role is UniversalRole;
/**
 * Get a human-readable label for a model tier.
 * @param tier - The model tier.
 * @returns Human-readable label string.
 */
export declare function modelTierLabel(tier: ModelTier): string;
//# sourceMappingURL=agent-spawner.d.ts.map
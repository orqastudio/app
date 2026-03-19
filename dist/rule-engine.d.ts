/**
 * Rule Engine — evaluates OrqaStudio governance rules against tool calls.
 *
 * This is the TypeScript equivalent of the rule-engine.mjs hook script,
 * extracted for reuse by other consumers (e.g. the OrqaStudio plugin).
 *
 * Rules are markdown artifacts in .orqa/process/rules/ with YAML frontmatter
 * containing an `enforcement` array. Each enforcement entry specifies:
 * - event: "file" (Write/Edit) or "bash" (Bash)
 * - action: "block" (deny), "warn" (proceed + message), "inject" (load skills)
 * - pattern: regex to match against content
 * - paths: glob patterns to match against file paths
 * - message: human-readable enforcement message
 * - skills: skill names to inject (for "inject" action)
 */
export interface RuleEnforcementEntry {
    event: "file" | "bash";
    action: "block" | "warn" | "inject";
    pattern?: string;
    paths?: string[];
    message?: string;
    skills?: string[];
}
export interface ParsedRule {
    id: string;
    name: string;
    enforcement: RuleEnforcementEntry[];
    filePath: string;
}
export interface RuleEnforcementResult {
    blocked: boolean;
    blockMessage?: string;
    warnings: string[];
    injectedSkills: string[];
    matchedRules: string[];
}
export declare class RuleEngine {
    private projectRoot;
    private rules;
    constructor(projectRoot?: string);
    /**
     * Load all rules from the .orqa/process/rules/ directory.
     * Results are cached after first load.
     */
    loadRules(): ParsedRule[];
    /**
     * Evaluate rules against a tool call.
     *
     * @param event - "file" for Write/Edit, "bash" for Bash
     * @param content - file content or bash command
     * @param filePath - file path (for file events)
     */
    evaluate(event: "file" | "bash", content: string, filePath?: string): RuleEnforcementResult;
    /** Invalidate the cached rules (e.g. after rules are modified). */
    invalidateCache(): void;
    private parseEnforcement;
    private matchGlob;
}
//# sourceMappingURL=rule-engine.d.ts.map
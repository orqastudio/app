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
export declare function generateAgentFiles(projectPath: string): {
    generated: string[];
    errors: string[];
};
/**
 * Run agent file generation and print results.
 *
 * Called from cmdPluginSync in install.ts and cmdRefresh in plugin.ts.
 */
export declare function runAgentFileGeneration(projectRoot: string): void;
//# sourceMappingURL=agent-file-generator.d.ts.map
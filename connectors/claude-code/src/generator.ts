/**
 * Connector generator — transforms engine data into the Claude Code Plugin.
 *
 * This is the connector's primary job (architecture section 8.1). It reads
 * from the plugin registry, composed workflow state, active rules, and
 * installed plugin declarations, then writes:
 *
 *   - .claude/agents/*.md        — generated from role definitions + tool constraints
 *   - .claude/CLAUDE.md          — generated orchestrator context with active rules
 *   - plugin/hooks/hooks.json    — assembled from plugin hook declarations
 *   - .mcp.json                  — aggregated MCP server configs from plugins
 *   - .lsp.json                  — aggregated LSP server configs from plugins
 *
 * Dry-run mode (ORQA_DRY_RUN=true) writes all output to .state/dry-run/
 * instead of live project paths. This allows comparison against targets/
 * without modifying the working tree.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { listInstalledPlugins, readManifest } from "@orqastudio/cli";
import { generateAgentFiles } from "./lib/agent-file-generator.js";

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/** Output of a single generator run. */
export interface GenerateResult {
  /** Paths to generated agent .md files. */
  agents: string[];
  /** Path to the generated CLAUDE.md file. */
  claudeMd: string;
  /** Path to the generated hooks.json file. */
  hooksJson: string;
  /** Path to the generated .mcp.json file. */
  mcpJson: string;
  /** Path to the generated .lsp.json file. */
  lspJson: string;
  /** Non-fatal errors encountered during generation. */
  errors: string[];
}

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

/** A single Claude Code hook command entry. */
interface HookEntry {
  type: "command";
  command: string;
  timeout: number;
}

/** A matcher group within a Claude Code hook event. */
interface HookMatcherGroup {
  matcher: string;
  hooks: HookEntry[];
}

/** The top-level hooks.json structure. */
interface HooksJson {
  hooks: Record<string, HookMatcherGroup[]>;
}

// ---------------------------------------------------------------------------
// Dry-run path resolution
// ---------------------------------------------------------------------------

/**
 * Resolve the effective output root.
 *
 * In dry-run mode (ORQA_DRY_RUN=true), all writes go to
 * .state/dry-run/ so they can be compared against targets/
 * without modifying the live project.
 */
function resolveOutputRoot(projectRoot: string): string {
  if (process.env["ORQA_DRY_RUN"] === "true") {
    return path.join(projectRoot, ".state", "dry-run");
  }
  return projectRoot;
}

// ---------------------------------------------------------------------------
// Agent file generation
// ---------------------------------------------------------------------------

/**
 * Generate .claude/agents/*.md files for all universal roles.
 *
 * Delegates to agent-file-generator, which combines role metadata, tool
 * constraints, and completion enforcement for each role. Returns the list
 * of generated file paths.
 */
function generateAgents(
  projectRoot: string,
  outputRoot: string,
  errors: string[],
): string[] {
  // agent-file-generator writes to <projectPath>/.claude/agents/.
  // Pass outputRoot so dry-run redirects correctly.
  const { generated, errors: genErrors } = generateAgentFiles(outputRoot);
  errors.push(...genErrors);
  return generated;
}

// ---------------------------------------------------------------------------
// CLAUDE.md generation
// ---------------------------------------------------------------------------

/**
 * Read active rule titles from .orqa/learning/rules/*.md frontmatter.
 *
 * Used to surface active enforcement context in the generated CLAUDE.md.
 * Returns an empty array if the rules directory does not exist.
 */
function readActiveRuleTitles(projectRoot: string): string[] {
  const rulesDir = path.join(projectRoot, ".orqa", "learning", "rules");
  if (!fs.existsSync(rulesDir)) return [];

  const titles: string[] = [];
  for (const file of fs.readdirSync(rulesDir)) {
    if (!file.endsWith(".md")) continue;
    try {
      const content = fs.readFileSync(path.join(rulesDir, file), "utf-8");
      const match = content.match(/^---\n[\s\S]*?title:\s*(.+?)\n[\s\S]*?---/m);
      if (match?.[1]) {
        titles.push(match[1].trim());
      }
    } catch {
      // Skip unreadable files silently
    }
  }
  return titles;
}

/**
 * Read active workflow names from .orqa/workflows/*.resolved.yaml.
 *
 * Used to surface the workflow context in the generated CLAUDE.md.
 * Returns an empty array if the workflows directory does not exist.
 */
function readActiveWorkflowNames(projectRoot: string): string[] {
  const workflowsDir = path.join(projectRoot, ".orqa", "workflows");
  if (!fs.existsSync(workflowsDir)) return [];

  const names: string[] = [];
  for (const file of fs.readdirSync(workflowsDir)) {
    if (!file.endsWith(".resolved.yaml")) continue;
    names.push(file.replace(".resolved.yaml", ""));
  }
  return names;
}

/**
 * Build the canonical CLAUDE.md content for the orchestrator.
 *
 * Reads design principles from .claude/architecture/core.md if present and
 * appends any active workflows and rules as project-specific context.
 * Falls back to baked-in P1-P7 content if architecture docs are unavailable.
 */
function buildClaudemd(projectRoot: string, errors: string[]): string {
  const workflowNames = readActiveWorkflowNames(projectRoot);
  const ruleTitles = readActiveRuleTitles(projectRoot);

  // Attempt to extract the P1-P7 principles block from architecture docs.
  const architectureCoreDoc = path.join(
    projectRoot,
    ".claude",
    "architecture",
    "core.md",
  );
  let principlesBlock = "";
  if (fs.existsSync(architectureCoreDoc)) {
    try {
      const raw = fs.readFileSync(architectureCoreDoc, "utf-8");
      const match = raw.match(
        /##\s+2\.\s+Design Principles[\s\S]*?(?=\n---|\n##\s+[0-9])/,
      );
      if (match) {
        principlesBlock = match[0].trim();
      }
    } catch (err) {
      errors.push(
        `Could not read architecture/core.md: ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }

  // Canonical baked-in fallback — matches the target reference exactly.
  if (!principlesBlock) {
    principlesBlock = `## Design Principles

| # | Principle | Constraint |
|---|-----------|------------|
| P1 | Plugin-Composed Everything | No governance pattern hardcoded in engine. Plugins provide definitions, engine provides capabilities. |
| P2 | One Context Window Per Task | Each agent spawns fresh for a single task. No persistent agents, no accumulated context. |
| P3 | Generated, Not Loaded | System prompts are generated from plugin registries and workflow state, not loaded from disk. |
| P4 | Declarative Over Imperative | State machines, guards, and workflows are YAML declarations validated by JSON Schema. |
| P5 | Token Efficiency as Architecture | 2-4x overhead ratio. Per-agent prompts: 1,500-4,000 tokens. |
| P6 | Hub-Spoke Orchestration | Persistent orchestrator coordinates ephemeral task-scoped workers via structured summaries. Orchestrator delegates review to a Reviewer agent and reads the verdict -- it does not self-assess. |
| P7 | Resolved Workflow Is a File | After plugin composition, the resolved workflow is a deterministic YAML file on disk. |`;
  }

  const lines: string[] = [];
  lines.push(`# OrqaStudio`);
  lines.push(`
Plugin-composed governance platform for AI-assisted development. The engine provides core capabilities (graph, workflow, state machine, prompt pipeline, search, enforcement). Plugins provide definitions (methodology, workflows, artifact types, state machines, knowledge). Nothing is hardcoded in the engine.

${principlesBlock}

**Core product principles:** Accuracy over speed. Mechanical enforcement enables autonomy. The learning loop hardens the system.

## Team Discipline

### Always Use Agent Teams

ALL work MUST use team infrastructure:

1. \`TeamCreate\` -- create a named team before spawning agents
2. \`TaskCreate\` -- create tasks within the team for tracking
3. \`Agent\` -- spawn agents with \`run_in_background: true\` and \`team_name\` set
4. \`TaskUpdate\` -- agents mark tasks complete, orchestrator verifies via findings file
5. \`TeamDelete\` -- clean up after committing work

Never spawn a bare Agent without a team. Never run agents in the foreground.

### Hub-Spoke Coordination

- The orchestrator coordinates. It does NOT implement.
- Delegate ALL implementation to background agents via teams.
- Read structured summaries from findings files -- do not accumulate agent output in context.
- Stay available for conversation with the user.

### Agent Delegation

| Task Type | Agent Role | Model |
|-----------|-----------|-------|
| Code changes, tests, configs | Implementer | sonnet |
| Quality verification, AC checks | Reviewer | sonnet |
| Investigation, information gathering | Researcher | sonnet |
| Documentation creation/editing | Writer | sonnet |
| Approach design, dependency mapping | Planner | opus |
| UI/UX design, component structure | Designer | sonnet |
| \`.orqa/\` artifact maintenance | Governance Steward | sonnet |

### Role-Based Tool Constraints

| Role | Can Edit | Can Run Shell | Artifact Scope |
|------|----------|---------------|----------------|
| Orchestrator | No | No | Read-only, delegation |
| Implementer | Yes | Yes | Source code only |
| Reviewer | No | Yes (checks only) | Read-only, produces verdicts |
| Researcher | No | No | Creates research artifacts only |
| Writer | Yes | No | Documentation only |
| Planner | Yes | No | Delivery artifacts only |
| Designer | Yes | No | Design artifacts, component code |
| Governance Steward | Yes | No | \`.orqa/\` artifacts only |

### Mandatory Independent Review

Every completed task MUST be reviewed by a Reviewer agent before it is accepted. The orchestrator:

1. Spawns an Implementer (or Writer, Governance Steward, etc.) to do the work
2. Reads the implementer's findings file
3. Spawns a **separate Reviewer agent** to verify each acceptance criterion with evidence
4. Reads the reviewer's verdict
5. Only accepts the task if the Reviewer returns PASS on all criteria
6. If any criterion is FAIL: spawns a new Implementer to fix, then re-reviews

The orchestrator NEVER judges quality itself. It reads verdicts from Reviewers. Self-assessment is not review.

### No Autonomous Decisions

When an agent encounters ambiguity or uncertainty:

1. Check project documentation and knowledge artifacts via MCP search
2. If still unclear: raise to the orchestrator
3. If the orchestrator cannot resolve: escalate to the user for human review

Agents do NOT make autonomous design decisions. Unclear requirements are not permission to improvise -- they are signals to escalate.

### Discovery During Execution

When agents discover unexpected findings during work (undocumented dependencies, architectural inconsistencies, missing test coverage):

1. Agents report discoveries in their findings files under "Follow-ups"
2. The orchestrator compiles discoveries across agents
3. Discoveries are surfaced to the user as actionable items
4. Discoveries do NOT block current work unless they are genuine blockers

### Completion Gate

Before creating a new team:

- Read all findings files from the current team
- Verify EVERY acceptance criterion is DONE or FAILED -- not "deferred"
- If any criterion is FAILED: fix it now or get explicit user approval to defer
- Commit all changes
- \`TeamDelete\` the current team
- Only then proceed

## Zero Tech Debt

No legacy code, no blind copies, no "we'll fix this later":

- **Review every file** -- do not copy code without understanding it. Every file must justify its existence against the architecture.
- **Delete dead code** -- do not comment it out, do not wrap it in feature flags
- **No backwards compatibility shims** -- pre-release, breaking changes are expected
- **No accumulation** -- every file, function, and artifact must serve the current architecture

## Autonomous Execution

Work continuously without stopping. Do not ask "shall I proceed?" or "ready for the next task?". The user will interrupt if they want to steer. Silence means continue.

The ONLY acceptable reasons to pause:

1. A genuine blocker you cannot resolve
2. A destructive/irreversible action that could lose user work

## Key Design Decisions

- **Forward-only relationships** -- task stores \`delivers: epic\`, epic does NOT store \`delivered-by: task\`. Graph computes inverses.
- **Plugin-composed everything** -- no governance patterns hardcoded in core.
- **Daemon is business logic boundary** -- MCP/LSP are access protocols, not application boundaries.
- **30 relationship types** -- semantic precision creates structure. Each type is a unique bond. Narrow from/to constraints.
- **No backwards compatibility** -- pre-release, breaking changes expected, data migrated via \`orqa migrate\`.
- **.state/ not tmp/** -- session state and metrics are operational data, not disposable.

## Git Workflow

- Commit at natural boundaries (task completion, phase completion)
- No \`--no-verify\` -- fix errors, don't skip hooks
- After Rust changes: rebuild and restart daemon
- Agents do NOT commit -- the orchestrator commits after reviewing findings

## Architecture Knowledge

Architecture documentation and knowledge are available as project governance artifacts. Use MCP search to retrieve detailed architecture knowledge on demand. High-level principles are embedded in this prompt; implementation details are injected into task-specific agents by the prompt pipeline based on the task's scope and domain.

## Session Protocol

1. Read this file
2. Check \`.state/session-state.md\` for previous session context
3. Check \`git status\` and \`git stash list\`
4. Resume from where the previous session left off
5. Begin working immediately
6. Write session state to \`.state/session-state.md\` periodically`);

  // Append active project context when workflows or rules are present.
  if (workflowNames.length > 0 || ruleTitles.length > 0) {
    lines.push("\n## Active Project Context");

    if (workflowNames.length > 0) {
      lines.push(
        `\nActive workflows: ${workflowNames.map((n) => `\`${n}\``).join(", ")}`,
      );
    }

    if (ruleTitles.length > 0) {
      lines.push(`\nActive rules (${ruleTitles.length}):`);
      for (const title of ruleTitles) {
        lines.push(`- ${title}`);
      }
    }
  }

  return lines.join("\n") + "\n";
}

/**
 * Write the generated CLAUDE.md to .claude/CLAUDE.md (or dry-run equivalent).
 */
function generateClaudemd(
  projectRoot: string,
  outputRoot: string,
  errors: string[],
): string {
  const content = buildClaudemd(projectRoot, errors);
  const outputPath = path.join(outputRoot, ".claude", "CLAUDE.md");
  const outputDir = path.dirname(outputPath);

  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  fs.writeFileSync(outputPath, content, "utf-8");
  return outputPath;
}

// ---------------------------------------------------------------------------
// hooks.json generation
// ---------------------------------------------------------------------------

/**
 * Map from Claude Code hook event name to the script file and timeout.
 *
 * Scripts live at plugin/scripts/ relative to the plugin root and are
 * referenced via CLAUDE_PLUGIN_ROOT. This matches the target structure at
 * targets/claude-code-plugin/plugin/hooks/hooks.json.
 */
const HOOK_SCRIPT_MAP: Record<
  string,
  { matcher: string; script: string; timeout: number }
> = {
  PreToolUse: { matcher: "Write|Edit", script: "pre-tool-use.mjs", timeout: 10 },
  PostToolUse: { matcher: "Write|Edit", script: "post-tool-use.mjs", timeout: 10 },
  UserPromptSubmit: { matcher: "*", script: "user-prompt-submit.mjs", timeout: 10 },
  SessionStart: { matcher: "*", script: "session-start.mjs", timeout: 15 },
  Stop: { matcher: "*", script: "stop.mjs", timeout: 10 },
  PreCompact: { matcher: "*", script: "pre-compact.mjs", timeout: 10 },
  SubagentStop: { matcher: "*", script: "subagent-stop.mjs", timeout: 15 },
  TeammateIdle: { matcher: "*", script: "teammate-idle.mjs", timeout: 10 },
  TaskCompleted: { matcher: "*", script: "task-completed.mjs", timeout: 10 },
};

/**
 * Build the hooks.json content from plugin hook declarations and the
 * connector's script-to-event mapping.
 *
 * Scans all installed plugin manifests for `provides.hooks` entries. Any
 * declared event that has a known mapping in HOOK_SCRIPT_MAP is included in
 * the output. Declared events with no mapping produce a warning. The
 * connector's own hooks are always included in full, regardless of whether
 * they appear in any manifest, since it is the primary source.
 */
function buildHooksJson(projectRoot: string, errors: string[]): HooksJson {
  // Collect hook events declared across all installed plugin manifests.
  const declaredEvents = new Set<string>();
  for (const plugin of listInstalledPlugins(projectRoot)) {
    try {
      const manifest = readManifest(plugin.path);
      for (const hookDecl of manifest.provides?.hooks ?? []) {
        if ((hookDecl as { event?: string }).event) {
          declaredEvents.add((hookDecl as { event: string }).event);
        }
      }
    } catch {
      // Skip plugins with unreadable manifests
    }
  }

  // Build the hooks.json from HOOK_SCRIPT_MAP — the full connector event set.
  // Includes all mapped events regardless of manifest declarations so the
  // generated file is always complete.
  const hooksJson: HooksJson = { hooks: {} };

  for (const [event, { matcher, script, timeout }] of Object.entries(
    HOOK_SCRIPT_MAP,
  )) {
    hooksJson.hooks[event] = [
      {
        matcher,
        hooks: [
          {
            type: "command",
            command: `node \${CLAUDE_PLUGIN_ROOT}/scripts/${script}`,
            timeout,
          },
        ],
      },
    ];
    declaredEvents.delete(event);
  }

  // Warn about any declared events that have no wiring.
  for (const unhandled of declaredEvents) {
    errors.push(
      `Hook event "${unhandled}" declared in a plugin manifest has no Claude Code wiring — skipped`,
    );
  }

  return hooksJson;
}

/**
 * Write the generated hooks.json to plugin/hooks/hooks.json.
 *
 * The target path matches targets/claude-code-plugin/plugin/hooks/hooks.json.
 */
function generateHooksJson(
  projectRoot: string,
  outputRoot: string,
  errors: string[],
): string {
  const hooksJson = buildHooksJson(projectRoot, errors);
  const outputPath = path.join(outputRoot, "plugin", "hooks", "hooks.json");
  const outputDir = path.dirname(outputPath);

  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }

  fs.writeFileSync(
    outputPath,
    JSON.stringify(hooksJson, null, 2) + "\n",
    "utf-8",
  );
  return outputPath;
}

// ---------------------------------------------------------------------------
// .mcp.json and .lsp.json generation
// ---------------------------------------------------------------------------

/**
 * Aggregate mcpServers or lspServers configs from all installed plugin manifests.
 *
 * Both mcpServers and lspServers are untyped extras on PluginProvides — they
 * exist in manifests like the connector and typescript plugin but are not
 * declared in the PluginProvides TypeScript interface. A runtime cast is used.
 *
 * First-declaration-wins on key collision (alphabetical plugin order from
 * listInstalledPlugins).
 */
function aggregateServerConfigs(
  projectRoot: string,
  key: "mcpServers" | "lspServers",
  errors: string[],
): Record<string, unknown> {
  const merged: Record<string, unknown> = {};

  for (const plugin of listInstalledPlugins(projectRoot)) {
    try {
      const manifest = readManifest(plugin.path);
      // Runtime access — key is not in the PluginProvides interface.
      // Double-cast via unknown to satisfy TypeScript's overlap check.
      const provides = manifest.provides as unknown as Record<string, unknown>;
      const servers = provides[key];
      if (!servers || typeof servers !== "object" || Array.isArray(servers)) {
        continue;
      }
      for (const [name, config] of Object.entries(
        servers as Record<string, unknown>,
      )) {
        if (name in merged) {
          errors.push(
            `${key} server "${name}" declared by multiple plugins — keeping first declaration`,
          );
        } else {
          merged[name] = config;
        }
      }
    } catch {
      // Skip plugins with unreadable manifests
    }
  }

  return merged;
}

/**
 * Write .mcp.json from aggregated mcpServers declarations.
 *
 * Wraps server configs under "mcpServers" as required by Claude Code format.
 */
function generateMcpJson(
  projectRoot: string,
  outputRoot: string,
  errors: string[],
): string {
  const servers = aggregateServerConfigs(projectRoot, "mcpServers", errors);
  const outputPath = path.join(outputRoot, ".mcp.json");

  fs.writeFileSync(
    outputPath,
    JSON.stringify({ mcpServers: servers }, null, 2) + "\n",
    "utf-8",
  );

  if (Object.keys(servers).length === 0) {
    errors.push(".mcp.json: no mcpServers found in any installed plugin manifest");
  }

  return outputPath;
}

/**
 * Write .lsp.json from aggregated lspServers declarations.
 *
 * Top-level flat object (no wrapper key) matching the live .lsp.json format.
 */
function generateLspJson(
  projectRoot: string,
  outputRoot: string,
  errors: string[],
): string {
  const servers = aggregateServerConfigs(projectRoot, "lspServers", errors);
  const outputPath = path.join(outputRoot, ".lsp.json");

  fs.writeFileSync(outputPath, JSON.stringify(servers, null, 2) + "\n", "utf-8");

  if (Object.keys(servers).length === 0) {
    errors.push(".lsp.json: no lspServers found in any installed plugin manifest");
  }

  return outputPath;
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Generate all Claude Code Plugin artifacts for the project.
 *
 * Reads from:
 *   - Role metadata and tool constraints → .claude/agents/*.md
 *   - .claude/architecture/core.md + active workflows/rules → .claude/CLAUDE.md
 *   - Plugin manifests (provides.hooks) → plugin/hooks/hooks.json
 *   - Plugin manifests (provides.mcpServers) → .mcp.json
 *   - Plugin manifests (provides.lspServers) → .lsp.json
 *
 * When ORQA_DRY_RUN=true, all output goes to .state/dry-run/ instead of
 * live project paths, enabling comparison against targets/ without
 * modifying the working tree.
 */
export function generatePlugin(projectRoot: string): GenerateResult {
  const outputRoot = resolveOutputRoot(projectRoot);
  const errors: string[] = [];

  // Ensure the output root exists (important for dry-run mode).
  if (!fs.existsSync(outputRoot)) {
    fs.mkdirSync(outputRoot, { recursive: true });
  }

  const agents = generateAgents(projectRoot, outputRoot, errors);
  const claudeMd = generateClaudemd(projectRoot, outputRoot, errors);
  const hooksJson = generateHooksJson(projectRoot, outputRoot, errors);
  const mcpJson = generateMcpJson(projectRoot, outputRoot, errors);
  const lspJson = generateLspJson(projectRoot, outputRoot, errors);

  return { agents, claudeMd, hooksJson, mcpJson, lspJson, errors };
}

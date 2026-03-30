/**
 * Connector generator — transforms engine data into the Claude Code Plugin.
 *
 * This is the connector's primary job (architecture section 8.1). It reads
 * from the plugin registry, composed workflow state, active rules, and
 * installed plugin declarations, then writes:
 *
 *   - .claude/agents/*.md            — generated from role definitions + tool constraints
 *   - .claude/CLAUDE.md              — generated orchestrator context with active rules
 *   - .claude/settings.json          — permissions and env config for Claude Code
 *   - plugin/hooks/hooks.json        — assembled from plugin hook declarations
 *   - plugin/scripts/*.js             — thin daemon-wrapper hook scripts (compiled from src/scripts/)
 *   - plugin/skills/<name>/SKILL.md  — user-invocable governance commands
 *   - plugin/.claude-plugin/plugin.json — Claude Code plugin manifest
 *   - .mcp.json                      — aggregated MCP server configs from plugins
 *   - .lsp.json                      — aggregated LSP server configs from plugins
 *
 * Dry-run mode (ORQA_DRY_RUN=true) writes all output to .state/dry-run/
 * instead of live project paths. This allows comparison against targets/
 * without modifying the working tree.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { listInstalledPlugins, readManifest } from "@orqastudio/cli";
import { generateAgentFiles } from "./lib/agent-file-generator.js";
import { callDaemon } from "./hooks/shared.js";

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/** Output of a single generator run. */
export interface GenerateResult {
  /** Paths to generated agent .md files. */
  agents: string[];
  /** Path to the generated CLAUDE.md file. */
  claudeMd: string;
  /** Path to the generated .claude/settings.json file. */
  settingsJson: string;
  /** Path to the generated hooks.json file. */
  hooksJson: string;
  /** Paths to generated plugin/scripts/*.js files. */
  scripts: string[];
  /** Paths to generated plugin/skills/<name>/SKILL.md files. */
  skills: string[];
  /** Path to the generated plugin.json file. */
  pluginJson: string;
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
 * @param projectRoot - Absolute path to the project root directory.
 * @returns Absolute path to the directory where output files should be written.
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
 * @param projectRoot - Absolute path to the project root directory.
 * @param outputRoot - Absolute path to the output root (may differ in dry-run mode).
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Array of absolute paths to the generated agent markdown files.
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

/** Response shape for POST /context. */
interface ContextResponse {
  rule_titles: string[];
  workflow_names: string[];
}

/**
 * Fetch active rule titles and workflow names from the daemon POST /context endpoint.
 *
 * The daemon reads .orqa/learning/rules/*.md frontmatter and
 * .orqa/workflows/*.resolved.json filenames. Business logic for reading .orqa/
 * lives in the daemon, not the connector.
 *
 * Falls back to empty arrays when the daemon is unavailable — CLAUDE.md
 * generation continues without the active-context section.
 * @param projectRoot - Absolute path to the project root directory.
 * @returns Tuple of [ruleTitles, workflowNames].
 */
async function fetchActiveContext(
  projectRoot: string,
): Promise<{ ruleTitles: string[]; workflowNames: string[] }> {
  // In dry-run mode, skip active context so CLAUDE.md output matches the
  // baseline target (which represents a freshly installed project with no
  // project-specific rules yet).
  if (process.env["ORQA_DRY_RUN"] === "true") {
    return { ruleTitles: [], workflowNames: [] };
  }
  const result = await callDaemon<ContextResponse>("/context", {
    project_path: projectRoot,
  }).catch(() => null);
  return {
    ruleTitles: result?.rule_titles ?? [],
    workflowNames: result?.workflow_names ?? [],
  };
}

/**
 * Build the canonical CLAUDE.md content for the orchestrator.
 *
 * Reads design principles from .orqa/documentation/architecture/ DOCs if present
 * and appends any active workflows and rules as project-specific context.
 * Falls back to baked-in P1-P7 content if architecture docs are unavailable.
 * Active rules and workflows are fetched from the daemon POST /context endpoint.
 * @param projectRoot - Absolute path to the project root directory.
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns The complete CLAUDE.md content as a string.
 */
async function buildClaudemd(projectRoot: string, errors: string[]): Promise<string> {
  const { ruleTitles, workflowNames } = await fetchActiveContext(projectRoot);

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
 * @param projectRoot - Absolute path to the project root directory (used to fetch rules and workflows from daemon).
 * @param outputRoot - Absolute path to the output root (may differ in dry-run mode).
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Absolute path to the written CLAUDE.md file.
 */
async function generateClaudemd(
  projectRoot: string,
  outputRoot: string,
  errors: string[],
): Promise<string> {
  const content = await buildClaudemd(projectRoot, errors);
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
 * Resolve the path to the connector's hooks source directory.
 *
 * The connector plugin lives at connectors/claude-code/. Its source hooks
 * declaration is at hooks/hooks.json within that directory. This is the
 * authoritative hook configuration — generated from the plugin manifest's
 * provides.hooks declarations, not from a hardcoded static map.
 * @param projectRoot - Absolute path to the project root directory.
 * @returns Absolute path to the connector's hooks/hooks.json source file.
 */
function resolveConnectorHooksPath(projectRoot: string): string {
  return path.join(projectRoot, "connectors", "claude-code", "hooks", "hooks.json");
}

/**
 * Build the hooks.json content by reading the connector plugin's own
 * hooks/hooks.json source declaration.
 *
 * The connector's hooks/hooks.json is the authoritative plugin-manifest-driven
 * declaration of what Claude Code hook events this connector provides. It is
 * maintained alongside the connector source and updated when new hook scripts
 * are added. This replaces the previous static HOOK_SCRIPT_MAP approach.
 *
 * Any plugin that declares hook events in its manifest but has no corresponding
 * entry in the connector hooks source produces a warning — those events are
 * acknowledged as needing wiring in a future connector update.
 * @param projectRoot - Absolute path to the project root directory.
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns The fully assembled hooks.json object ready for serialization.
 */
function buildHooksJson(projectRoot: string, errors: string[]): HooksJson {
  // Read the connector's own hooks.json — the authoritative hook declaration.
  const hooksSourcePath = resolveConnectorHooksPath(projectRoot);
  let hooksJson: HooksJson;

  try {
    const raw = fs.readFileSync(hooksSourcePath, "utf-8");
    hooksJson = JSON.parse(raw) as HooksJson;
  } catch (err) {
    errors.push(
      `Could not read connector hooks source at ${hooksSourcePath}: ${err instanceof Error ? err.message : String(err)}`,
    );
    hooksJson = { hooks: {} };
  }

  // Collect hook events declared across all installed plugin manifests and
  // warn about any that have no corresponding entry in the connector hooks source.
  const wiredEvents = new Set(Object.keys(hooksJson.hooks));

  for (const plugin of listInstalledPlugins(projectRoot)) {
    try {
      const manifest = readManifest(plugin.path);
      for (const hookDecl of manifest.provides?.hooks ?? []) {
        const event = (hookDecl as { event?: string }).event;
        if (event && !wiredEvents.has(event)) {
          errors.push(
            `Hook event "${event}" declared in ${manifest.name} has no Claude Code wiring — update connector hooks/hooks.json`,
          );
        }
      }
    } catch {
      // Skip plugins with unreadable manifests
    }
  }

  return hooksJson;
}

/**
 * Write the generated hooks.json to plugin/hooks/hooks.json.
 *
 * The target path matches targets/claude-code-plugin/plugin/hooks/hooks.json.
 * @param projectRoot - Absolute path to the project root directory.
 * @param outputRoot - Absolute path to the output root (may differ in dry-run mode).
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Absolute path to the written hooks.json file.
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
// scripts/ generation
// ---------------------------------------------------------------------------

/**
 * Resolve the connector's dist/scripts/ directory containing compiled hook scripts.
 *
 * Hook scripts are compiled TypeScript modules under src/scripts/ and output to
 * dist/scripts/ by the connector's tsc build. They are thin daemon wrappers
 * with no business logic and are copied verbatim to plugin/scripts/ in the output.
 * @param projectRoot - Absolute path to the project root directory.
 * @returns Absolute path to the connector's dist/scripts/ directory.
 */
function resolveConnectorScriptsDir(projectRoot: string): string {
  return path.join(projectRoot, "connectors", "claude-code", "dist", "scripts");
}

/**
 * Copy compiled hook scripts from dist/scripts/ to plugin/scripts/ in the output.
 *
 * Only the runnable .js files are copied — declaration and map files are omitted.
 * Each script corresponds to a Claude Code hook event and is a thin daemon wrapper.
 * @param projectRoot - Absolute path to the project root directory.
 * @param outputRoot - Absolute path to the output root (may differ in dry-run mode).
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Array of absolute paths to the written script files.
 */
function generateScripts(
  projectRoot: string,
  outputRoot: string,
  errors: string[],
): string[] {
  const scriptsSourceDir = resolveConnectorScriptsDir(projectRoot);
  const scriptsOutputDir = path.join(outputRoot, "plugin", "scripts");
  const generated: string[] = [];

  if (!fs.existsSync(scriptsSourceDir)) {
    errors.push(`Scripts source directory not found: ${scriptsSourceDir}`);
    return generated;
  }

  if (!fs.existsSync(scriptsOutputDir)) {
    fs.mkdirSync(scriptsOutputDir, { recursive: true });
  }

  // Copy only .js files (not .d.ts or .js.map) from dist/scripts/ to output.
  const entries = fs.readdirSync(scriptsSourceDir);
  for (const entry of entries) {
    if (!entry.endsWith(".js") || entry.endsWith(".d.ts")) continue;
    const srcPath = path.join(scriptsSourceDir, entry);
    const dstPath = path.join(scriptsOutputDir, entry);
    try {
      fs.copyFileSync(srcPath, dstPath);
      generated.push(dstPath);
    } catch (err) {
      errors.push(
        `Failed to copy script ${entry}: ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }

  return generated;
}

// ---------------------------------------------------------------------------
// skills/ generation
// ---------------------------------------------------------------------------

/**
 * Resolve the connector's skills/ source directory.
 *
 * Skills are user-invocable commands defined as SKILL.md files in named
 * subdirectories. The connector owns the orqa skill set.
 * @param projectRoot - Absolute path to the project root directory.
 * @returns Absolute path to the connector's skills/ source directory.
 */
function resolveConnectorSkillsDir(projectRoot: string): string {
  return path.join(projectRoot, "connectors", "claude-code", "skills");
}

/**
 * Copy user-invocable skill SKILL.md files from the connector's skills/
 * source directory to plugin/skills/ in the output.
 *
 * Only skills with user-invocable: true frontmatter are included. Each
 * skill is a directory containing a single SKILL.md file.
 * @param projectRoot - Absolute path to the project root directory.
 * @param outputRoot - Absolute path to the output root (may differ in dry-run mode).
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Array of absolute paths to the written SKILL.md files.
 */
function generateSkills(
  projectRoot: string,
  outputRoot: string,
  errors: string[],
): string[] {
  const skillsSourceDir = resolveConnectorSkillsDir(projectRoot);
  const skillsOutputDir = path.join(outputRoot, "plugin", "skills");
  const generated: string[] = [];

  if (!fs.existsSync(skillsSourceDir)) {
    errors.push(`Skills source directory not found: ${skillsSourceDir}`);
    return generated;
  }

  const entries = fs.readdirSync(skillsSourceDir, { withFileTypes: true });
  for (const entry of entries) {
    if (!entry.isDirectory()) continue;
    const skillMdPath = path.join(skillsSourceDir, entry.name, "SKILL.md");
    if (!fs.existsSync(skillMdPath)) continue;

    // Only include user-invocable skills (frontmatter check).
    const content = fs.readFileSync(skillMdPath, "utf-8");
    if (!content.includes("user-invocable: true")) continue;

    const outputSkillDir = path.join(skillsOutputDir, entry.name);
    if (!fs.existsSync(outputSkillDir)) {
      fs.mkdirSync(outputSkillDir, { recursive: true });
    }

    const dstPath = path.join(outputSkillDir, "SKILL.md");
    try {
      fs.copyFileSync(skillMdPath, dstPath);
      generated.push(dstPath);
    } catch (err) {
      errors.push(
        `Failed to copy skill ${entry.name}: ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }

  return generated;
}

// ---------------------------------------------------------------------------
// plugin.json generation
// ---------------------------------------------------------------------------

/**
 * Generate the Claude Code plugin manifest at plugin/.claude-plugin/plugin.json.
 *
 * The manifest identifies the plugin to Claude Code. It includes the plugin
 * name, description, version, and author. Commands, hooks, skills, and
 * resources are discovered at runtime from the plugin directory structure.
 * @param outputRoot - Absolute path to the output root (may differ in dry-run mode).
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Absolute path to the written plugin.json file.
 */
function generatePluginJson(
  outputRoot: string,
  errors: string[],
): string {
  const pluginManifest = {
    name: "orqastudio",
    description: "OrqaStudio governance integration for Claude Code",
    version: "1.0.0",
    author: { name: "OrqaStudio" },
  };

  const outputPath = path.join(outputRoot, "plugin", ".claude-plugin", "plugin.json");
  const outputDir = path.dirname(outputPath);

  try {
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }
    fs.writeFileSync(
      outputPath,
      JSON.stringify(pluginManifest, null, 2) + "\n",
      "utf-8",
    );
  } catch (err) {
    errors.push(
      `Failed to write plugin.json: ${err instanceof Error ? err.message : String(err)}`,
    );
  }

  return outputPath;
}

// ---------------------------------------------------------------------------
// .claude/settings.json generation
// ---------------------------------------------------------------------------

/**
 * Build the .claude/settings.json content.
 *
 * Settings configure Claude Code environment variables, file access
 * permissions (allow/deny lists), and hook commands for artifact validation.
 * The deny list protects read-only files (targets/, ARCHITECTURE.md, settings)
 * and sensitive paths (.env, secrets, credentials).
 * @returns The settings object ready for JSON serialization.
 */
function buildSettingsJson(): Record<string, unknown> {
  return {
    $schema: "https://json.schemastore.org/claude-code-settings.json",
    env: {
      CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "70",
      CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS: "1",
      ORQA_DRY_RUN: "false",
    },
    permissions: {
      allow: [
        "Bash(./*)",
        "Read(./**)",
        "Write(./.state/team/**)",
      ],
      deny: [
        "Read(./.env)",
        "Read(./.env.*)",
        "Read(./secrets/**)",
        "Read(//.aws/**)",
        "Read(//.ssh/**)",
        "Edit(./targets/**)",
        "Write(./targets/**)",
        "Edit(./ARCHITECTURE.md)",
        "Write(./ARCHITECTURE.md)",
        "Edit(./.claude/settings.json)",
        "Edit(./.claude/settings.local.json)",
        "Write(./.claude/settings.json)",
        "Write(./.claude/settings.local.json)",
        "Edit(./.claude/CLAUDE.md)",
        "Write(./.claude/CLAUDE.md)",
        "Edit(./.orqa/manifest.json)",
        "Edit(./.orqa/project.json)",
        "Edit(./scripts/validate-artifacts.mjs)",
        "Write(./scripts/validate-artifacts.mjs)",
        "Bash(rm -rf *)",
        "Bash(git push --force *)",
        "Bash(git reset --hard *)",
        "Bash(curl *)",
        "Bash(wget *)",
        "Bash(cp * targets/*)",
        "Bash(mv * targets/*)",
        "Bash(tee * targets/*)",
        "Bash(sed -i * targets/*)",
        "Bash(cp * ARCHITECTURE.md)",
        "Bash(mv * ARCHITECTURE.md)",
        "Bash(cp * .claude/settings*)",
        "Bash(mv * .claude/settings*)",
      ],
    },
    hooks: {
      PreToolUse: [
        {
          matcher: "Write|Edit",
          hooks: [
            {
              type: "command",
              command: "node \"$CLAUDE_PROJECT_DIR/scripts/validate-artifacts.mjs\" --hook --file \"$(echo $TOOL_INPUT | node -e \"process.stdin.on('data',d=>{const j=JSON.parse(d);console.log(j.file_path||j.content&&'skip'||'skip')})\"  2>/dev/null || echo skip)\"",
              timeout: 10,
              statusMessage: "Validating artifact schema",
            },
          ],
        },
      ],
      PostToolUse: [
        {
          matcher: "Write|Edit",
          hooks: [
            {
              type: "command",
              command: "bash -c 'FILE=$(echo \"$TOOL_RESULT\" | node -e \"process.stdin.on(\\\"data\\\",d=>{try{const j=JSON.parse(d);console.log(j.filePath||\\\"\\\")}catch{console.log(\\\"\\\")}}\" 2>/dev/null); if [[ \"$FILE\" == *.md ]] && [[ \"$FILE\" == */.orqa/* ]]; then npx markdownlint-cli2 \"$FILE\" 2>&1 || true; fi'",
              timeout: 10,
              statusMessage: "Checking markdown lint",
            },
          ],
        },
      ],
    },
  };
}

/**
 * Write the generated settings.json to .claude/settings.json.
 *
 * Configures Claude Code with OrqaStudio-specific permissions and hooks.
 * The settings file is written to the output root's .claude/ directory.
 * @param outputRoot - Absolute path to the output root (may differ in dry-run mode).
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Absolute path to the written settings.json file.
 */
function generateSettingsJson(
  outputRoot: string,
  errors: string[],
): string {
  const settings = buildSettingsJson();
  const outputPath = path.join(outputRoot, ".claude", "settings.json");
  const outputDir = path.dirname(outputPath);

  try {
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }
    fs.writeFileSync(
      outputPath,
      JSON.stringify(settings, null, 2) + "\n",
      "utf-8",
    );
  } catch (err) {
    errors.push(
      `Failed to write settings.json: ${err instanceof Error ? err.message : String(err)}`,
    );
  }

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
 * @param projectRoot - Absolute path to the project root directory.
 * @param key - Which server config key to aggregate: "mcpServers" or "lspServers".
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Merged map of server name to server config object.
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
 * @param projectRoot - Absolute path to the project root directory.
 * @param outputRoot - Absolute path to the output root (may differ in dry-run mode).
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Absolute path to the written .mcp.json file.
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
 * @param projectRoot - Absolute path to the project root directory.
 * @param outputRoot - Absolute path to the output root (may differ in dry-run mode).
 * @param errors - Mutable array to which non-fatal error messages are appended.
 * @returns Absolute path to the written .lsp.json file.
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
 *   - Daemon /context + architecture DOCs → .claude/CLAUDE.md
 *   - Static settings config → .claude/settings.json
 *   - Plugin manifests (provides.hooks) → plugin/hooks/hooks.json
 *   - Connector dist/scripts/*.js → plugin/scripts/*.js
 *   - Connector skills/<name>/SKILL.md → plugin/skills/<name>/SKILL.md
 *   - Static plugin manifest → plugin/.claude-plugin/plugin.json
 *   - Plugin manifests (provides.mcpServers) → .mcp.json
 *   - Plugin manifests (provides.lspServers) → .lsp.json
 *
 * When ORQA_DRY_RUN=true, all output goes to .state/dry-run/ instead of
 * live project paths, enabling comparison against targets/ without
 * modifying the working tree.
 * @param projectRoot - Absolute path to the project root directory.
 * @returns Generation result containing paths to all generated files and any non-fatal errors.
 */
export async function generatePlugin(projectRoot: string): Promise<GenerateResult> {
  const outputRoot = resolveOutputRoot(projectRoot);
  const errors: string[] = [];

  // Ensure the output root exists (important for dry-run mode).
  if (!fs.existsSync(outputRoot)) {
    fs.mkdirSync(outputRoot, { recursive: true });
  }

  const agents = generateAgents(projectRoot, outputRoot, errors);
  const claudeMd = await generateClaudemd(projectRoot, outputRoot, errors);
  const settingsJson = generateSettingsJson(outputRoot, errors);
  const hooksJson = generateHooksJson(projectRoot, outputRoot, errors);
  const scripts = generateScripts(projectRoot, outputRoot, errors);
  const skills = generateSkills(projectRoot, outputRoot, errors);
  const pluginJson = generatePluginJson(outputRoot, errors);
  const mcpJson = generateMcpJson(projectRoot, outputRoot, errors);
  const lspJson = generateLspJson(projectRoot, outputRoot, errors);

  return { agents, claudeMd, settingsJson, hooksJson, scripts, skills, pluginJson, mcpJson, lspJson, errors };
}

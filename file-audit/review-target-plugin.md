# Review: Target Claude Code Plugin (.claude-plugin/)

## Verdict: FAIL

Two acceptance criteria fail. The file structure nests skills/, hooks/, and scripts/ inside `.claude-plugin/`, which violates the Claude Code plugin spec. Additionally, scripts lack per-function description comments as required by the zero tech debt standard.

---

## Acceptance Criteria

### 1. plugin.json is valid and minimal

**PASS**

File: `targets/claude-code-plugin/.claude-plugin/plugin.json`

```json
{
  "name": "orqastudio",
  "description": "OrqaStudio governance integration for Claude Code",
  "version": "1.0.0",
  "author": { "name": "OrqaStudio" }
}
```

- Valid JSON: confirmed via `JSON.parse()`
- Contains only `name`, `description`, `version`, `author` -- all standard fields per the plugin spec
- No extraneous fields, no component path overrides, no `userConfig`, no `channels`
- `name` is kebab-case-compatible (lowercase, no spaces)
- Minimal and correct

---

### 2. All 4 skills have correct SKILL.md format (frontmatter + body)

**PASS**

All four skills follow the correct format:

| Skill | `name` | `description` | `user-invocable` | Body present |
|-------|--------|---------------|-------------------|--------------|
| `skills/orqa/SKILL.md` | `orqa` | Yes | `true` | Yes -- command table, usage |
| `skills/orqa-save/SKILL.md` | `orqa-save` | Yes | `true` | Yes -- usage, behavior description |
| `skills/orqa-create/SKILL.md` | `orqa-create` | Yes | `true` | Yes -- usage, artifact types table |
| `skills/orqa-validate/SKILL.md` | `orqa-validate` | Yes | `true` | Yes -- usage, validation checks list |

Each has:
- YAML frontmatter delimited by `---`
- `name` field (kebab-case)
- `description` field (quoted string)
- `user-invocable: true`
- Markdown body with usage instructions

No issues found with skill format.

---

### 3. hooks.json covers all required events, uses correct format (wrapped in {"hooks":{}})

**PASS**

File: `targets/claude-code-plugin/.claude-plugin/hooks/hooks.json`

- Valid JSON: confirmed via `JSON.parse()`
- Top-level structure: `{"hooks": { ... }}` -- correct wrapper format per spec
- Events covered:

| Event | Matcher(s) | Script | Present |
|-------|-----------|--------|---------|
| `PreToolUse` | `Write\|Edit`, `Bash` | `pre-tool-use.mjs` | Yes |
| `PostToolUse` | `Write\|Edit`, `TaskUpdate` | `post-tool-use.mjs` | Yes |
| `UserPromptSubmit` | `*` | `user-prompt-submit.mjs` | Yes |
| `SessionStart` | `*` | `session-start.mjs` | Yes |
| `Stop` | `*` | `stop.mjs` | Yes |
| `PreCompact` | `*` | `pre-compact.mjs` | Yes |
| `SubagentStop` | `*` | `subagent-stop.mjs` | Yes |
| `TeammateIdle` | `*` | `teammate-idle.mjs` | Yes |
| `TaskCompleted` | `*` | `task-completed.mjs` | Yes |

All 9 events from ARCHITECTURE.md Appendix A.1 (lines 1069-1077) are covered. Each has the correct handler structure with `matcher`, `hooks` array, `type: "command"`, and `command` field.

**Note (non-blocking):** The spec research (line 529) notes that `UserPromptSubmit`, `Stop`, `TeammateIdle`, `TaskCompleted` have no official matcher support. Using `"*"` may be harmless (treated as always-fire) but is technically undocumented. This is a minor conformance concern, not a blocker.

---

### 4. All scripts are thin daemon wrappers -- zero business logic, just stdin->CLI/MCP->stdout

**PASS**

All 9 scripts follow the same pattern:
1. Read stdin JSON
2. Build a context object from input fields
3. POST to `${DAEMON_URL}/hook` or `/health`
4. Apply the daemon's response (block/warn/pass)
5. Exit with appropriate code

No script contains business logic. All decisions are delegated to the daemon. The `session-start.mjs` has slightly more logic (session guard file, session state loading, daemon health gate), but this is infrastructure plumbing, not business logic -- the actual governance decisions still come from the daemon.

Each script has a purpose comment at the top (e.g., `// PreToolUse hook -- Write|Edit|Bash`) and a description of what the daemon handles.

**Minor concern:** `session-start.mjs` writes a `.session-started` guard file and reads `session-state.md` directly from the filesystem. This is session plumbing, not governance logic, so it passes. But it's the thickest of the thin wrappers.

---

### 5. Timeout values are in seconds (not milliseconds)

**PASS**

All timeout values in `hooks.json`:
- `PreToolUse`: 10 (seconds)
- `PostToolUse`: 10 (seconds)
- `UserPromptSubmit`: 10 (seconds)
- `SessionStart`: 15 (seconds)
- `Stop`: 10 (seconds)
- `PreCompact`: 10 (seconds)
- `SubagentStop`: 15 (seconds)
- `TeammateIdle`: 10 (seconds)
- `TaskCompleted`: 10 (seconds)

All values are in the 10-15 range, consistent with seconds. The spec research (line 530) flagged a previous issue where values like 5000-15000 were used (milliseconds). That has been corrected.

Internal `AbortSignal.timeout()` values in scripts use milliseconds (2000, 8000, 10000, 12000) which is correct for the Node.js API. These are internal fetch timeouts, not hook timeouts.

---

### 6. Script paths use ${CLAUDE_PLUGIN_ROOT}

**PASS**

Every `command` field in `hooks.json` uses `${CLAUDE_PLUGIN_ROOT}/scripts/<name>.mjs`:

```
"command": "node ${CLAUDE_PLUGIN_ROOT}/scripts/pre-tool-use.mjs"
"command": "node ${CLAUDE_PLUGIN_ROOT}/scripts/post-tool-use.mjs"
"command": "node ${CLAUDE_PLUGIN_ROOT}/scripts/user-prompt-submit.mjs"
"command": "node ${CLAUDE_PLUGIN_ROOT}/scripts/session-start.mjs"
"command": "node ${CLAUDE_PLUGIN_ROOT}/scripts/stop.mjs"
"command": "node ${CLAUDE_PLUGIN_ROOT}/scripts/pre-compact.mjs"
"command": "node ${CLAUDE_PLUGIN_ROOT}/scripts/subagent-stop.mjs"
"command": "node ${CLAUDE_PLUGIN_ROOT}/scripts/teammate-idle.mjs"
"command": "node ${CLAUDE_PLUGIN_ROOT}/scripts/task-completed.mjs"
```

All use the `${CLAUDE_PLUGIN_ROOT}` variable correctly. No hardcoded paths.

---

### 7. No migration artifacts present

**PASS**

- No files matching `migrat`, `legacy`, `TODO`, `FIXME`, or `HACK` found anywhere under `.claude-plugin/`
- No stale files, no marketplace.json, no `.state/`, no `tmp/`, no `dist/`, no `src/`
- File list is exactly: plugin.json, 4 SKILL.md files, hooks.json, 9 .mjs scripts
- Clean target with no migration debris

---

### 8. File structure matches Claude Code plugin spec (skills/hooks/scripts nested inside .claude-plugin/)

**FAIL**

The current structure nests ALL directories inside `.claude-plugin/`:

```
targets/claude-code-plugin/
  .claude-plugin/
    plugin.json
    skills/          <-- INSIDE .claude-plugin/
    hooks/           <-- INSIDE .claude-plugin/
    scripts/         <-- INSIDE .claude-plugin/
```

The Claude Code plugin spec (research doc, line 34) is explicit:

> ".claude-plugin/ contains ONLY plugin.json. All other directories (commands/, agents/, skills/, hooks/) MUST be at the plugin root, NOT inside .claude-plugin/."

The correct structure per the spec should be:

```
targets/claude-code-plugin/
  .claude-plugin/
    plugin.json              # ONLY file in .claude-plugin/
  skills/                    # At plugin root
    orqa/SKILL.md
    orqa-save/SKILL.md
    orqa-create/SKILL.md
    orqa-validate/SKILL.md
  hooks/                     # At plugin root
    hooks.json
  scripts/                   # At plugin root
    pre-tool-use.mjs
    post-tool-use.mjs
    ...
```

**Important context:** ARCHITECTURE.md lines 1089-1090 contain an internal contradiction:
- Line 1089: "`.claude-plugin/` contains `plugin.json` plus `skills/`, `hooks/`, and `scripts/` nested inside it"
- Line 1090: "hooks/ and scripts/ are at plugin root, NOT inside .claude-plugin/"

The official Claude Code documentation (which the spec research is based on) is unambiguous: only `plugin.json` goes inside `.claude-plugin/`. Everything else is at the plugin root. The ARCHITECTURE.md contradiction should be resolved in favor of the official spec.

The `${CLAUDE_PLUGIN_ROOT}` variable resolves to the **plugin root directory** (the directory containing `.claude-plugin/`), not to `.claude-plugin/` itself. So when hooks.json references `${CLAUDE_PLUGIN_ROOT}/scripts/pre-tool-use.mjs`, it expects `scripts/` to be at the plugin root, not inside `.claude-plugin/`.

**This is a structural FAIL.** Skills, hooks, and scripts must be moved to the plugin root.

---

### 9. Every file has a purpose comment and every function has a description comment

**FAIL**

**Purpose comments (file-level):** All 9 scripts have a purpose comment at the top of the file. Examples:

- `pre-tool-use.mjs`: `// PreToolUse hook -- Write|Edit|Bash` + description of what the daemon checks
- `session-start.mjs`: `// SessionStart hook -- all matchers` + description of daemon responsibilities
- Each also states `// No business logic here -- all decisions are made by the daemon.`

**Function description comments:** MISSING. Every script defines an `async function main()` but none of them have a JSDoc or description comment on the function. The ARCHITECTURE.md zero tech debt standard (line 30) requires:

> "Every function should have a comment describing what it does and why."

None of the 9 scripts have comments on `main()`. While the file-level purpose comment partially covers this (since each file has a single function), the standard is explicit about per-function comments.

Additionally, `plugin.json`, `hooks.json`, and the SKILL.md files are data/documentation files, not code -- purpose comments don't apply to these in the traditional sense. The SKILL.md files are self-describing through their content. The JSON files cannot contain comments.

**Scripts that need `main()` function comments:**
- `pre-tool-use.mjs:17`
- `post-tool-use.mjs:14`
- `user-prompt-submit.mjs:14`
- `session-start.mjs:20`
- `stop.mjs:18`
- `pre-compact.mjs:17`
- `subagent-stop.mjs:16`
- `teammate-idle.mjs:14`
- `task-completed.mjs:16`

---

## Issues Found

### ISSUE-1 (CRITICAL): File structure violates Claude Code plugin spec

**Files affected:** All files under `targets/claude-code-plugin/.claude-plugin/skills/`, `targets/claude-code-plugin/.claude-plugin/hooks/`, `targets/claude-code-plugin/.claude-plugin/scripts/`

**Problem:** `skills/`, `hooks/`, and `scripts/` are nested inside `.claude-plugin/` but the official spec requires them at the plugin root. The `${CLAUDE_PLUGIN_ROOT}` variable resolves to the plugin root (parent of `.claude-plugin/`), so hook script paths would not resolve correctly with the current nesting.

**Fix:** Move `skills/`, `hooks/`, and `scripts/` from inside `.claude-plugin/` to `targets/claude-code-plugin/` (the plugin root). Only `plugin.json` should remain inside `.claude-plugin/`.

**Also fix:** Resolve the contradiction in ARCHITECTURE.md lines 1089-1090. Line 1089 says nested; line 1090 says not nested. Update line 1089 to match the official spec (not nested).

### ISSUE-2 (MINOR): Missing function-level description comments on `main()` in all 9 scripts

**Files affected:** All `.mjs` files under `targets/claude-code-plugin/.claude-plugin/scripts/`

**Problem:** Each script defines `async function main()` without a description comment. The zero tech debt standard requires every function to have a comment describing what it does and why.

**Fix:** Add a brief JSDoc or inline comment above each `main()` function. Since each file has only one function and the file-level comment is descriptive, a one-line comment like `// Reads hook event from stdin, forwards to daemon, applies response.` would suffice.

### ISSUE-3 (OBSERVATION): ARCHITECTURE.md internal contradiction on nesting

**File:** `ARCHITECTURE.md:1089-1090`

**Problem:** Two consecutive lines contradict each other about whether directories are nested inside `.claude-plugin/`. This is the root cause of ISSUE-1.

**Fix:** Update line 1089 to read: "`.claude-plugin/` contains ONLY `plugin.json`" and keep line 1090 as the authoritative statement.

---

## Lessons

- **Always cross-reference against the official plugin spec research**, not just ARCHITECTURE.md. The ARCHITECTURE.md contained a contradiction that propagated into the implementation.
- **The `${CLAUDE_PLUGIN_ROOT}` variable semantics are the key test**: if hook commands use `${CLAUDE_PLUGIN_ROOT}/scripts/...`, then `scripts/` must be at the plugin root, not nested inside `.claude-plugin/`. The variable name itself tells you the correct structure.
- **File-level purpose comments do not satisfy per-function comment requirements.** Even when a file has a single function and a detailed file-level comment, the zero tech debt standard explicitly requires function-level comments.

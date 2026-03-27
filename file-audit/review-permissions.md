# Permissions Review: Both settings.json Files

## Verdict: FAIL

Both files have critical bypass vulnerabilities and missing protections that undermine the deny rules.

---

## Files Reviewed

- `targets/claude-code-plugin/.claude/settings.json` (plugin target)
- `targets/claude-code-migration/.claude/settings.json` (migration target)

## Differences Between the Two Files

The files are nearly identical. Differences:

| Aspect | Plugin | Migration |
|--------|--------|-----------|
| `ORQA_DRY_RUN` | `"false"` | `"true"` |
| `ORQA_SKIP_SCHEMA_VALIDATION` | absent | `"true"` |
| PostToolUse hooks | markdownlint only | markdownlint + ESLint + Clippy |

Permissions (allow/deny) are **identical** between the two files.

---

## 1. Bypass Analysis

### FAIL: `Bash(cat *)` bypasses Edit/Write deny rules on `targets/`

The deny list blocks `Edit(./targets/**)` and `Write(./targets/**)`. However, the allow list includes `Bash(cat *)`. An agent could write to a protected file using shell redirection:

```bash
cat > targets/claude-code-plugin/.claude/settings.json << 'EOF'
{ "malicious": true }
EOF
```

This is auto-allowed because `Bash(cat *)` matches. The Edit/Write deny rules only apply to the Edit and Write tools, not to Bash commands that achieve the same effect.

**Severity: CRITICAL.** The entire `targets/` protection is bypassable.

### FAIL: `Bash(cat *)` bypasses Edit/Write deny on `ARCHITECTURE.md`

Same vector: `cat > ARCHITECTURE.md << 'EOF' ... EOF` is allowed by `Bash(cat *)`.

**Severity: CRITICAL.**

### FAIL: `Bash(cat *)` bypasses Edit/Write deny on `.claude/settings.json`

Same vector: `cat > .claude/settings.json << 'EOF' ... EOF` is allowed by `Bash(cat *)`.

**Severity: CRITICAL.** An agent could rewrite its own permission rules.

### FAIL: `Bash(cat *)` bypasses Edit/Write deny on `.claude/architecture/**`

Same vector for any architecture file.

**Severity: CRITICAL.**

### WARN: Other Bash commands that could bypass protections

The following are NOT in the allow list, so they would trigger a user prompt (not auto-allowed), but they are also NOT denied:

- `Bash(cp ...)` -- copy over a protected file
- `Bash(mv ...)` -- rename over a protected file
- `Bash(sed -i ...)` -- in-place edit of a protected file
- `Bash(tee ...)` -- write to a protected file
- `Bash(echo ... > file)` -- shell redirection (would need `Bash(echo *)` in allow list)
- `Bash(node -e "fs.writeFileSync(...)")` -- programmatic file write

These would prompt the user, so they are lower risk. But they are not explicitly denied either.

### WARN: `Bash(rm -rf *)` deny is too narrow

`Bash(rm -rf *)` is denied, but these are NOT:
- `Bash(rm -r *)` -- recursive delete without force
- `Bash(rm *)` -- delete individual files
- `Bash(rm -f *)` -- force delete individual files

An agent could delete protected files with `rm` (no `-rf` flag). However, since `rm` is not in the allow list, it would trigger a user prompt.

### PASS: `Bash(curl *)` and `Bash(wget *)` denies

These correctly block network exfiltration. No bypass via the allow list.

### PASS: `Bash(git push --force *)` and `Bash(git reset --hard *)` denies

These correctly block destructive git operations. No bypass via allowed git commands.

---

## 2. Legitimate Work Analysis

### PASS: Implementers can build Rust code

`Bash(cargo build *)`, `Bash(cargo test *)`, `Bash(cargo clippy *)`, `Bash(cargo fmt *)` are all allowed.

### PASS: Implementers can run frontend checks

`Bash(npx eslint *)`, `Bash(npx svelte-check *)`, `Bash(npx vitest *)`, `Bash(npx tsc *)` are all allowed.

### PASS: Agents can create new files in non-protected directories

The Edit and Write tools are not globally denied. Only specific paths are denied. Agents can freely create/edit files outside `targets/`, `ARCHITECTURE.md`, `.claude/settings.json`, and `.claude/architecture/`.

### PASS: Agents can run git workflow

`Bash(git add *)`, `Bash(git commit *)`, `Bash(git status *)`, `Bash(git diff *)`, `Bash(git log *)`, `Bash(git stash *)`, `Bash(git branch *)`, `Bash(git checkout *)`, `Bash(git merge *)` are all allowed.

### PASS: Agents can run orqa CLI commands

`Bash(orqa *)` is allowed.

### PASS: Agents can read all source files

Read is not denied for any source files (only `.env`, `.env.*`, `secrets/`, `.aws/`, `.ssh/`). Agents can freely read all code.

### WARN: `Bash(git push *)` is not allowed

`git push --force` is denied, but regular `git push` is not in the allow list. This means a normal push would trigger a user prompt. Per the CLAUDE.md instructions, agents don't push -- the orchestrator does after committing. But if a push is needed, it would require user approval each time.

This is actually CORRECT behavior per the project's design (agents don't push), but worth noting.

### WARN: `Bash(npm install *)` is not allowed

If an agent needs to install a new dependency, `npm install` is not in the allow list. `npm run *` is allowed (for scripts), but `npm install` would trigger a user prompt. This is likely intentional (dependency changes should require approval).

---

## 3. Missing Protections

### FAIL: `.orqa/manifest.json` is unprotected

The plugin manifest at `.orqa/manifest.json` controls which plugins are installed and active. An agent could silently modify this with Edit or Write. Only a Governance Steward should modify `.orqa/` artifacts per the role constraints, but settings.json cannot enforce role-based restrictions.

**Recommendation:** Add `Edit(./.orqa/manifest.json)` and `Write(./.orqa/manifest.json)` to deny list.

### FAIL: `.orqa/project.json` is unprotected

Project configuration. Same risk as manifest.json.

**Recommendation:** Add `Edit(./.orqa/project.json)` and `Write(./.orqa/project.json)` to deny list.

### WARN: `package.json` and `Cargo.toml` are unprotected

Root `package.json` and `Cargo.toml` control dependencies. An agent could add malicious dependencies. However, `npm install` is not auto-allowed, so the dependencies wouldn't be installed without user approval. The risk is lower but still present (a subsequent `npm run *` or `make *` could trigger installation).

**Recommendation:** Consider adding deny rules for root `package.json` and `Cargo.toml`, or accept the risk since dependency installation requires user prompt.

### WARN: `.claude/CLAUDE.md` is unprotected

The main project instructions file is not protected. An agent could modify the orchestrator's instructions. However, modifying CLAUDE.md mid-session would only take effect after a context compaction or new session.

**Recommendation:** Add `Edit(./.claude/CLAUDE.md)` and `Write(./.claude/CLAUDE.md)` to deny list.

### WARN: `Makefile` is unprotected

Since `Bash(make *)` is auto-allowed, an agent that modifies the Makefile could inject arbitrary commands that would then execute without prompts via `make <target>`.

**Recommendation:** Add `Edit(./Makefile)` and `Write(./Makefile)` to deny list.

### WARN: `scripts/validate-artifacts.mjs` is unprotected

This script is used in PreToolUse hooks for artifact validation. An agent could disable validation by modifying this script, then write invalid artifacts freely.

**Recommendation:** Add deny rules for `scripts/validate-artifacts.mjs`.

---

## 4. Role-Specific Gaps

### FAIL: settings.json cannot enforce per-role restrictions

Claude Code's settings.json permissions apply to ALL agents equally. The CLAUDE.md defines role-based tool constraints:

| Role | Can Edit | Can Run Shell |
|------|----------|---------------|
| Reviewer | No | Yes (checks only) |
| Researcher | No | No |
| Writer | Yes | No |

But settings.json cannot enforce these. A Reviewer agent spawned with `disallowedTools: "Edit,Write"` in its agent frontmatter would be restricted at the agent level, not the settings level.

**How frontmatter complements settings:**
- Settings.json: coarse-grained, protects specific paths from ALL agents
- Agent frontmatter `tools`/`disallowedTools`: fine-grained, restricts which tools a specific agent can use

**Gap:** If an agent's frontmatter says `disallowedTools: "Edit,Write"` but it has `Bash(cat *)` allowed, it can still write files via `cat > file`. The frontmatter `disallowedTools` field blocks the Edit/Write tools but does NOT block Bash commands that write files.

**This is the same bypass as section 1, but now it also undermines role-based restrictions.** A Reviewer that shouldn't edit code could use `Bash(cat > src/main.rs)` because `Bash(cat *)` is in the allow list.

### WARN: No PreToolUse hook validates Bash commands against protected paths

The PreToolUse hook only matches `Write|Edit` (artifact validation). There is no hook that inspects Bash commands to block writes to protected paths. A `PreToolUse` hook matching `Bash` that checks the command string for writes to protected paths would close the bypass gap.

---

## 5. Migration-Specific

### PASS: `ORQA_SKIP_SCHEMA_VALIDATION` is appropriate for migration

During migration, artifacts will be created/modified in bulk and may temporarily not conform to schemas. Skipping validation during migration prevents false-positive hook failures that would block progress. This should be removed post-migration.

### PASS: `ORQA_DRY_RUN` differs correctly between targets

Plugin target has `"false"` (normal operation). Migration target has `"true"` (safe mode during restructuring). Correct.

### WARN: No migration-specific env vars for phase gating

The migration CLAUDE.md describes a 10-phase migration with strict phase gating. There is no env var like `ORQA_MIGRATION_PHASE` that could be used by hooks to enforce which phase is active. This is enforcement via CLAUDE.md instructions only (soft enforcement).

### WARN: Post-migration permission changes not documented

After migration completes, several permissions should change:
- `ORQA_SKIP_SCHEMA_VALIDATION` should be removed
- `ORQA_DRY_RUN` should be set to `"false"`
- Additional deny rules may be needed for migrated file locations

There is no documented checklist for post-migration permission changes.

---

## 6. Additional Findings

### WARN: PostToolUse hooks differ between targets without clear rationale

The migration target has additional PostToolUse hooks (ESLint for `.ts`/`.svelte`, Clippy for `.rs`) that the plugin target lacks. If agents in the plugin target context will also edit TypeScript or Rust files, they should have the same quality hooks.

### INFO: Hook timeout differences

The migration target's Clippy hook has a 60-second timeout. Clippy on a full Rust workspace can take longer. If the workspace grows, this may need increasing.

### INFO: Both files use identical PreToolUse artifact validation

Both targets share the same `validate-artifacts.mjs` PreToolUse hook. This is good -- consistent validation regardless of which target is active.

---

## Summary of Issues

| # | Severity | Issue | Section |
|---|----------|-------|---------|
| 1 | CRITICAL | `Bash(cat *)` bypasses all Edit/Write deny rules | 1 |
| 2 | CRITICAL | Self-modification: agent can rewrite `.claude/settings.json` via Bash | 1 |
| 3 | CRITICAL | `targets/` protection entirely bypassable via Bash | 1 |
| 4 | FAIL | `.orqa/manifest.json` and `.orqa/project.json` unprotected | 3 |
| 5 | FAIL | Role-based restrictions bypassable via `Bash(cat *)` | 4 |
| 6 | WARN | `rm -rf` deny too narrow (doesn't cover `rm -r`, `rm -f`) | 1 |
| 7 | WARN | `.claude/CLAUDE.md` unprotected | 3 |
| 8 | WARN | `Makefile` unprotected (command injection via `make *`) | 3 |
| 9 | WARN | `scripts/validate-artifacts.mjs` unprotected | 3 |
| 10 | WARN | No PreToolUse hook on Bash to enforce path restrictions | 4 |
| 11 | WARN | Post-migration permission changes not documented | 5 |
| 12 | WARN | PostToolUse hooks differ between targets | 6 |

## Recommendations

### P0: Fix the `Bash(cat *)` bypass (CRITICAL)

**Option A (Recommended):** Remove `Bash(cat *)` from the allow list. Agents should use the Read tool to read files, not `cat`. If `cat` is needed for piping (e.g., `cat file | grep`), those specific patterns should be allowed instead.

**Option B:** Add a `PreToolUse` hook on `Bash` that parses the command string and blocks writes to protected paths. This is fragile (many ways to write files via Bash) but better than nothing.

**Option C:** Add explicit deny rules: `Bash(cat > *)`, `Bash(cat >> *)`. However, Claude Code's glob matching for Bash commands matches the full command string, and `>` is shell syntax, not part of the command -- this may not work reliably.

### P1: Add missing deny rules

```json
"Edit(./.orqa/manifest.json)",
"Write(./.orqa/manifest.json)",
"Edit(./.orqa/project.json)",
"Write(./.orqa/project.json)",
"Edit(./.claude/CLAUDE.md)",
"Write(./.claude/CLAUDE.md)",
"Edit(./Makefile)",
"Write(./Makefile)",
"Edit(./scripts/validate-artifacts.mjs)",
"Write(./scripts/validate-artifacts.mjs)"
```

### P2: Broaden destructive command denies

```json
"Bash(rm -r *)",
"Bash(rm -f *)"
```

### P3: Document post-migration permission changes

Create a checklist of settings.json changes needed when migration completes.

---

## Acceptance Criteria

- [x] Bypass analysis for each deny rule -- **FAIL**: Critical bypasses found via `Bash(cat *)`
- [x] Legitimate work analysis for each allow rule -- **PASS**: Agents can complete all migration tasks
- [x] Missing protections identified -- **FAIL**: `.orqa/manifest.json`, `.orqa/project.json`, `.claude/CLAUDE.md`, `Makefile`, validation script all unprotected
- [x] Role-specific gaps documented -- **FAIL**: Role restrictions bypassable via same `Bash(cat *)` vector
- [x] Migration-specific assessment -- **PASS**: `ORQA_SKIP_SCHEMA_VALIDATION` appropriate; post-migration changes need documentation

## Lessons

- Claude Code's permission system has a fundamental design limitation: Edit/Write deny rules do not prevent Bash commands from writing to the same paths. Any file protection strategy that relies solely on Edit/Write denies without also controlling Bash write vectors is incomplete.
- The `Bash(<command> *)` allow pattern is very broad. Allowing `Bash(cat *)` effectively grants unrestricted file write access via shell redirection, undermining all Edit/Write deny rules.
- Role-based tool restrictions in agent frontmatter (`disallowedTools`) suffer the same bypass -- blocking Edit/Write on an agent doesn't prevent it from writing via Bash.

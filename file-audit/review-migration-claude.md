# Review: Migration .claude/ Directory

## Verdict: FAIL

Three critical gaps found: architecture split files are missing recent ARCHITECTURE.md additions, settings.json deny rules are incomplete, and code comment standards are absent from agent instructions.

---

## Acceptance Criteria

### 1. All 12 architecture split files present and contain the LATEST content from ARCHITECTURE.md

**Verdict:** FAIL

**Files present (12/12):** PASS -- all 12 files exist in `targets/claude-code-migration/.claude/architecture/`:
- `core.md`, `plugins.md`, `agents.md`, `governance.md`, `enforcement.md`, `connector.md`, `structure.md`, `decisions.md`, `migration.md`, `targets.md`, `audit.md`, `glossary.md`

**Content currency:** FAIL -- the split files are missing critical recent additions from ARCHITECTURE.md:

| Missing Content | ARCHITECTURE.md Location | Split File | Status |
|----------------|-------------------------|------------|--------|
| **Zero tech debt** (4th core product principle) | Line 30 | `core.md` | MISSING -- core.md only has 3 principles (accuracy, enforcement, learning loop). The full "Zero tech debt" principle with code comment standards is absent. |
| **P6 title "with Mandatory Review"** | Line 58 | `core.md` line 55 | MISSING -- core.md says "P6: Hub-Spoke Orchestration" without "with Mandatory Review" |
| **P6 mandatory review process** (5-step process) | Lines 62-68 | `core.md` | MISSING -- core.md P6 body is a single sentence about summaries. The full 5-step mandatory review process (agent completes -> orchestrator spawns reviewer -> verdict -> re-assign or accept) is absent. |
| **Orchestrator gate check** (double-layer check) | Line 72 | `core.md` | MISSING -- The paragraph about orchestrator performing its own gate check after Reviewer PASS is absent from all split files. |
| **No autonomous decisions** | Line 74 | `core.md` | MISSING -- The full paragraph about agents not making their own interpretations, orchestrator compiling ambiguities for human review, is absent from all split files. |
| **Discovery during execution** | Line 76 | `core.md` | MISSING -- The full paragraph about agents reporting undocumented discoveries back to orchestrator, who compiles for user review, is absent from all split files. |
| **Code comment standards** | Line 30 (within zero tech debt) | `core.md` | MISSING -- "Every file should have a comment describing its purpose. Every function should have a comment describing what it does and why. Good inline documentation for active code; zero documentation of dead code." |

**Evidence:** Searched all files under `targets/claude-code-migration/.claude/architecture/` for "autonomous decision", "discovery during", "gate check", "double-layer", "compiles.*ambiguities", "Zero tech debt", "Every file should have a comment", "Comments describe active". None found except an unrelated use of "autonomously" in enforcement.md.

---

### 2. CLAUDE.md includes required sections

**Verdict:** PARTIAL PASS (3 items FAIL)

| Required Section | Status | Evidence |
|-----------------|--------|----------|
| Migration plan awareness | PASS | Lines 27-45: "Migration Plan" section with all 10 phases listed |
| Phase gating | PASS | Lines 43-45: "Phase Gating (STRICT)" section with explicit "Do NOT start Phase N+1 until Phase N is complete" |
| Zero tech debt enforcement | PASS | Lines 53-61: "Zero Tech Debt Enforcement" section with delete/no-compat/no-later/no-accumulation rules |
| Mandatory independent review | PASS | Lines 92-101: "Mandatory Independent Review" with 6-step process |
| Target protection | PASS | Lines 47-51: "Target Protection (NON-NEGOTIABLE)" with clear "NEVER modify files in targets/" |
| NEVER list | PASS | Lines 148-159: 10 items covering deferrals, ACs, legacy, targets, self-review, phases, shims, hardcoding, accumulation, permission |
| No autonomous decisions policy | FAIL | Not present in CLAUDE.md. The NEVER list and review sections do not include the "no autonomous decisions" policy from ARCHITECTURE.md P6 (agents must raise unclear items to orchestrator, who compiles for human review). |
| Discovery reporting | FAIL | Not present in CLAUDE.md. No mention of agents reporting discoveries (legacy code, stale artifacts, inconsistencies) back to orchestrator for user review. |
| Orchestrator gate check | FAIL | Not present in CLAUDE.md. The "Mandatory Independent Review" section describes spawning a reviewer but does NOT include the orchestrator's own gate check after Reviewer PASS (the double-layer check from ARCHITECTURE.md line 72). |

---

### 3. All 8 agents present with migration-specific context

**Verdict:** PASS

All 8 agent files exist in `targets/claude-code-migration/.claude/agents/`:
- `implementer.md` -- migration context via "Before Starting" section referencing architecture docs + migration.md
- `reviewer.md` -- migration context via "Review against ARCHITECTURE.md and the architecture files, not against current patterns"
- `researcher.md` -- references file-audit/ for existing analysis
- `writer.md` -- "Documentation must match the target architecture, not the current state"
- `planner.md` -- references migration.md for phase plan and sequencing
- `designer.md` -- references Phase 9 frontend alignment
- `governance-steward.md` -- references target .orqa/ structure from governance.md
- `orchestrator.md` -- full migration coordination with team lifecycle, reviewer spawning, phase gating

All agents have valid YAML frontmatter (name, description, model, tools, maxTurns) and migration-aware instructions.

---

### 4. Every agent that writes code instructs: file purpose comments, function description comments, no legacy removal comments

**Verdict:** FAIL

Code-writing agents (implementer, designer, governance-steward) do NOT include instructions about code comment standards. Searched all agent files for "file purpose", "function description", "function comment", "Comments describe active", "no legacy removal". Zero matches.

The implementer.md has a "Zero Tech Debt" section that says "Delete legacy code -- do not comment it out" but this is about not leaving commented-out legacy code. It does NOT instruct agents to:
- Add file purpose comments to every file
- Add function description comments to every function
- Never leave comments documenting what was removed ("that's code smell, not documentation")

These specific standards come from ARCHITECTURE.md line 30 and are not reflected anywhere in the migration .claude/ directory.

---

### 5. Agents reference architecture docs with specific file pointers

**Verdict:** PASS

Every agent includes an "Architecture Reference" section listing all 12 architecture files with specific file paths:
- `.claude/architecture/core.md` -- design principles, engine libraries
- `.claude/architecture/plugins.md` -- plugin system, composition
- `.claude/architecture/agents.md` -- agent architecture, prompt pipeline
- `.claude/architecture/governance.md` -- `.orqa/` structure, artifact lifecycle
- `.claude/architecture/enforcement.md` -- enforcement layers, validation
- `.claude/architecture/connector.md` -- connector architecture
- `.claude/architecture/structure.md` -- directory structure
- `.claude/architecture/decisions.md` -- key design decisions
- `.claude/architecture/migration.md` -- migration phases
- `.claude/architecture/targets.md` -- target state specifications
- `.claude/architecture/audit.md` -- audit criteria
- `.claude/architecture/glossary.md` -- term definitions

Additionally, role-specific agents reference the most relevant files in their "Before Starting" sections (e.g., governance-steward references governance.md first, planner references migration.md).

---

### 6. Settings.json has ORQA_SKIP_SCHEMA_VALIDATION=true

**Verdict:** PASS

`settings.json` line 7: `"ORQA_SKIP_SCHEMA_VALIDATION": "true"` is present in the `env` section. This correctly disables schema validation during migration per ARCHITECTURE.md Phase 1 Step 3.

---

### 7. Settings.json has direct enforcement hooks (not daemon wrappers)

**Verdict:** PASS

All hooks call tools directly, not through a daemon:

- **PreToolUse** (line 58-69): Calls `node "$CLAUDE_PROJECT_DIR/scripts/validate-artifacts.mjs"` directly
- **PostToolUse** (line 72-95): Three hooks calling tools directly:
  1. `npx markdownlint-cli2` for .md files in .orqa/
  2. `npx eslint` for .ts/.svelte files
  3. `cargo clippy` for .rs files

No daemon wrappers, no MCP calls. All enforcement is direct invocation. This matches the ARCHITECTURE.md Phase 1 Step 3 requirement that "hooks invoke eslint, clippy, markdownlint, the validation script, and scoped tests directly."

---

### 8. Settings.json deny rules protect targets/, ARCHITECTURE.md, .claude/settings*

**Verdict:** FAIL

| Protection Target | Status | Evidence |
|-------------------|--------|----------|
| `targets/` | PARTIAL | `Edit(./targets/**)` is denied (line 49), but `Write(./targets/**)` is NOT denied. An agent could use the Write tool to overwrite target files, bypassing the Edit deny rule. |
| `ARCHITECTURE.md` | MISSING | No deny rule for `Edit(./ARCHITECTURE.md)` or `Write(./ARCHITECTURE.md)`. The architecture reference document is unprotected. |
| `.claude/settings*` | MISSING | No deny rule for `Edit(./.claude/settings*)` or `Write(./.claude/settings*)`. Settings could be modified by agents. |

Full deny list from settings.json:
```json
"deny": [
  "Read(./.env)",
  "Read(./.env.*)",
  "Read(./secrets/**)",
  "Read(//.aws/**)",
  "Read(//.ssh/**)",
  "Edit(./targets/**)",
  "Bash(rm -rf *)",
  "Bash(git push --force *)",
  "Bash(git reset --hard *)",
  "Bash(curl *)",
  "Bash(wget *)"
]
```

---

### 9. Reviewer agent instructions: review against ARCHITECTURE.md not current patterns, every AC verified with evidence, FAIL if any AC incomplete

**Verdict:** PASS

`reviewer.md` contains all required instructions:

- **Review against architecture:** Line 21: "Review against ARCHITECTURE.md and the architecture files, not against current patterns. The migration is moving FROM current patterns TO the target architecture. If the implementation matches current patterns but not the architecture, that is a FAIL."
- **Every AC verified with evidence:** Line 42: "Every acceptance criterion MUST have a verdict -- no omissions." Verdict format (lines 50-56) requires evidence for each AC.
- **FAIL if any AC incomplete:** Line 43: "FAIL if any AC is incomplete -- no partial passes."

Additional good checks: FAIL if legacy code survives, FAIL if targets/ modified, FAIL if implementation contradicts architecture docs.

---

### 10. Orchestrator CLAUDE.md: spawns Reviewer for every task, does own gate check after Reviewer PASS, compiles ambiguities for human review

**Verdict:** FAIL

| Requirement | Status | Evidence |
|-------------|--------|----------|
| Spawns Reviewer for every task | PASS | CLAUDE.md lines 92-101 and orchestrator.md lines 36-38: "You MUST spawn a Reviewer for every completed task. No task is done without a PASS verdict from an independent Reviewer." |
| Does own gate check after Reviewer PASS | FAIL | Neither CLAUDE.md nor orchestrator.md include the double-layer check. CLAUDE.md's "Completion Gate" (lines 135-146) checks that a Reviewer returned PASS, but does NOT describe the orchestrator's own verification that "the verdict is consistent with the acceptance criteria, the architecture, and the zero tech debt standard." The orchestrator is told to read verdicts but not to independently verify them against the architecture. |
| Compiles ambiguities for human review | FAIL | Neither CLAUDE.md nor orchestrator.md include the "no autonomous decisions" policy. There is no instruction for the orchestrator to compile ambiguities and discoveries into a list for human review. |

---

### 11. No autonomous decision-making -- unclear items raised to orchestrator who compiles for human review

**Verdict:** FAIL

This policy from ARCHITECTURE.md P6 (line 74) is entirely absent from the migration .claude/ directory:

- Not in CLAUDE.md
- Not in orchestrator.md
- Not in any agent file
- Not in any architecture split file

The original text reads: "No autonomous decisions. If something isn't clear in the task description, the agent refers to the architecture documentation. If still not clear, the agent raises it to the orchestrator -- it does NOT make its own interpretation and proceed. The orchestrator compiles all ambiguities and discoveries into a list for human review before the task moves forward."

Similarly, the discovery reporting policy (line 76) about agents reporting undiscovered legacy code, stale artifacts, and inconsistencies is also absent.

---

## Issues Found

### CRITICAL: Architecture split file `core.md` is stale

`targets/claude-code-migration/.claude/architecture/core.md` is missing content that was added to ARCHITECTURE.md after the split files were created. The following items from ARCHITECTURE.md are not in any split file:

1. **Zero tech debt** as a 4th core product principle (line 30) -- including code comment standards
2. **P6 "with Mandatory Review"** title and expanded content (lines 58-76):
   - 5-step mandatory review process
   - Orchestrator gate check (double-layer)
   - No autonomous decisions policy
   - Discovery during execution reporting

These are load-bearing architectural constraints that agents will not see if they only read the split files (which is what all agent instructions tell them to do).

**File:** `targets/claude-code-migration/.claude/architecture/core.md` lines 23-61

### CRITICAL: Settings deny rules incomplete

Three protection targets missing from `settings.json` deny rules:

1. `Write(./targets/**)` -- targets can be overwritten via Write tool
2. `Edit(./ARCHITECTURE.md)` and `Write(./ARCHITECTURE.md)` -- architecture reference unprotected
3. `Edit(./.claude/settings*)` and `Write(./.claude/settings*)` -- settings unprotected

**File:** `targets/claude-code-migration/.claude/settings.json` lines 43-55

### CRITICAL: Code comment standards missing from all agent instructions

No agent that writes code is instructed to:
- Add a comment describing the file's purpose to every file
- Add a comment describing what each function does and why
- Never leave comments documenting what was removed

This is specified in ARCHITECTURE.md line 30 as part of the zero tech debt principle.

**Files:** All agent .md files in `targets/claude-code-migration/.claude/agents/`

### HIGH: No autonomous decisions policy missing everywhere

ARCHITECTURE.md P6 line 74 specifies that agents must not make autonomous decisions when things are unclear -- they must raise to the orchestrator who compiles for human review. This is absent from:
- CLAUDE.md
- orchestrator.md
- All agent files
- All architecture split files

### HIGH: Discovery reporting policy missing everywhere

ARCHITECTURE.md P6 line 76 specifies that agents must report anything they encounter that was not covered by the task list. This is absent from all migration .claude/ files.

### HIGH: Orchestrator gate check missing

ARCHITECTURE.md line 72 specifies a double-layer check: after Reviewer PASS, the orchestrator performs its own gate check verifying the verdict against AC, architecture, and zero tech debt. This is not in CLAUDE.md or orchestrator.md.

---

## Summary of Verdicts

| # | Check | Verdict |
|---|-------|---------|
| 1 | 12 architecture split files with latest content | FAIL |
| 2 | CLAUDE.md includes required sections | FAIL (3 sub-items) |
| 3 | All 8 agents with migration context | PASS |
| 4 | Code-writing agents instruct comment standards | FAIL |
| 5 | Agents reference architecture docs | PASS |
| 6 | ORQA_SKIP_SCHEMA_VALIDATION=true | PASS |
| 7 | Direct enforcement hooks | PASS |
| 8 | Deny rules protect targets/, ARCHITECTURE.md, settings | FAIL |
| 9 | Reviewer instructions correct | PASS |
| 10 | Orchestrator spawns reviewer + gate check + compiles ambiguities | FAIL |
| 11 | No autonomous decision-making policy | FAIL |

**Overall: 5 PASS, 6 FAIL**

## Lessons

1. When architecture documents are split into multiple files, any additions to the source document must be propagated to the split files. The split files were created from an earlier version of ARCHITECTURE.md and were not updated when P6 was expanded.
2. Deny rules need both `Edit` and `Write` variants to be effective -- using only `Edit` leaves a bypass via the `Write` tool.
3. The most critical policy additions (no autonomous decisions, discovery reporting, orchestrator gate check) were added to ARCHITECTURE.md but never propagated to ANY of the operational files that agents actually read during execution.

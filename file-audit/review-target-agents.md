# Review: Target .claude/ Agents and CLAUDE.md

## Verdict: FAIL

6 of 9 acceptance criteria pass. 3 fail.

---

## Acceptance Criteria

### AC1: All 8 agents present
**Verdict:** PASS

All 8 agent files exist in `targets/claude-code-plugin/.claude/agents/`:
- `orchestrator.md`
- `implementer.md`
- `reviewer.md`
- `researcher.md`
- `writer.md`
- `planner.md`
- `designer.md`
- `governance-steward.md`

### AC2: Frontmatter uses correct fields
**Verdict:** PASS

All agents use only: `name`, `description`, `model`, `tools`, `maxTurns`. None use `hooks`, `mcpServers`, or `permissionMode` -- which aligns with the plugin agent restrictions documented in the research spec (lines 144-151): "plugin-shipped agents do NOT support hooks, mcpServers, permissionMode."

Summary of frontmatter by agent:

| Agent | name | description | model | tools | maxTurns |
|-------|------|-------------|-------|-------|----------|
| orchestrator | yes | yes | opus | yes | 200 |
| implementer | yes | yes | sonnet | yes | 50 |
| reviewer | yes | yes | sonnet | yes | 30 |
| researcher | yes | yes | sonnet | yes | 40 |
| writer | yes | yes | sonnet | yes | 30 |
| planner | yes | yes | opus | yes | 40 |
| designer | yes | yes | sonnet | yes | 30 |
| governance-steward | yes | yes | sonnet | yes | 30 |

### AC3: Tool restrictions match ARCHITECTURE.md section 6.1 role definitions
**Verdict:** FAIL

Cross-referencing agent tool lists against ARCHITECTURE.md section 6.1 permission scopes:

| Role | ARCH 6.1 Permission Scope | Agent Tools | Issues |
|------|--------------------------|-------------|--------|
| **Orchestrator** | Read-only, delegation | Read,Glob,Grep,Agent,TeamCreate,TaskCreate,TaskUpdate,TaskGet,TaskList,SendMessage,TeamDelete | OK -- read tools + delegation tools, no Edit/Write/Bash |
| **Implementer** | Source code, shell access | Read,Write,Edit,Bash,Grep,Glob,Agent,TaskCreate,TaskUpdate,TaskGet,TaskList,SendMessage | ISSUE: Has Agent, TaskCreate, TaskList, SendMessage -- implementer should not be delegating or creating tasks. These are orchestrator capabilities. |
| **Reviewer** | Read-only, checks only | Read,Bash,Grep,Glob,TaskUpdate | OK -- read tools + Bash for checks + TaskUpdate for status |
| **Researcher** | Read-only, creates research artifacts | Read,Glob,Grep,WebSearch,WebFetch,Write,TaskUpdate | ISSUE: Has Write tool but ARCH 6.1 says "Read-only, creates research artifacts." The CLAUDE.md role table says "Creates research artifacts only" which implies Write is needed for research output. Marginal -- the Write is scoped by the system prompt to `.orqa/discovery/research/` or `.state/research/` only. Acceptable with the system prompt boundary. |
| **Writer** | Documentation only | Read,Write,Edit,Glob,Grep,TaskUpdate | OK -- write tools scoped to documentation by system prompt |
| **Planner** | Plans and delivery artifacts | Read,Glob,Grep,Write,TaskUpdate | OK -- write for plan artifacts, scoped by system prompt |
| **Designer** | Design artifacts, component code | Read,Write,Edit,Glob,Grep,TaskUpdate | OK -- write/edit for design artifacts and components |
| **Governance Steward** | `.orqa/` artifacts only | Read,Write,Edit,Glob,Grep,TaskUpdate | OK -- write/edit scoped to `.orqa/` by system prompt |

**Failure reason:** The implementer agent has `Agent`, `TaskCreate`, `TaskList`, `SendMessage` tools. These are orchestration/delegation tools. Per ARCHITECTURE.md 6.1, the implementer's scope is "Source code, shell access" -- it should not be creating teams, spawning sub-agents, or messaging teammates. The CLAUDE.md role table (line 54) confirms: Implementer = "Can Edit: Yes, Can Run Shell: Yes, Artifact Scope: Source code only." No delegation capability is listed.

### AC4: Model assignments are correct
**Verdict:** PASS

- Orchestrator: `opus` -- CORRECT
- Planner: `opus` -- CORRECT
- All others (implementer, reviewer, researcher, writer, designer, governance-steward): `sonnet` -- CORRECT

Matches the CLAUDE.md Agent Delegation table (lines 42-51).

### AC5: System prompts include required sections
**Verdict:** PASS

Checking each agent for: role description, boundaries, quality checks, completion standard, output format.

| Agent | Role Description | Boundaries | Quality Checks | Completion Standard | Output Format |
|-------|-----------------|------------|---------------|-------------------|---------------|
| orchestrator | Yes (line 9-11) | Yes ("## Boundaries" section) | N/A (delegates) | Yes ("## Completion Gate") | Yes ("## Output") |
| implementer | Yes (line 9) | Yes ("## Boundaries") | Yes ("## Quality Checks" -- cargo, svelte-check, eslint) | Yes ("## Completion Standard") | Yes ("## Output") |
| reviewer | Yes (line 9) | Yes ("## Boundaries") | Yes ("## Verification Approach" -- tests, linters, type checks) | N/A (produces verdicts) | Yes ("## Output" + "## Verdict Format") |
| researcher | Yes (line 9) | Yes ("## Boundaries") | Yes ("## Research Quality") | N/A (research output) | Yes ("## Output") |
| writer | Yes (line 9) | Yes ("## Boundaries") | Yes ("## Writing Quality") | N/A (doc output) | Yes ("## Output") |
| planner | Yes (line 9) | Yes ("## Boundaries") | Yes ("## Planning Quality") | N/A (plan output) | Yes ("## Output") |
| designer | Yes (line 9) | Yes ("## Boundaries") | Yes ("## Design Quality") | N/A (design output) | Yes ("## Output") |
| governance-steward | Yes (line 9) | Yes ("## Boundaries") | Yes ("## Artifact Quality") | N/A (artifact output) | Yes ("## Output") |

All agents that can modify files (implementer, writer, planner, designer, governance-steward) have explicit completion standards or quality sections. Reviewer has verdict format. Orchestrator has completion gate.

### AC6: Code comment instructions
**Verdict:** FAIL

**No agent system prompt instructs agents to add file purpose comments or function description comments when writing code.**

The implementer agent (the primary code-writing role) has no mention of:
- Adding file purpose comments (e.g., `// This file provides...` at the top of new files)
- Adding function description comments (e.g., JSDoc, rustdoc, docstrings)

The designer agent (which writes component code) also lacks these instructions.

This was an explicit acceptance criterion: "System prompts instruct agents to add file purpose comments and function description comments when writing code."

### AC7: CLAUDE.md covers required topics
**Verdict:** FAIL

Checking each required topic:

| Topic | Present | Evidence |
|-------|---------|----------|
| Principles | YES | "## Design Principles" table (lines 5-16), plus "Core product principles" (line 17) |
| Team discipline | YES | "## Team Discipline" section (lines 19-73) |
| Mandatory independent review | **NO** | Not mentioned anywhere in the CLAUDE.md. No instruction that implementation work must be independently reviewed by a reviewer agent before acceptance. |
| Agent delegation | YES | "### Agent Delegation" table (lines 42-51) + "### Role-Based Tool Constraints" table (lines 52-63) |
| Completion gate | PARTIAL | "### Completion Gate" section (lines 65-73) exists but does NOT include an orchestrator gate check -- i.e., no instruction that the orchestrator must verify findings before proceeding. The gate says "Read all findings files" and "Verify EVERY acceptance criterion" but does not explicitly require the orchestrator to check that a reviewer has PASSED the work. |
| Git workflow | YES | "## Git Workflow" section (lines 92-97) |

**Failure reasons:**
1. **Mandatory independent review is missing entirely.** There is no instruction requiring implementation work to be reviewed by a reviewer agent before the orchestrator accepts it. The completion gate says to "read findings" and "verify ACs" but does not mandate that a reviewer agent has produced a PASS verdict.
2. **Orchestrator gate check is incomplete.** The completion gate should specify that the orchestrator must verify a reviewer's PASS verdict exists (not just read its own assessment of findings). The current wording allows the orchestrator to skip independent review and accept work based solely on the implementer's self-reported findings.

### AC8: No migration-specific content
**Verdict:** PASS

The CLAUDE.md contains no migration-specific content. The only mention of "migrate" is the generic design decision "data migrated via `orqa migrate`" (line 89), which is a permanent product feature, not migration-specific.

Note: There IS a `migration.md` file in `.claude/architecture/` but this is architecture documentation about the migration plan, not migration-specific content in the agent prompts or CLAUDE.md itself. The architecture docs are reference material, not operational instructions.

### AC9: Settings.json deny rules
**Verdict:** PASS (with observations)

Required deny rules and their status:

| Required Protection | Present | Rule |
|-------------------|---------|------|
| targets/ (Edit) | YES | `Edit(./targets/**)` |
| targets/ (Write) | YES | `Write(./targets/**)` |
| ARCHITECTURE.md (Edit) | YES | `Edit(./ARCHITECTURE.md)` |
| .claude/settings.json (Edit) | YES | `Edit(./.claude/settings.json)` |
| .claude/settings.local.json (Edit) | YES | `Edit(./.claude/settings.local.json)` |

**Observations (non-blocking):**
- `Write(./ARCHITECTURE.md)` is NOT denied -- only `Edit` is. An agent could use `Write` to overwrite ARCHITECTURE.md entirely. This is a gap, though `Write` is a less common operation for existing files.
- `Write(./.claude/settings.json)` and `Write(./.claude/settings.local.json)` are NOT denied -- only `Edit` is. Same gap pattern.
- These are gaps but not explicitly required by the AC, which says "deny rules protect targets/, ARCHITECTURE.md, .claude/settings*". The Edit denials do provide protection. However, incomplete protection against Write is a real risk.

---

## Issues Found

### FAIL-1: Implementer has orchestration tools (AC3)
**File:** `targets/claude-code-plugin/.claude/agents/implementer.md:6`
**Issue:** Tools list includes `Agent,TaskCreate,TaskList,SendMessage` which are delegation/orchestration capabilities. Per ARCHITECTURE.md 6.1, implementer scope is "Source code, shell access." These tools should be removed.
**Fix:** Change tools to `"Read,Write,Edit,Bash,Grep,Glob,TaskUpdate,TaskGet"`

### FAIL-2: No code comment instructions (AC6)
**File:** `targets/claude-code-plugin/.claude/agents/implementer.md`
**Issue:** System prompt does not instruct the agent to add file purpose comments or function description comments when writing code. The designer agent has the same gap.
**Fix:** Add a section like:
```
## Code Documentation
- Add a file purpose comment at the top of new files (e.g., `// Provides X for Y`)
- Add brief function/method description comments for non-trivial functions
```

### FAIL-3: Missing mandatory independent review and incomplete orchestrator gate check (AC7)
**File:** `targets/claude-code-plugin/.claude/CLAUDE.md`
**Issue:** No instruction requiring independent reviewer verification before accepting work. The completion gate allows the orchestrator to accept implementer self-reported findings without reviewer validation.
**Fix:** Add to CLAUDE.md:
```
### Mandatory Independent Review

All implementation work MUST be independently reviewed by a Reviewer agent before the orchestrator accepts it. The orchestrator must verify that a reviewer has produced a PASS verdict -- implementer self-reported findings are NOT sufficient for acceptance.
```
And update the Completion Gate to include: "Verify a Reviewer has produced a PASS verdict for each implementation task."

### OBSERVATION-1: Write deny gaps in settings.json (AC9)
**File:** `targets/claude-code-plugin/.claude/settings.json:42-58`
**Issue:** `ARCHITECTURE.md` and `.claude/settings*` are protected against `Edit` but not `Write`. An agent could bypass protection by using Write (full file overwrite) instead of Edit.
**Recommended fix:** Add:
```json
"Write(./ARCHITECTURE.md)",
"Write(./.claude/settings.json)",
"Write(./.claude/settings.local.json)"
```

---

## Lessons

- **Tool scope creep:** When defining agent tool lists, verify that each tool granted is justified by the role's permission scope. Delegation tools (Agent, TaskCreate, SendMessage) are orchestrator-only capabilities and should not appear on worker agents.
- **Deny rule completeness:** When protecting files via deny rules, both `Edit` and `Write` must be denied. `Edit` prevents modifications but `Write` can overwrite the entire file, achieving the same result.
- **Implicit vs explicit review:** A completion gate that says "verify ACs" is weaker than one that says "verify a reviewer PASSED." The former allows self-assessment; the latter enforces independent verification.

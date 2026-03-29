---
id: "PD-26b0eb9f"
type: principle-decision
title: "Skill-Driven Project Initialisation"
description: "Project setup is driven by skills, not templates. A base setup skill handles scaffolding, an inference skill reads existing folder content, and project type skills provide domain-specific governance presets."
status: completed
created: "2026-03-09"
updated: "2026-03-09"
relationships:
  - target: "EPIC-7bf140e2"
    type: "drives"
---

## Decision

Project initialisation is handled entirely through skills — no separate
template directory, no hardcoded scaffolding logic in the app. Skills
contain the knowledge of what a project needs, how to infer project
characteristics, and how to scaffold governance accordingly.

## Context

OrqaStudio needs to initialise governance (`.orqa/` directory) for new
and existing projects. The initial approach considered a `/templates/`
directory containing agent definitions, rules, and skills that would be
copied during init. This creates a sync problem — templates and skills
diverge over time.

The insight: a skill IS a template. It contains the knowledge of what
to create and how to configure it. The setup skill replaces the template
directory entirely.

## The Four Setup Skills

### 1. `project-setup` (Base Skill)

Universal scaffolding knowledge. Loaded on every project initialisation.

**Knows how to:**

- Create the `.orqa/` directory structure
- Install the lean orchestrator core (Section 1 only)
- Install core rules (documentation-first, honest-reporting, etc.)
- Install core skills (code-search, planning, composability, etc.)
- Create `project.json` with base configuration
- Set up symlinks for CLI compatibility (`.claude/` → `.orqa/`)

**Does NOT know:** What kind of project this is. That comes from
inference or user input.

### 2. `project-inference` (Analysis Skill)

Reads an existing folder and produces a project profile. Does not
modify anything — only produces findings.

**Scans for:**

- **Software signals**: `package.json`, `Cargo.toml`, `requirements.txt`,

  `go.mod`, `pom.xml`, `.sln` → language, framework, package manager

- **Existing governance**: `.github/`, `CONTRIBUTING.md`, linting configs,

  `Makefile`, CI configs → existing standards to respect

- **Existing AI tooling**: `.claude/`, `.cursor/`, `.aider/`, `.copilot/`

  → existing AI config to integrate with

- **Documentation signals**: `README.md`, `docs/`, wiki structures

  → existing documentation patterns

- **Domain signals**: file types, directory names, content patterns

  → research papers, design files, data files, etc.

**Produces:** A structured project profile that the setup skill and
type skills consume:

```yaml
inferred_type: software
languages: [typescript, rust]
frameworks: [svelte, tauri]
package_managers: [npm, cargo]
existing_governance:
  - eslint (configured)
  - clippy (configured)
  - github_actions (3 workflows)
existing_ai_tooling:
  - claude_code (settings.json exists)
documentation_patterns:
  - docs/ directory with markdown
  - README.md present
domain_signals: []
confidence: high
```

### 3. Project Type Skills (Domain Presets)

Preconfigured governance for known project domains. Each adds
domain-appropriate rules, skills, agent configurations, and
artifact types on top of the base setup.

**Initial types:**

| Skill | Domain | Adds |
| ------- | -------- | ------ |
| `project-type-software` | Software development | Worktree rules, code quality rules, tech-specific skills, CI patterns, testing standards |
| `project-type-consulting` | Consulting engagements | Engagement structure, deliverable templates, stakeholder rules, review cycles |
| `project-type-research` | Research projects | Methodology framework, findings structure, literature review patterns, hypothesis tracking |

**Future types** can be added as skills — no app code changes needed:

- `project-type-operations` — process improvement, SOP management
- `project-type-product` — product management, roadmapping, user research
- `project-type-creative` — content creation, editorial workflows

### 4. `project-type-custom` (Collaborative Setup)

For projects that don't fit a known type. Uses inference findings
as a starting point, then conducts a structured conversation:

- "What are you trying to achieve with this project?"
- "What does quality look like in your domain?"
- "What artefacts do you produce?"
- "What review/approval processes exist?"
- "Who are the stakeholders?"

Generates governance configuration from the answers. The user
validates and adjusts before the skill scaffolds.

## Initialisation Flow

### New Project (Empty Folder)

```text
1. Load project-setup
2. Ask: "What kind of project?"
   → Known type → Load project-type-X → Scaffold with sensible defaults
   → Custom → Load project-type-custom → Collaborative conversation → Scaffold
3. project-setup creates .orqa/ with base + type-specific config
4. Orchestrator Section 2 generated from project profile
```

### Existing Project (Folder Has Content)

```text
1. Load project-setup + project-inference
2. project-inference scans folder → produces project profile
3. Present findings: "I see a TypeScript/React project with ESLint..."
4. User confirms or corrects the inference
5. Load appropriate project-type-X (or custom)
6. project-setup creates .orqa/ respecting existing governance
7. Orchestrator Section 2 generated from confirmed profile
```

### Existing Agentic Project (Has AI Governance)

```text
1. Load project-setup + project-inference + project-migration
2. project-inference scans folder → produces project profile
3. project-migration scans for existing agentic config:
   - .claude/, CLAUDE.md (Claude Code)
   - .cursorrules, .cursor/ (Cursor)
   - .aider/ (Aider)
   - .github/copilot-instructions.md (Copilot)
   - AGENTS.md (emerging convention)
   - Previous .orqa/ (version upgrade)
4. project-migration reads existing governance, maps to OrqaStudio concepts:
   - Their rules/instructions → our rules + skills
   - Their agent configs → our role definitions
   - Their ignore patterns → our excluded_paths
5. Present migration plan: "You have 12 rules in CLAUDE.md. Here's how
   they map to OrqaStudio governance..."
6. User approves, adjusts, or rejects each mapping
7. project-setup scaffolds .orqa/ incorporating migrated governance
8. Existing config preserved or symlinked for coexistence where sensible
```

### Migration Skill Capabilities

The `project-migration` skill teaches the agent to:

1. **Detect** which agentic frameworks are present (file patterns, config formats)
2. **Parse** their governance — read rules, instructions, agent definitions in

   whatever format the source framework uses

3. **Understand intent** — what is each rule/instruction trying to achieve?

   Not just syntactic mapping but semantic understanding

4. **Map to OrqaStudio concepts** — source rule → OrqaStudio rule, source

   instruction → OrqaStudio skill, source agent config → OrqaStudio role

5. **Identify gaps** — what does the existing setup NOT cover that OrqaStudio

   would add? Present these as recommendations, not forced additions

6. **Propose coexistence** — some teams need both systems during transition.

   Where possible, maintain the source config as symlinks or generated output
   from OrqaStudio's governance (e.g., generate `.cursorrules` from `.orqa/rules/`)

7. **Evolve** — new AI tooling frameworks will emerge. The skill needs to be

   updatable without app code changes

## How OrqaStudio Itself Uses This

OrqaStudio's `.orqa/` directory is the result of:

- `project-setup` (base scaffolding)
- `project-inference` (detected Tauri + Svelte + Rust)
- `project-type-software` (code quality, worktrees, testing)
- OrqaStudio-specific extensions:
  - Dogfood mode rules
  - Two-Pillar governance framework
  - Streaming pipeline skills (`orqa-streaming`, etc.)
  - Custom system prompt for self-awareness

The project eats its own cooking. Its `.orqa/` is a concrete instance
of exactly what the setup skills would produce for a Tauri/Svelte/Rust
project, plus the project-specific additions that make it OrqaStudio.

## The Full Skill Set

| Skill | Role | When Loaded |
| ------- | ------ | ------------- |
| `project-setup` | Base scaffolding mechanics | Every initialisation |
| `project-inference` | Reads folder, produces project profile | Every initialisation |
| `project-migration` | Reads existing agentic config, maps to OrqaStudio | When existing AI governance detected |
| `project-type-software` | Software development governance preset | Software projects |
| `project-type-consulting` | Consulting engagement preset | Consulting projects |
| `project-type-research` | Research project preset | Research projects |
| `project-type-custom` | Collaborative setup for unknown domains | When no type matches |

## Permanent Coexistence: Governance Hub Model

OrqaStudio is not a replacement for existing AI development tools — it is
an **additive governance layer** that distributes to whatever tools the
team already uses. Coexistence is not just for migration — it is the
long-term operating model.

### The Distribution Model

`.orqa/` is the single source of truth for governance. Other tools
consume it via symlinks (same format) or generated output (different format):

```text
.orqa/                              ← Source of truth (OrqaStudio manages)
  governance/rules/
  team/agents/
  team/skills/

.claude/                            ← Claude Code (symlinks — same format)
  rules/ → .orqa/process/rules/
  agents/ → .orqa/process/agents/
  CLAUDE.md → .orqa/process/agents/orchestrator.md

.cursorrules                        ← Cursor (generated — different format)
.github/copilot-instructions.md     ← Copilot (generated — different format)
.aider/                             ← Aider (generated — different format)
```

Symlinks work when the consuming tool reads the same markdown format
(Claude Code). Generated files are needed when the consuming tool expects
a different format (Cursor wants a single flat file, Copilot wants a
specific path).

### Team Roles and Enforcement Layers

| Person | Tool | Gets Rules From | Enforcement Level |
| -------- | ------ | ---------------- | ------------------- |
| PM / Tech Lead | OrqaStudio app | `.orqa/` directly | Full: scanner, gates, learning loop |
| Developer A | Claude Code CLI | `.claude/` symlinks | Self-policed: rules in context, no scanner |
| Developer B | Cursor | `.cursorrules` generated | Self-policed: rules in context |
| Developer C | OrqaStudio app | `.orqa/` directly | Full enforcement |

Everyone works from the same governance. The PM defines and manages rules
in OrqaStudio's UI. Developers consume those rules through whatever tool
they prefer. OrqaStudio users get the full enforcement layer (scanning,
verification gates, learning loop). CLI users get the rules loaded into
AI context but enforcement is self-policed.

### Generation Skills

The migration skill handles initial import. For ongoing distribution,
the setup skill (or a dedicated `governance-distribution` skill) knows
how to generate tool-specific config files from `.orqa/` governance:

- **Symlink targets** (Claude Code): direct symlinks, no transformation
- **Flat file generation** (Cursor): concatenate relevant rules into `.cursorrules`
- **Path-specific generation** (Copilot): write to `.github/copilot-instructions.md`
- **Custom formats**: extensible for future tools

Generation runs automatically when governance changes (file watcher
triggers regeneration) or on demand.

## Ongoing Maintenance: Drift Detection and Reconciliation

The governance hub model only works if sync is maintained. Three
mechanisms work together: prevention, detection, and reconciliation.

### Prevention (Pre-Commit Hook)

A git pre-commit hook detects direct edits to distribution targets:

```text
On commit:
  1. Check for modified distribution targets (.claude/, .cursorrules, etc.)
  2. If modified:
     a. Is the matching change also in .orqa/? → Allow (proper workflow)
     b. Is the change ONLY in the target? → Warn:
        "⚠ .cursorrules modified directly. Open OrqaStudio to
         reconcile before committing."
     c. Block commit until reconciled, or committer uses --force
```

Symlinked files (`.claude/rules/`) don't have this problem — editing
the symlink target IS editing `.orqa/`. This only applies to generated
files that are copies in a different format.

### Detection (File Watcher — Bidirectional)

The file watcher (already built for `.orqa/` → UI refresh) extends to
watch distribution targets too:

| Watch Direction | Trigger | Action |
| ---------------- | --------- | -------- |
| `.orqa/` changes | Rule/skill/agent edited in OrqaStudio | Regenerate all distribution targets |
| Distribution target changes | Someone edits `.cursorrules` directly | Flag drift in UI |

On drift detection, the app computes the delta between what the
distribution target SHOULD contain (generated from `.orqa/`) and
what it ACTUALLY contains.

### Reconciliation (App UI)

When drift is detected, the app surfaces it — no CLI needed:

**Status bar notification:**
> ".cursorrules was modified outside OrqaStudio — 2 rules added"
> [Review Changes]

**Reconciliation dialog** (click through):

- Shows diff: expected (generated from `.orqa/`) vs actual (on disk)
- Each change block gets three options:
  - **Adopt** — pull into `.orqa/rules/` as a new rule, regenerate all targets
  - **Discard** — revert to the generated version
  - **Defer** — leave the drift for now, remind later
- After resolution: all distribution targets regenerated from `.orqa/`
- Adopted rules appear in the governance browser immediately

### Why Not Just Block Direct Edits?

Hard-blocking would break the team workflow. A developer using Cursor
who discovers a useful rule mid-session shouldn't have to stop, open
OrqaStudio, create the rule there, wait for regeneration, then continue.
They should be able to add it where they are. The system catches it,
offers to absorb it, and keeps everything in sync.

The philosophy: **make the right path easy, make the wrong path
recoverable, never make it destructive.**

### Product Positioning

OrqaStudio is an **AI-assisted clarity engine** — it helps people turn
messy situations into structured understanding and evolving plans. That
core identity applies to any domain: software, consulting, research,
operations, anything.

The governance hub capability is a **natural consequence** of that core
applied to projects that involve AI tooling. When a project uses Claude
Code, Cursor, Copilot, or other AI tools, OrqaStudio's structured
governance can distribute to those tools and keep them in sync. But this
is one application of the clarity engine, not its identity.

A research project with no AI tooling gets full value from OrqaStudio.
A solo product manager planning a strategy gets full value. The
distribution and coexistence features are simply irrelevant for those
use cases — they activate when the context calls for them.

> OrqaStudio helps you think clearly and govern effectively.
> When your project involves AI tools, it also keeps them aligned.

## Consequences

1. **No `/templates/` directory** — skills replace templates entirely
2. **Skills are the single source of truth** for what a project needs
3. **New project types** are added as new skills, not app code changes
4. **The orchestrator stays lean** — Section 1 is universal core identity,

   Section 2 is generated from the project profile

5. **Existing projects are respected** — inference reads before writing
6. **Existing AI governance is migrated, not discarded** — the migration

   skill maps existing rules/instructions to OrqaStudio equivalents

7. **Permanent coexistence** — OrqaStudio distributes governance to other

   tools via symlinks and generated config. Not a transitional state.

8. **Team-heterogeneous** — different team members can use different AI

   tools while sharing the same governance source of truth

9. **The setup conversation is structured** — not freeform, guided by skills
10. **OrqaStudio's own governance is a reference implementation** — the

    most complex example of what the setup skills produce

## Documentation Requirements

This decision reshapes OrqaStudio's product identity. The product
documentation MUST be updated to reflect the governance hub philosophy
as the central narrative — not as a technical implementation detail
but as the core value proposition:

- **`vision.md`** — Update to position OrqaStudio as a governance hub

  that distributes to other tools, not a replacement for them. The
  "additive layer" philosophy and "manage the governance your tools
  consume" positioning must be prominent.

- **`governance.md`** — Update with the governance concept taxonomy

  (agent, skill, rule, hook, lesson) and the agent vs skill decision
  framework from [PD-48b310f9](PD-48b310f9).

- **`artifact-framework.md`** — Already updated with concept taxonomy

  and universal roles. Verify alignment with this decision.

- **New: `governance-hub.md`** — Dedicated page explaining the

  distribution model, coexistence architecture, drift reconciliation,
  and team workflow in detail. This is the key product differentiator
  and deserves its own page.

This documentation work is a prerequisite for implementation, not a
follow-up. The docs define what we build.

## Relationship to [PD-48b310f9](PD-48b310f9)

[PD-48b310f9](PD-48b310f9) established universal agent roles and the concept taxonomy.
This decision extends that by defining how those universal elements
are installed into a project. The setup skills know about the 7
universal roles and install the appropriate orchestrator configuration.

## Open Questions

1. How do type skills compose? Can a project be "software + research"?

   Probably yes — skills layer additively. But conflict resolution
   between type skills needs design.

2. Should inference run periodically (detect new languages added) or

   only at init? Probably only at init, with manual re-scan available.

3. How much of the collaborative custom setup can be automated vs

   requires human judgment? Start conservative (ask more), learn
   which questions can be inferred over time.

## Rationale

Rationale documented at time of decision.

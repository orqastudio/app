---
id: EPIC-053
title: Graph-Based Knowledge Injection
description: |
  Replace content injection with graph injection. The orchestrator prompt becomes
  minimal — just "here's how to read and extend the artifact graph." Agents receive
  the relationship map and read nodes on demand. The plugin architecture layers on
  top, making the system self-building. Web search parity ensures research and
  skills can be informed by external knowledge.
status: done
priority: P1
created: "2026-03-12"
updated: "2026-03-12"
deadline: null
milestone: MS-002
horizon: null
pillars:
  - PILLAR-001
  - PILLAR-002
depends-on:
  - EPIC-050
  - EPIC-052
blocks: []
research-refs: []
docs-required: []
docs-produced: []
scoring:
  dogfood-value: 5
  user-facing: 4
  foundation: 5
  complexity: 3
  score: 4.5
---

## Context

OrqaStudio's governance system has matured through several phases:

- **[EPIC-050](EPIC-050)**: Built the enforcement engine and companion plugin — rules are
  mechanically enforced via hooks, the plugin bridges `.orqa/` to Claude Code
- **[EPIC-052](EPIC-052)**: Shifted enforcement to structured thinking process gates, knowledge
  injection, and tooling ecosystem management
- **[EPIC-051](EPIC-051)**: Abstracted tool names into capabilities for provider-agnostic agents

But the orchestrator prompt (`CLAUDE.md` / `orchestrator.md`) is still massive — it
contains the full rule set, all skill injection tables, all delegation protocols, all
process gates. Every rule is loaded into every session whether relevant or not.

**The insight**: The artifact system is already a graph. Epics reference milestones,
tasks reference epics, research feeds into epics, rules reference related rules,
skills declare scope. If we inject the graph itself — the nodes and edges — instead
of all the content, agents can navigate to what they need on demand.

### The Shift

| Current | Target |
|---------|--------|
| Inject all rules into every prompt | Inject the graph + "read relevant nodes" |
| Orchestrator prompt has all process logic | Orchestrator prompt has graph navigation |
| Skills injected by hardcoded path tables | Skills discovered via graph relationships |
| Tasks don't link to their docs | Tasks carry `docs` edges → documentation loaded at impl time |
| Plugin adds enforcement on top | Plugin reads graph, extends graph — self-building |
| No external knowledge sources | Web search informs research docs and skills creation |

### Self-Building System

Once the plugin architecture sits on top of the graph prompt:

1. Plugin reads the graph to know what context to inject
2. Agent works with that context, creates/modifies artifacts
3. New artifacts extend the graph with proper relationships
4. Next session's graph is richer → better context → better output
5. The system improves itself through normal use

### Web Search Parity

Claude Code has `WebSearch` and `WebFetch` tools. These should inform:
- Research documents (investigating external solutions, library docs)
- Skills creation (pulling best practices from authoritative sources)
- Architecture decisions (comparing approaches documented online)

Currently these tools are available but not systematically connected to the artifact
workflow. The graph makes this natural: research artifacts link to their sources,
skills link to their references.

## Core Graph as Firmware

The artifacts that define graph relationships and traversal are **firmware** — they
ship with OrqaStudio and are non-editable by users in the app. Project-specific
context layers on top.

| Layer | Editable in App? | Updated By | Contains |
|-------|-----------------|-----------|----------|
| **Core graph** | No (read-only) | OrqaStudio update system | Artifact schemas, relationship definitions, traversal instructions, process model, role definitions, safety constraints |
| **Project context** | Yes | User/agents during normal use | Project-specific rules, skills, docs, tasks, epics, research, decisions |

**Exception**: Dogfood mode (`project.json` → `dogfood: true`) bypasses read-only
because the developer IS the update system.

The core graph artifacts must be **rock-solid, concise, and accurate**. They are the
foundation everything else builds on. When the app gets editing capabilities, these
are protected — only updated through a proper versioned release process.

### What's Core (Firmware)

1. **Artifact type schemas** — `schema.json` in each artifact directory
2. **Edge vocabulary** — what relationship fields exist and what they mean
3. **Traversal instructions** — how to navigate the graph (the orchestrator prompt)
4. **Process model** — understand → plan → document → implement → review → learn
5. **Role definitions** — the 7 universal roles and ownership boundaries
6. **Safety constraints** — NON-NEGOTIABLE rules (no unwrap, no --no-verify, etc.)
7. **Core skills** — composability, research-methodology, planning, code-search

### What's Project (User-Editable)

Everything else: project rules, project skills, documentation, planning artifacts,
governance decisions, lessons. These are created and extended through normal use.

### Git Hook Enforcement

The pre-commit hook blocks commits that modify core graph artifacts without a
`--force` flag. The hook:

1. Checks staged files against a list of core artifact paths
2. If core files are modified, blocks the commit with a warning:
   "You are modifying core graph artifacts. These define the system's thinking
   structure. Changes risk breaking graph traversal for all projects.
   Use `git commit --force-core` or set `ORQA_CORE_OVERRIDE=1` to proceed."
3. Dogfood mode (`project.json` → `dogfood: true`) skips this check
4. Core artifact paths include: `schema.json` files in artifact directories,
   the orchestrator prompt, core skill definitions, role definitions

## Implementation Design

### The Graph Model

```
Artifact Graph (static data — nodes + edges):

  Nodes: Every .orqa/ markdown file with YAML frontmatter
  Edges: Relationships declared in frontmatter fields

  Edge types:
    task.epic        → parent epic
    task.depends-on  → blocking tasks
    task.docs        → documentation to load during implementation (NEW)
    task.skills      → skills to load during implementation (NEW — replaces injection table)
    epic.milestone   → parent milestone
    epic.depends-on  → blocking epics
    epic.research-refs → informing research
    epic.docs-required → prerequisite documentation
    epic.docs-produced → output documentation
    rule.related-rules → related rules (via "Related Rules" section links)
    skill.scope      → which agents use this skill
    decision.supersedes → decision chain
    lesson.promoted-to → promotion target
```

### Graph Injection (Orchestrator Prompt)

The orchestrator prompt shrinks dramatically. Instead of encoding all rules, it
becomes:

```markdown
# How to Work

You have access to the artifact graph at `.orqa/`. It is a set of markdown files
with YAML frontmatter that define nodes and relationships.

## Reading the Graph

1. Start from the task or request
2. Follow edges to load context:
   - task.docs → read these docs before implementing
   - task.skills → load these skills
   - task.epic → read the epic for design context
   - epic.docs-required → prerequisite docs
   - epic.research-refs → background research
3. Check applicable rules: read `.orqa/process/rules/` for active rules
   matching your scope
4. Load skills matching the file paths you'll touch (see skill `scope` fields)

## Extending the Graph

When you create or modify artifacts:
1. Add proper frontmatter with all required fields
2. Set relationship fields (epic, depends-on, docs, skills)
3. Cross-reference related artifacts in the body
4. The graph gets richer with every artifact — future sessions benefit

## Process

understand → plan → document → implement → review → learn

Each step: read relevant graph nodes first, then act, then update the graph.
```

This replaces the current ~2000-line orchestrator prompt with ~50 lines + the graph
itself.

### Task `docs` Field (Schema Change)

Add `docs` to task schema:

```yaml
docs:
  - ".orqa/documentation/architecture/ipc-commands.md"
  - ".orqa/documentation/development/coding-standards.md"
```

These are documentation files that should be loaded into agent context when
implementing this task. Backfilled from git history for existing tasks.

### Task `skills` Field (Schema Change)

Add `skills` to task schema:

```yaml
skills:
  - "orqa-ipc-patterns"
  - "orqa-error-composition"
```

Replaces the hardcoded Tier 2 injection table in the orchestrator. Skills are now
graph edges on the task itself.

### Plugin Self-Building Loop

The companion plugin (EPIC-050) becomes the primary consumer of the graph:

1. **SessionStart hook** reads the graph, determines what to inject based on:
   - Current task (if set in session state)
   - File paths being touched (from git status)
   - User prompt (from intent classification)
2. **PreToolUse hook** reads rules from the graph for enforcement
3. **PostToolUse hook** can update the graph (add edges when artifacts are created)
4. **Stop hook** prompts for graph maintenance (missing edges, orphaned artifacts)

The plugin doesn't hardcode injection tables — it reads them from the graph.
When someone adds a new skill with the right `scope` field, it's automatically
discoverable. When someone adds `docs` to a task, those docs are automatically
loaded. The system builds itself.

### Web Search Integration

Add web search as a first-class tool in the artifact workflow:

1. **Research documents** can cite web sources with URLs
2. **Skills** can reference external documentation
3. **WebSearch/WebFetch** tools are available to all agents that need them
4. **Capability mapping** already includes `web_fetch` and `web_search` (RULE-040)

The graph connects external knowledge to internal artifacts:
```yaml
# In a research document
sources:
  - url: "https://docs.rs/tauri/latest/"
    description: "Tauri v2 official docs"
  - url: "https://svelte.dev/docs/svelte/runes"
    description: "Svelte 5 runes reference"
```

### Orchestrator Simplification Path

The current orchestrator.md (`CLAUDE.md`) is ~2000 lines. The target:

| Current Section | Target |
|----------------|--------|
| Universal Orchestration (~200 lines) | Keep — core process, shrink to ~50 lines |
| 40 rules inlined | Remove — agents read from graph |
| Skill injection tables | Remove — tasks carry `skills` edges |
| Delegation protocol details | Simplify — "follow the graph" |
| Project-specific requirements | Move to project.json or graph metadata |
| Dev server lifecycle | Move to a skill or doc |
| Worktree lifecycle | Move to a skill or doc |
| Hooks configuration | Move to plugin config |

Estimated reduction: ~2000 lines → ~200 lines.

## Tasks

### Schema & Graph Foundation
- [ ] [TASK-208](TASK-208): Add `docs` and `skills` fields to task schema
- [ ] [TASK-209](TASK-209): Backfill `docs` field on existing tasks from git history context
- [ ] [TASK-210](TASK-210): Backfill `skills` field on existing tasks from orchestrator injection table

### Orchestrator Simplification
- [ ] [TASK-211](TASK-211): Write graph-based orchestrator prompt (target: ~200 lines)
- [ ] [TASK-212](TASK-212): Extract dev server/worktree/hooks content to skills or docs
- [ ] [TASK-213](TASK-213): Extract project-specific requirements to graph-readable artifacts

### Plugin Graph Integration
- [ ] [TASK-214](TASK-214): Plugin reads task `docs` and `skills` fields for context injection
- [ ] [TASK-215](TASK-215): Plugin reads skill `scope` fields for path-based injection
- [ ] [TASK-216](TASK-216): Plugin extends graph on artifact creation (PostToolUse)

### Web Search Parity
- [ ] [TASK-217](TASK-217): Add `web_fetch` and `web_search` capabilities to research/planner agents
- [ ] [TASK-218](TASK-218): Add `sources` field to research schema for web references

### Core Protection
- [ ] [TASK-221](TASK-221): Pre-commit hook blocks core graph artifact modifications

### Decision Audit
- [ ] [TASK-222](TASK-222): Audit existing architecture decisions (AD-001 to AD-037) against [AD-038](AD-038)/039/040

### Project Setup Integration
- [ ] [TASK-223](TASK-223): Create skill for epic-requirement inference during project setup

### Verification
- [ ] [TASK-219](TASK-219): End-to-end test: task with docs/skills → plugin injects correct context
- [ ] [TASK-220](TASK-220): Verify orchestrator prompt works at ~200 lines (dogfood session)

## Out of Scope

- App-native graph query engine (future — app reads graph via Rust scanner)
- Visual graph explorer in the UI (future epic)
- Automated graph integrity checking (future — scanner job)
- Cross-project graph federation (future — multi-project support)

## The Virtuous Cycle

```
Better graph relationships
  → Plugin injects more relevant context
    → Agent produces better artifacts with proper relationships
      → Graph gets richer
        → Next session starts from a better position
```

This IS OrqaStudio's value proposition: turning messy situations into structured
understanding. The graph is the structure. The injection is the understanding.
The learning loop is the continuous improvement.

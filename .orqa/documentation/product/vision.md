---
id: DOC-046
title: Product Vision
description: Product vision defining OrqaStudio as an AI-assisted clarity engine built on two pillars.
created: "2026-03-02"
updated: "2026-03-07"
---

**Date:** 2026-03-07

## Mission

> OrqaStudio™ is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.

## Vision

OrqaStudio is an **opinionated clarity engine** designed to help people think clearly about complex situations and translate that clarity into structured action.

Rather than focusing primarily on building software or managing tasks, OrqaStudio focuses on **improving the quality of thinking that precedes action**. Projects, backlogs, and execution plans are outputs of improved understanding, not the primary goal.

The platform operationalises high-quality **agile thinking** through AI-assisted structured reasoning, enabling individuals and teams to explore problems, shape ideas, design experiments, and continuously learn through structured retrospection.

## Core Principles

**Clarity before execution** — Most tools optimise for productivity or output generation. OrqaStudio optimises for clarity of understanding, based on the belief that clear thinking leads to better action.

**Human-led AI** — AI does not replace human judgement. It acts as a structured thinking partner that helps users explore problems, organise complexity, surface assumptions, design experiments, and reflect on outcomes. Humans remain responsible for interpretation and decision-making.

**Agile as a thinking system** — Agile is often reduced to backlog management and sprint cycles. OrqaStudio restores the full agile learning loop, treating retrospection and learning as first-class elements of the system — not afterthoughts bolted onto delivery.

**Artifact-driven reasoning** — Markdown artifacts represent structured knowledge that can evolve over time. Plans, decisions, rules, retrospectives, and lessons are not ephemeral chat messages — they are living documents that accumulate into a knowledge base.

## Platform Identity

**OrqaStudio is not an AI development tool.** It is a clarity engine for structured thinking that happens to use AI as its reasoning partner. Software development is its first domain, but the product's identity is rooted in the principled approach to structured thinking — not in code generation, agentic coding workflows, or developer productivity.

When applied to projects that involve AI tooling (Claude Code, Cursor, Copilot, etc.), OrqaStudio can act as a governance hub — distributing rules and standards to those tools and keeping them in sync. But this is one capability that activates when the context calls for it, not the product's core identity. A research project with no AI tooling, a solo product manager planning a strategy, or a consulting engagement with no code at all — all get full value from OrqaStudio's structured thinking framework.

The distinction matters for every product decision: features are evaluated against "does this improve structured thinking?" — not "does this make AI-assisted coding better?"

OrqaStudio sits at the intersection of several categories but is not limited to any of them:

| Category | Relationship |
|----------|-------------|
| Productivity tools | Provides structured artifacts |
| AI assistants | Offers conversational reasoning |
| Project tools | Generates execution frameworks |
| Knowledge systems | Maintains evolving artifacts |
| Learning systems | Captures and analyses iteration history |

In essence, OrqaStudio becomes **cognitive infrastructure for structured reasoning and action**.

## The Agile Learning Loop

The core cycle that OrqaStudio operationalises:

```
Chaos / Input
  --> Structured Understanding
  --> Experiments / Backlog
  --> Execution
  --> Retrospective
  --> Improved Understanding
```

Every feature in OrqaStudio serves this loop:

1. **Chaos → Structured Understanding** — AI-assisted conversations help users explore ambiguous problems, challenge assumptions, and build structured models of what they're dealing with
2. **Structured Understanding → Experiments** — Understanding produces actionable plans: experiments to run, tasks to execute, hypotheses to test
3. **Experiments → Execution** — Plans are executed with AI assistance, with governance ensuring quality and standards
4. **Execution → Retrospective** — Outcomes are reviewed. What worked? What didn't? What was learned?
5. **Retrospective → Improved Understanding** — Lessons feed back into the knowledge base. The next cycle starts from a better position than the last.

The loop is domain-agnostic. Software development is OrqaStudio's first domain — where the loop manifests as code governance, implementation cycles, and learning from agent sessions. But the underlying pattern applies wherever complex work requires structured thinking: product strategy, research, operations, consulting, and beyond.

---

## Core Interaction Model

The platform blends three interaction layers:

### Artifact Layer

Human-readable markdown artifacts represent the current state of understanding. Examples: context analysis, backlog, strategy outputs, retrospectives. Artifacts are the **canonical state of knowledge**.

Key properties:
- Human-readable
- Reusable in prompts (artifacts feed directly into AI reasoning)
- Composable across workflows
- Portable outside the system (standard markdown, not proprietary formats)

Artifacts serve as both outputs and inputs to subsequent reasoning cycles.

### Insight Layer

Dashboards and views aggregate insights across artifacts:
- Progress indicators
- Change summaries
- Iteration insights
- Retrospective analysis

These views help users observe patterns and evolution across their work.

### Reasoning Layer

A persistent chat panel acts as a **project-aware thinking partner**, allowing users to ask questions, trigger workflows, interpret artifacts, and explore alternatives. The conversation persists across views, maintaining **cognitive continuity** — the user's thinking context is not lost when switching between artifacts and dashboards.

---

## Five-Layer Enforcement Architecture

The two pillars define *what* OrqaStudio achieves. The five-layer enforcement architecture is *how* it achieves them. This is the mechanism that makes OrqaStudio opinionated rather than generic — the philosophy is not just described in documentation, it is encoded in executable governance artifacts that agents follow.

### Layer 1: Core (App-Managed Framework)

The core layer is the opinionated agentic framework shipped with OrqaStudio. It encodes systems thinking and agile planning philosophy that is the product's core value proposition. Core artifacts are installed into `.orqa/` on project setup and updated centrally when the app updates.

Core artifacts are **not user-editable**. They include a warning comment at the top of each file discouraging manual modification. When the app updates, core artifacts are overwritten with the latest version.

What core encodes:
- How to decompose problems into manageable, structured thinking
- How to plan iteratively with explicit approval gates
- How to track lessons and promote patterns into governance
- How to enforce quality standards across agents
- How to reflect on outcomes and feed improvements back into the next cycle

The core layer is identical across every project and every industry. A software development project, a healthcare planning initiative, and a personal goal-tracking project all get the same core framework. This is the "opinionated" part: OrqaStudio has a point of view about how structured thinking works, and it ships that point of view as enforceable artifacts.

### Layer 2: Project (User-Managed)

The project layer is where users customise the framework for their specific context. Project artifacts are layered on top of core and are fully editable by the user and AI.

Project artifacts encode:
- Domain-specific rules and conventions (what "done" means for this project)
- Custom coding standards, naming conventions, or workflow patterns
- Industry-specific terminology and artifact types
- Project-specific agent configurations and skill sets

The project layer is **additive only**. Project artifacts can extend the core framework — adding new rules, new agent behaviours, new artifact types — but they cannot override or weaken core rules. If a core rule says "all errors must be handled as Result types," no project rule can override that constraint. The core layer defines the floor; the project layer raises it.

### Layer 3: Plugin (1st Party Official Extensions)

The plugin layer extends both the artifact system and the display system with officially maintained extensions. Plugins allow the framework to serve radically different use cases — from software development to personal life management — without changing the underlying structure.

Plugins can:
- **Register new artifact types** — Define new types of structured knowledge beyond the built-in set (milestones, epics, tasks, ideas, lessons)
- **Register artifact groups** — Organise multiple artifact types into a named section in the UI
- **Provide custom display views** — Gantt charts, kanban boards, graph visualisations, timelines, custom dashboards

The artifact configuration in `project.json` drives what the app shows and scans:

```json
{
  "artifacts": [
    { "key": "docs", "label": "Documentation", "icon": "file-text", "path": ".orqa/documentation" },
    { "key": "planning", "label": "Planning", "icon": "target",
      "children": [
        { "key": "ideas", "label": "Ideas", "path": ".orqa/delivery/ideas" },
        { "key": "milestones", "label": "Milestones", "path": ".orqa/delivery/milestones" }
      ]
    }
  ]
}
```

Each entry is either a direct type (has a `path`) or a group (has `children`). The app requires certain keys to exist — the core types (`lessons`, `rules`, `decisions`, `milestones`, `epics`, `tasks`, `ideas`, `agents`) — but their paths, labels, icons, and grouping are configurable. A plugin can rename, regroup, and display these types differently without changing what they are.

### Layer 4: Community (Community-Contributed)

The community layer consists of publicly shared extensions contributed by the OrqaStudio user community. Community artifacts extend the framework with domain-specific knowledge, workflow patterns, and display views that have been built and shared by other users. They follow the same structure as official plugins but are maintained by their contributors rather than the OrqaStudio team.

### Layer 5: User (Personal Workflows)

The user layer is where individuals build and maintain their own local extensions. User artifacts encode personal workflow patterns, custom artifact types, and private views that are not intended for sharing. They are the most dynamic layer — built through conversations with the AI and iterated on freely without any publishing or review process.

### The Core Thesis

Systems thinking and agile project planning can be applied to any industry and any problem — if the underlying framework is sound and the principles are consistently followed. OrqaStudio automates doing it properly through the core layer and enforces those principles so they cannot be shortcut. How users want to interact with the artifacts and use the framework is configurable through the project, plugin, community, and user layers. This enables software development, healthcare, consulting, personal planning, and any other domain where complex work benefits from structured thinking.

The framework that produces structured outcomes is not optional or configurable. How users experience and extend that framework is.

### The Self-Extension Principle

OrqaStudio can build anything — including extensions of itself. A plugin is just a project, and OrqaStudio is a tool for turning messy ideas into structured, working output. When a user needs a capability the app doesn't have, they don't file a feature request and wait — they describe what they need, and the AI helps them build it as a plugin using the same structured thinking process the app provides for any other problem.

This creates a flywheel: the app teaches structured thinking → the user applies structured thinking to extend the app → the extension makes the app more capable → the more capable app enables more ambitious extensions. The five layers reflect this progression:

| Layer | What | Who |
|-------|------|-----|
| **Core** | Platform principles and framework shipped with the app | OrqaStudio team |
| **Plugin** | Curated extensions in the official repository | OrqaStudio team |
| **Community** | Publicly shared extensions | Anyone |
| **User** | Local extensions built with the AI | The individual user |

The user layer is the most important. The barrier from "I had an idea" to "I have a working plugin" is a single conversation with the AI. The barrier from "working locally" to "shared with the world" is a git push. This means OrqaStudio can solve ANY problem and build ANY useful tool — not because the app ships with every possible feature, but because it ships with the ability to help users build whatever they need through structured thinking.

---

## Entry Modes

Users arrive at OrqaStudio from different starting points. Not everyone enters with a clear task — many enter with ambiguity. The platform meets them where they are:

| Mode | Starting point | What the user needs |
|------|---------------|-------------------|
| **Problem** | Something is not working and needs diagnosis | Understand root causes, map the situation, produce a fix plan |
| **Idea** | A concept needs validation and shaping | Explore feasibility, challenge assumptions, shape into something actionable |
| **Goal** | A desired outcome requires planning | Gap analysis, breakdown into steps, produce a backlog or experiment plan |
| **Chaos** | A messy situation needs clarity | Triage, find structure in the mess, identify what matters and what to do first |

Each entry mode supports both **new projects** (starting from scratch) and **existing work** (adapting a codebase, asset library, research corpus, or any body of work the user is already engaged with). The AI-assisted onboarding flow helps the user assess what they have, understand where they are, and create a structured thinking process around it.

Each mode triggers a different onboarding flow — different questions, different reasoning patterns, different initial artifact structures. But all four paths converge into the same agile learning loop: structured understanding → experiments → execution → retrospective → improved understanding.

The entry modes are how agile thinking is operationalised at the point of entry. The user doesn't need to know agile terminology or frameworks. They just describe their situation, and OrqaStudio guides them from wherever they are toward structured reasoning.

While software development is OrqaStudio's first domain — where implementation can be directly integrated into the structured thinking process — the entry modes are domain-agnostic by design. The same onboarding patterns apply to personal life management, healthcare project management, research planning, consulting engagements, or any situation where complex work benefits from structured thinking.

```
Problem ──┐
Idea ─────┤
Goal ─────┼──→ Structured Understanding ──→ Agile Learning Loop
Chaos ────┘
```

---

## Projects as Thinking Environments

Projects act as **bounded reasoning environments** where ideas, problems, and goals are explored. Each project contains:

- Artifacts (the current state of structured understanding)
- Reasoning history (conversations and decisions)
- Backlog (actionable items emerging from understanding)
- Retrospectives (structured reflection on outcomes)
- AI-assisted analysis (insights derived from the above)

Projects can exist for personal thinking, team initiatives, or organisational programs. The project is the unit of structured reasoning — not a task list or a repository, but a container for evolving understanding.

---

## Historical Memory

All artifact changes are tracked in a SQLite history layer. This enables auditability, change tracking, rollback, and retrospective analysis. Over time, this history becomes a **learning dataset** — allowing OrqaStudio to analyse how thinking evolves across iterations, surface patterns in decision-making, and detect strategic drift.

---

## Reflective Intelligence

OrqaStudio does not only generate outputs; it helps users reflect on their thinking. Historical insight enables features like iteration summaries, assumption change tracking, decision evolution analysis, and retrospective guidance. This creates **AI-assisted reflective practice**, reinforcing the learning aspect of agile thinking.

---

## Product Pillars

> **Source of truth:** Active pillars are defined as structured artifacts in `.orqa/process/pillars/PILLAR-NNN.md`. The narrative below provides context; the pillar artifacts are authoritative for gate questions and conflict resolution.

### Pillar 1: Clarity Through Structure

Making thinking visible, structured, and evolvable.

When knowledge lives only in conversations and people's heads, it is fragile — lost between sessions, invisible to collaborators, impossible to build upon systematically. OrqaStudio makes the invisible tangible:

- **Governance as visible structure** — Rules, standards, architecture decisions, and agent definitions are not hidden configuration. They are browsable, editable, first-class artifacts in the UI.
- **Artifact-driven knowledge** — Conversations produce structured markdown artifacts (plans, decisions, retrospectives) that persist and evolve across sessions
- **Process visibility** — Scanner dashboards, compliance status, and quality metrics surface what would otherwise be buried in terminal output and dotfiles
- **Documentation-first workflow** — The system enforces document → approve → implement → verify, ensuring understanding precedes action

Clarity Through Structure answers the question: *"Can I see and understand the current state of this work, its standards, and its decisions — without asking someone or reading raw files?"*

### Pillar 2: Learning Through Reflection

The system and its users improve over time through structured retrospection.

Most tools accumulate conversation history. OrqaStudio accumulates *understanding*. Every cycle of work produces not just output but insight that feeds the next cycle:

- **Lesson capture** — Mistakes and discoveries are documented with recurrence tracking, not just mentioned in chat
- **Pattern promotion** — When a lesson recurs, it is promoted to a rule, coding standard, or skill update — the governance framework evolves automatically
- **Retrospectives** — Process-level observations become entries that inform future governance evolution
- **Metrics and trends** — Pass/fail rates, quality trends, and violation recurrence are visualised over time, making improvement (or regression) visible
- **Session continuity** — Handoff notes and searchable session history prevent context loss between sessions, so each session builds on the last

Learning Through Reflection answers the question: *"Is this work getting better over time, and can I see the evidence?"*

### Pillar Relationship

The two pillars are complementary and deeply intertwined:

- **Clarity Through Structure** provides the foundation — you cannot improve a process that isn't visible and structured
- **Learning Through Reflection** operates on that foundation — structured artifacts can be measured, compared, and evolved

The pillars are equal in importance. When they appear to conflict, the conflict is flagged to the user for resolution rather than one pillar automatically taking precedence. See the pillar artifacts in `.orqa/process/pillars/` for authoritative conflict resolution guidance.

---

## Primary Users

### Deep Users

Product managers, project managers, founders, program leads, tech leads, and anyone who needs to turn complex situations into structured understanding and evolving plans. For them, OrqaStudio becomes a **daily workspace for structured thinking and direction**. They want to:

- Use AI as a thinking partner to explore problems and shape ideas before committing to action
- Build up a structured knowledge base of decisions, plans, and lessons that persists across sessions
- Define standards and governance that are visible, enforceable, and improving over time
- Delegate execution to AI agents with confidence that process is followed
- Track quality and learning outcomes to know whether the work is getting better

A capable solo operator should be able to use OrqaStudio to build well-researched, well-considered outputs by defining the governance framework, delegating to agents, and reviewing results — while retaining oversight and approval authority.

See [Personas](personas.md) for detailed profiles: Alex (the Lead), Sam (the Practitioner), and Jordan (the Independent).

### Light Users

Subject matter experts, contributors, volunteers, and stakeholders. For them, OrqaStudio acts as a **thinking tool** for contributing insight or reviewing understanding. They engage with specific artifacts, provide input through structured flows, and benefit from clarity without needing to manage the full reasoning environment.

### The Key Insight

OrqaStudio is not a developer tool that happens to have process features. It is not a chat interface that happens to save history. It is a **clarity engine** — a tool for improving the quality of thinking that leads to action.

The difference matters: the UI, the workflow, and the default experience are designed for someone who thinks in terms of understanding, decisions, and standards — not someone who thinks in terms of code editors and terminal commands. Software development is the first domain because the governance infrastructure can be directly integrated into the development workflow, but the underlying value — structured thinking that improves over time — applies far beyond code.

---

## Problem

People use AI tools to help with complex work — software development, product planning, research, operations. But current AI tools operate as conversational interfaces with no structural layer. The thinking that happens in conversation is ephemeral: context is lost between sessions, standards are inconsistently applied, mistakes recur, and there is no mechanism for the process to improve over time.

For software development specifically, agentic coding tools like Claude Code are powerful but operate as developer-facing CLI tools with no product management layer. The governance framework that makes agentic development reliable — agents, skills, rules, learning loops, documentation-first workflow — lives in dotfiles, markdown documents, and terminal output. Product managers and tech leads are locked out of the implementation loop entirely.

The result across all domains: AI-assisted work produces inconsistent quality, accumulates decisions invisibly, and fails to learn from its own history.

## Solution

OrqaStudio is a desktop application that provides a structural layer for AI-assisted thinking and work.

**For any domain:** Conversations with AI produce structured artifacts — plans, decisions, retrospectives, and knowledge — that persist across sessions and evolve over time. The agile learning loop (chaos → clarity → execution → reflection → improved clarity) is operationalised through the UI.

**For software development (the first domain):** OrqaStudio integrates with Claude Code and other AI providers, making governance artifacts (agents, skills, rules, architecture decisions, learning loops) visible and manageable through a graphical interface. The system learns from every session and feeds improvements back into the governance framework automatically.

OrqaStudio turns ephemeral AI conversations into accumulated, structured, improving knowledge.

---

## Strategic Direction

OrqaStudio is best understood as:

> A system that helps people transform messy situations into clear, evolving plans.

It enables individuals and organisations to **learn faster than the problems they are trying to solve**.

---

## Dogfooding Principle

OrqaStudio is built using OrqaStudio alongside the Claude Code CLI. Once the MVP delivers a working conversation UI with Claude integration, the project transitions from terminal-only governance to using OrqaStudio's UI as the primary governance management layer — while the CLI remains available for all development tasks.

This is not optional — it is a foundational design constraint:

- **Every feature must be good enough for OrqaStudio's own team to use daily.** If a feature isn't useful for managing this project, it isn't useful for anyone.
- **Deficiencies discovered through self-use are highest-priority fixes.** The dogfooding loop is the primary driver of roadmap priority after the MVP.
- **OrqaStudio and the CLI coexist permanently.** The transition is from "invisible governance buried in dotfiles" to "visible governance through OrqaStudio's UI." See [Product Governance](governance.md) for transition criteria.

## AI Provider Integration

OrqaStudio is a framework for systems thinking and agile planning. What powers it under the hood is an implementation option.

The current implementation uses Claude (via the Agent SDK) as the AI provider, but the architecture is provider-agnostic by design. The Rust core speaks a neutral protocol; provider-specific logic lives in swappable sidecar processes. Future iterations should support integration with other AI providers and tools.

### External Tool Compatibility

OrqaStudio's governance artifacts live under `.orqa/` as standard markdown files. When a project involves AI development tools, OrqaStudio acts as a **governance hub** — distributing rules, agent definitions, and standards to whatever tools the team uses:

- **Same-format tools** (Claude Code CLI) — symlinks from `.claude/` to `.orqa/` paths
- **Different-format tools** (Cursor, Copilot) — generated config files (`.cursorrules`, `.github/copilot-instructions.md`) produced from `.orqa/` governance

Distribution is bidirectional: changes in OrqaStudio regenerate tool configs; direct edits to tool configs are detected and can be adopted back into `.orqa/`. See [AD-030](../../governance/decisions/[AD-030](AD-030).md) for the full governance hub architecture.

This is a contextual capability, not a requirement. If a user is not using any external AI tools, the distribution features are simply irrelevant — OrqaStudio handles all governance natively.

### Data Ownership

- **`.orqa/` is the source of truth** — All governance artifacts live under `.orqa/`. The directory structure is configurable via `project.json`, but `.orqa/` is the default root.
- **Bidirectional editing** — Users can edit governance artifacts in OrqaStudio, in a text editor, or through any compatible tool. OrqaStudio's file watcher detects external changes and reflects them in the UI.
- **No lock-in** — All governance artifacts are standard markdown files on disk. A user can stop using OrqaStudio and continue with any tool or text editor.
- **SQLite is a derived cache** — OrqaStudio's SQLite database stores session history, project metadata, and indexed artifact data. The `.orqa/` files on disk are the source of truth. If the database is deleted, OrqaStudio re-indexes from disk on next launch.

## Key Differentiators

1. **Opinionated clarity engine** — Not a neutral chat interface but a system with a point of view about how thinking should work. The five-layer enforcement architecture (core + project + plugin + community + user) means the philosophy is not just documented — it is enforced.
2. **Structured thinking, not just task execution** — Designed to improve the quality of thinking that leads to action, not just generate output faster
3. **Learning that compounds** — The system genuinely gets smarter over time through the reflection loop, not just accumulating conversation history
4. **Three-layer interaction model** — Artifacts (canonical knowledge), Insights (aggregated views), and Reasoning (persistent thinking partner) work together as cognitive infrastructure
5. **Five-layer enforcement architecture** — Core (app-managed, non-negotiable principles), Project (user-managed, additive extensions), Plugin (1st party official extensions), Community (community-contributed extensions), and User (personal workflows). The framework that produces structured outcomes is fixed; how users experience and extend it is configurable.
6. **Governance made tangible** — What was invisible (standards, rules, decisions, quality trends) becomes a first-class, visual, manageable layer
7. **Domain-agnostic core** — Software development is the first domain, but the clarity engine applies to any complex work: research, operations, consulting, personal management. The plugin layer is what makes domain adaptation practical.
8. **Provider-agnostic architecture** — The AI provider is an implementation option, not a product dependency. The architecture supports multiple providers through a composable sidecar interface
9. **Solo capability** — A single person can define standards, delegate to AI agents, and produce quality work with oversight but without dedicated team resource
10. **Dogfooding-driven design** — OrqaStudio is its own first customer, ensuring every feature is validated by real use before release

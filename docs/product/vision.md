# Product Vision

**Date:** 2026-03-02

## Problem

Agentic coding tools like Claude Code are powerful, but they operate as developer-facing CLI tools with no product management layer. The governance framework that makes agentic development reliable — agents, skills, rules, learning loops, documentation-first workflow — lives in dotfiles, markdown documents, and terminal output. There is no tool that lets a Product Manager or Tech Lead define the process, delegate implementation to AI agents, and verify the results through a managed cycle.

The result: AI-assisted development produces inconsistent quality, accumulates technical debt invisibly, and requires deep developer expertise to manage. Product managers are locked out of the implementation loop entirely, and solo technical PMs cannot leverage agentic tools without becoming full-time developers.

## Solution

Forge is a desktop application that automates product management and the agentic implementation cycle. It wraps Claude's capabilities with a visual process layer where governance artifacts — agents, skills, rules, architecture decisions, learning loops — live alongside the conversation as interactive, editable documents. The system learns from every session and feeds improvements back into the governance framework automatically.

Forge turns the invisible infrastructure of managed agentic development into a tangible, manageable product.

## Primary Users

### Product Managers & Tech Leads (Primary)

Technical product managers and tech leads who want to:

- Define product requirements, architecture, and standards through a structured UI
- Delegate implementation to AI agents with confidence that process governance is enforced
- Review and approve implementation plans, tool calls, and code changes through a visual interface
- Track quality metrics, scanner results, and learning loop outcomes over time
- Manage the full product lifecycle — from research through implementation to verification — without needing dedicated developer resource for every task

A capable solo technical PM should be able to use Forge to build well-researched, well-considered products by defining the governance framework, delegating to agents, and reviewing results — while retaining architecture oversight and code review authority.

### Developers (Secondary)

Developers who want structured, repeatable, improving processes for their AI-assisted work. Forge makes the governance layer visible and manageable rather than buried in dotfiles and terminal output.

### The Key Insight

Forge is not a developer tool that happens to have process features. It is a **product management tool** that automates the implementation cycle. The difference matters: the UI, the workflow, and the default experience are designed for someone who thinks in terms of product requirements, architecture decisions, and quality standards — not someone who thinks in terms of code editors and terminal commands.

## Product Pillars

### Pillar 1: Self-Learning Loop

The system improves over time. Every session contributes to a growing body of knowledge:

- **Lesson capture** — Implementation failures are documented as IMPL entries with recurrence tracking
- **Pattern promotion** — When a lesson recurs, it is automatically promoted to a rule, coding standard, or skill update
- **Retrospectives** — Process-level failures become RETRO entries that inform future governance evolution
- **Metrics tracking** — Pass/fail rates, coverage trends, violation recurrence are visualized over time
- **Session continuity** — Handoff notes and searchable session history prevent context loss between sessions

The learning loop is the core differentiator. A team using Forge for a month will have a fundamentally better governance framework than when they started — without manually writing any of it.

### Pillar 2: Process Governance

Standards, rules, and workflows are visible, enforceable, and manageable:

- **Rule enforcement** — Rules are not just documents; they are active constraints that agents follow and the UI surfaces
- **Agent management** — Agent definitions, skills, and tool assignments are browsable and editable
- **Scanner dashboards** — Quality scanners run automatically and their results are visualized as pass/fail trends
- **Architecture decisions** — AD records are tracked, cross-referenced, and compliance is verified in plans
- **Documentation-first workflow** — The system enforces document → approve → implement → verify

Governance is not a document collecting dust. It is a living, enforceable, visible layer of the development process.

## Dogfooding Principle

Forge is built using Forge. Once the MVP delivers a working conversation UI with Claude integration, the project transitions from the bootstrap Alvarez-derived process (CLI-based agents, markdown rules, terminal-only governance) to using Forge itself as the primary development management tool.

This is not optional — it is a foundational design constraint:

- **Every governance feature must be good enough for Forge's own team to use daily.** If a feature isn't useful for managing this project, it isn't useful for anyone.
- **Deficiencies discovered through self-use are highest-priority fixes.** The dogfooding loop is the primary driver of roadmap priority after the MVP.
- **The bootstrap process is temporary scaffolding**, not the long-term architecture. See [Product Governance](/product/governance) for transition criteria.

## Key Differentiators

1. **Product management, not developer tooling** — Designed for PMs and tech leads who define process and review results, not just developers who write code
2. **Process visibility** — What was invisible (governance artifacts, scanner results, learning loops) becomes a first-class UI
3. **Automated governance backfill** — Point at an existing codebase, answer questions, and Forge builds the governance framework through conversation
4. **Continuous improvement** — The system genuinely gets smarter over time through the learning loop, not just accumulating conversation history
5. **Solo PM capability** — A technical PM can define product standards, delegate to agents, and ship software with architecture oversight but without dedicated developer resource
6. **Dogfooding-driven design** — Forge is its own first customer, ensuring every feature is validated by real use before release

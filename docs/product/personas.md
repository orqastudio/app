---
title: "User Personas"
category: product
tags: []
created: 2026-03-02
updated: 2026-03-04
---

# User Personas

**Date:** 2026-03-02

Detailed profiles of Orqa Studio's target users. These personas drive UI design, feature prioritization, and acceptance criteria throughout the product.

---

## Persona 1: The Technical PM (Primary)

**Name:** Alex — Product Manager / Tech Lead hybrid

### Profile

Alex is a senior product manager with a strong technical background. They can read code, understand architecture, and make informed technical decisions — but they don't write production code day-to-day. They manage a product or team of products and use AI-assisted development to move faster than would be possible with traditional engineering staffing.

### Demographics

- **Role:** Product Manager, Tech Lead, or both
- **Experience:** 5-15 years in technology, transitioned from engineering to product/leadership
- **Technical comfort:** Can read and review code in multiple languages. Understands version control, CI/CD, database schemas, and API design. Writes scripts and prototypes but not production systems.
- **Team context:** May lead a small team or work solo. Uses AI agents as a force multiplier.

### Goals

1. **Define product standards** — Establish the rules, architecture decisions, and quality standards that agents follow during implementation
2. **Delegate with confidence** — Hand tasks to AI agents knowing that governance is enforced and quality is verified
3. **Review efficiently** — Evaluate implementation plans and code changes without needing to understand every line — focus on intent, architecture, and standards compliance
4. **Track quality over time** — See trends in scanner results, review outcomes, and learning loop metrics to know whether the process is improving
5. **Ship consistently** — Maintain a predictable development cadence where each cycle builds on the last

### Pain Points

1. **Invisible governance** — Claude Code CLI's governance lives in `.claude/` dotfiles and terminal output. Alex can't see the governance framework without reading raw markdown files and CLI logs.
2. **No oversight layer** — There's no way to define standards and verify they're being followed without manually reviewing every output. The approval gate is missing.
3. **Context loss** — Every new conversation starts from scratch. Lessons learned in one session aren't automatically applied in the next.
4. **Cost unpredictability** — API-based pricing makes costs hard to forecast. A productive day of agentic development can cost hundreds of dollars.
5. **Tool complexity** — Current agentic development requires deep CLI expertise. The barrier to entry is "be a developer who knows terminal workflows."

### Typical Workflow

1. Review current project status (open tasks, recent sessions, scanner results)
2. Define a new requirement or refine an existing one
3. Review and approve the implementation plan proposed by the AI
4. Monitor progress as agents implement
5. Review tool calls and code changes through the UI
6. Approve or request changes at the review gate
7. Check quality metrics and learning loop outcomes

### What Success Looks Like

Alex can manage a product's entire development cycle — from requirement to shipped feature — through Orqa Studio's UI. They define the standards, Claude Code agents implement within those standards, and the system gets smarter over time. Alex spends their time on product decisions and architecture review, not on terminal commands and dotfile management.

### Relationship to Claude Code CLI

Alex uses Orqa Studio as their primary interface. The governance artifacts they create and edit through Orqa Studio's UI are native `.claude/` files — meaning Claude Code agents running in the CLI also follow them. Alex benefits from CLI-compatible artifacts without needing to manage the CLI directly.

### Key Design Implications

- **UI must be approachable** — No terminal commands required for core workflows
- **Governance is first-class** — Artifacts are browsable, editable, and visualized, not hidden in files
- **Plans before code** — Every implementation starts with a reviewable plan
- **Cost transparency** — Usage is predictable (Max subscription) with clear visibility
- **Progressive complexity** — Advanced features (MCP extensibility, custom scanners) appear as Alex grows into them

---

## Persona 2: The Developer (Secondary)

**Name:** Sam — Senior Developer

### Profile

Sam is a senior software developer who already uses Claude Code CLI and other AI coding tools (Copilot, Cursor) daily. They write production code and are comfortable in the terminal. They value Orqa Studio as a visual layer that makes the Claude Code CLI's governance visible and manageable — seeing rule enforcement trends, browsing agent definitions, and tracking learning loop metrics through a UI rather than reading raw dotfiles.

### Demographics

- **Role:** Senior Developer, Staff Engineer
- **Experience:** 5-10+ years in software development
- **Technical comfort:** Expert. Writes code daily, manages build systems, debugs complex issues, reviews PRs.
- **Team context:** Works on a team with defined standards. May also define those standards.

### Goals

1. **Structured process** — Replace ad-hoc AI conversations with a repeatable workflow that enforces project standards
2. **Visibility into governance** — See what rules, agents, and skills are active without reading raw markdown files
3. **Learning loop value** — Have the system actually learn from mistakes and improve over time, not just accumulate chat history
4. **Session continuity** — Resume work across sessions without losing context
5. **Extensibility** — Connect additional MCP servers, customize tools, adapt the system to their specific workflow

### Pain Points

1. **Repeated mistakes** — AI agents make the same errors across sessions because there's no mechanism to learn from past failures
2. **Inconsistent quality** — Without enforced standards, the same task produces different quality depending on the session
3. **No audit trail** — Can't look back at what agents did, what was approved, and what the outcomes were
4. **Fragmented tools** — Using separate tools for coding, review, documentation, and process tracking with no integration between them
5. **Process overhead** — Manual governance management (editing agent files, maintaining rules, tracking lessons) takes time away from coding

### Typical Workflow

1. Open a project, review pending tasks and recent session history
2. Start a new session for a specific task
3. Converse with the AI, reviewing tool calls and approving operations
4. Let the agent iterate on implementation based on review feedback
5. Run scanners to verify compliance before considering the task done
6. Capture any lessons learned through the interface

### What Success Looks Like

Sam's AI-assisted development produces consistently high-quality results because the governance framework is active, not passive. Mistakes from last week don't recur this week. Standards are enforced automatically. The process improves without Sam manually editing rule files.

### Relationship to Claude Code CLI

Sam uses both Orqa Studio and the Claude Code CLI interchangeably. As a power user of the CLI, Sam appreciates that Orqa Studio operates on the same `.claude/` artifacts — governance changes made in Orqa Studio are immediately effective in CLI sessions, and vice versa. Sam may prefer the CLI for coding and Orqa Studio for governance visibility.

### Key Design Implications

- **Keyboard-first** — Power users expect keyboard shortcuts for common operations
- **Detail when needed** — Tool call cards expand to show full input/output, not just summaries
- **MCP extensibility** — Sam will want to connect project-specific tools
- **Transparent pipeline** — Sam wants to see exactly what's happening (streaming tokens, tool calls, sidecar activity)
- **No dumbing down** — The UI should be approachable for PMs but not at the expense of depth for developers

---

## Persona 3: The Solo Technical PM (Tertiary)

**Name:** Jordan — Solo Founder / Technical PM

### Profile

Jordan is building a product alone or with a very small team. They have enough technical skill to define architecture, review code, and make design decisions, but they rely heavily on AI agents for implementation. Jordan's constraint is time — they wear every hat (PM, architect, developer, QA) and need a tool that makes the most of AI assistance while maintaining quality.

### Demographics

- **Role:** Solo founder, indie developer, technical consultant
- **Experience:** 3-10 years, generalist background
- **Technical comfort:** Moderate to high. Can code but prefers to define and review rather than implement. Comfortable with frameworks but not low-level systems.
- **Team context:** Solo or tiny team (1-3 people). AI agents are the "engineering team."

### Goals

1. **Ship solo** — Build production-quality software with AI agents doing the heavy implementation work
2. **Maintain quality without a team** — Scanners, review gates, and learning loops substitute for a QA team and code review culture
3. **Governance as a force multiplier** — Define standards once, enforce them automatically across all future sessions
4. **Minimize ramp-up** — Get value from Orqa Studio within minutes of opening a project, not hours of configuration
5. **Cost control** — Flat-rate AI usage (Max subscription) so productivity isn't penalized by cost

### Pain Points

1. **Wearing every hat** — Quality suffers when one person is PM, architect, developer, and QA simultaneously
2. **No safety net** — Without a team, there's no one to catch mistakes that the AI makes
3. **Governance is overhead** — Maintaining process artifacts manually feels like bureaucracy when you're also writing the code
4. **Learning curve** — New tools need to deliver value immediately; elaborate setup is a dealbreaker
5. **Scaling yourself** — The gap between "I can define what I want" and "the code ships" is too wide without better tooling

### Typical Workflow

1. Open an existing project (or start a new one with automatic codebase scanning)
2. Define a feature requirement in conversation
3. Review and approve the implementation plan
4. Let agents implement while monitoring progress
5. Check scanner results for quality gate
6. Capture lessons, move to the next task
7. Review learning loop metrics weekly to ensure quality trend is positive

### What Success Looks Like

Jordan ships quality software at a pace that would normally require a small team. The governance framework they build up over time becomes a compounding advantage — each session is better than the last because the rules, lessons, and scanners accumulate. Jordan spends time on product decisions, not process management.

### Relationship to Claude Code CLI

Jordan may discover Orqa Studio after already using the Claude Code CLI, or may use Orqa Studio to bootstrap governance that is then available in CLI sessions. Either path works because Orqa Studio and the CLI share the same `.claude/` artifact format. Jordan benefits from Orqa Studio's visual governance management without being locked out of the CLI when they need it.

### Key Design Implications

- **Instant value** — First-run experience must deliver a working conversation within 1 minute
- **Progressive disclosure** — Don't show governance features until they're relevant
- **Automated governance backfill** — Point at a codebase, answer questions, get a governance framework through conversation
- **Quality without a team** — Scanners, review gates, and learning loops are the safety net
- **Low ceremony** — Minimum viable governance that grows organically, not upfront configuration

---

## Persona Comparison

| Dimension | Alex (PM/Tech Lead) | Sam (Developer) | Jordan (Solo PM) |
|-----------|-------------------|-----------------|-------------------|
| Primary goal | Manage process, delegate with governance | Structured AI-assisted development | Ship solo with quality |
| Technical depth | Can review, not implement | Expert implementer | Moderate, generalist |
| Governance role | Defines and manages | Follows and benefits from | Builds up gradually |
| UI expectation | Visual, approachable | Keyboard-first, detailed | Minimal, progressive |
| MCP extensibility | Low priority | High priority | Low priority |
| Learning loop value | Strategic (quality trends) | Tactical (stop repeating mistakes) | Survival (safety net) |
| Cost sensitivity | Budget-aware, prefers flat rate | Employer pays, less sensitive | Highly sensitive, needs flat rate |
| CLI usage | Minimal — prefers Orqa Studio UI | Heavy — uses CLI and Orqa Studio interchangeably | Occasional — prefers Orqa Studio, CLI when needed |
| First-run expectation | 5-10 min setup, configure governance | 1-2 min, connect to existing project | < 1 min to first conversation |

---

## Design Priorities by Persona

**Conflicts and resolutions:**

1. **Approachable vs. powerful** — Alex wants visual, Jordan wants minimal, Sam wants deep. Resolution: progressive disclosure with keyboard shortcuts. Default UI is approachable; power features are always accessible but not upfront.

2. **Governance upfront vs. organic** — Alex will configure governance before starting. Jordan needs it to grow organically. Resolution: conversation-first with auto-scanning and progressive feature gates. Alex can skip ahead; Jordan discovers features naturally.

3. **Detail level** — Sam wants full tool call details and streaming visibility. Alex wants summaries with drill-down. Resolution: collapsible detail panels. Summary by default, expand for full context.

---

## Related Documents

- [Product Vision](/product/vision) — Primary users section
- [User Journeys](/product/journeys) — How each persona interacts with Orqa Studio
- [Information Architecture](/product/information-architecture) — UI structure driven by persona needs

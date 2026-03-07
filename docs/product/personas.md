---
title: "User Personas"
category: product
tags: []
created: 2026-03-02
updated: 2026-03-07
---

# User Personas

**Date:** 2026-03-07

Detailed profiles of OrqaStudio™'s target users. These personas drive UI design, feature prioritisation, and acceptance criteria throughout the product.

OrqaStudio's first domain is AI-assisted software development, so personas are grounded in that context. The underlying needs — structured thinking, visible governance, reflective learning — apply more broadly and will inform how personas evolve as the platform expands to other domains.

---

## Persona 1: Alex — The Lead (Primary)

### Profile

Alex is a senior product manager or tech lead with a strong technical background. They can read code, understand architecture, and make informed technical decisions — but they don't write production code day-to-day. They manage a product or team and use AI-assisted development to move faster than would be possible with traditional staffing.

More fundamentally, Alex's job is to **turn ambiguity into clarity**. They take complex situations — unclear requirements, competing priorities, evolving markets — and produce structured understanding that their team can execute against. AI is a thinking partner in this process, not just a code generator.

### Demographics

- **Role:** Product Manager, Tech Lead, Strategy Lead, or hybrid
- **Experience:** 5-15 years in technology, transitioned from execution to leadership
- **Technical comfort:** Can read and review code, understand architecture, and evaluate technical trade-offs. Writes scripts and prototypes but not production systems.
- **Team context:** May lead a small team or work solo. Uses AI agents as a force multiplier.

### Goals

1. **Structure complex problems** — Use AI as a thinking partner to explore ambiguous situations, challenge assumptions, and build understanding before committing to action
2. **Define visible standards** — Establish rules, architecture decisions, and quality standards that are browsable, enforceable, and improving — not buried in files
3. **Delegate with confidence** — Hand tasks to AI agents knowing that governance is enforced and quality is verified
4. **Track improvement** — See evidence that the work is getting better over time: quality trends, learning loop metrics, violation recurrence
5. **Ship consistently** — Maintain a predictable cadence where each cycle builds on the last

### Pain Points

1. **Invisible governance** — Standards and decisions live in dotfiles and terminal output. Alex can't see the governance framework without reading raw markdown files and CLI logs.
2. **Ephemeral thinking** — Conversations with AI produce good insights, but they're lost between sessions. There's no structured knowledge base that accumulates.
3. **No oversight layer** — There's no way to define standards and verify they're being followed without manually reviewing every output.
4. **Context loss** — Every new conversation starts from scratch. Lessons learned in one session aren't automatically applied in the next.
5. **Tool complexity** — Current agentic development requires deep CLI expertise. The barrier to entry is "be a developer who knows terminal workflows."

### Typical Workflow

1. Review current state (open tasks, recent sessions, quality metrics)
2. Explore a problem or requirement through AI conversation
3. Produce structured artifacts (plans, decisions, requirements) from the conversation
4. Review and approve the implementation plan proposed by the AI
5. Monitor progress as agents implement within the governance framework
6. Review outcomes and capture lessons
7. Check quality trends to verify the process is improving

### What Success Looks Like

Alex can manage a product's entire cycle — from ambiguous requirement to shipped feature — through OrqaStudio's UI. They define the standards, explore problems with AI, produce structured plans, delegate implementation to agents, and watch the system get smarter over time. Alex spends their time on clarity and decisions, not on terminal commands and dotfile management.

### Key Design Implications

- **UI must be approachable** — No terminal commands required for core workflows
- **Governance is first-class** — Artifacts are browsable, editable, and visualised, not hidden in files
- **Thinking before doing** — The UI encourages exploration and structuring before committing to action
- **Plans before code** — Every implementation starts with a reviewable plan
- **Progressive complexity** — Advanced features (MCP extensibility, custom scanners) appear as Alex grows into them

---

## Persona 2: Sam — The Practitioner (Secondary)

### Profile

Sam is a senior professional who already uses AI tools daily in their domain — software development, research, consulting, or operations. They are deep experts comfortable with complexity. They value OrqaStudio as a structural layer that makes their AI-assisted work visible, governed, and improving — seeing quality trends, browsing active standards, and tracking what they've learned across sessions.

In software development (OrqaStudio's first domain), Sam already uses Claude Code CLI and other AI coding tools. They write production code and are comfortable in the terminal.

### Demographics

- **Role:** Senior Developer, Staff Engineer, Lead Researcher, Senior Consultant
- **Experience:** 5-10+ years in their domain
- **Technical comfort:** Expert. Works with complex systems daily, manages quality standards, reviews others' work.
- **Team context:** Works on a team with defined standards. May also define those standards.

### Goals

1. **Structured process** — Replace ad-hoc AI conversations with a repeatable workflow that enforces project standards
2. **Visibility into governance** — See what rules, agents, and skills are active without reading raw files
3. **Learning loop value** — Have the system actually learn from mistakes and improve over time, not just accumulate chat history
4. **Session continuity** — Resume work across sessions without losing context or re-explaining background
5. **Extensibility** — Connect additional tools, customise workflows, adapt the system to their specific needs

### Pain Points

1. **Repeated mistakes** — AI agents make the same errors across sessions because there's no mechanism to learn from past failures
2. **Inconsistent quality** — Without enforced standards, the same task produces different quality depending on the session
3. **No audit trail** — Can't look back at what agents did, what was approved, and what the outcomes were
4. **Fragmented tools** — Using separate tools for work, review, documentation, and process tracking with no integration
5. **Process overhead** — Manual governance management (editing rule files, maintaining standards, tracking lessons) takes time away from actual work

### Typical Workflow

1. Open a project, review pending tasks and recent session history
2. Start a new session for a specific task
3. Converse with the AI, reviewing tool calls and approving operations
4. Let the agent iterate based on review feedback
5. Run quality checks to verify compliance before considering the task done
6. Capture any lessons learned through the interface

### What Success Looks Like

Sam's AI-assisted work produces consistently high-quality results because the governance framework is active, not passive. Mistakes from last week don't recur this week. Standards are enforced automatically. The process improves without Sam manually editing rule files.

### Key Design Implications

- **Keyboard-first** — Power users expect keyboard shortcuts for common operations
- **Detail when needed** — Tool call cards expand to show full input/output, not just summaries
- **Extensibility** — Sam will want to connect project-specific tools and customise workflows
- **Transparent pipeline** — Sam wants to see exactly what's happening (streaming tokens, tool calls, activity)
- **No dumbing down** — The UI should be approachable for leads but not at the expense of depth for practitioners

---

## Persona 3: Jordan — The Independent (Tertiary)

### Profile

Jordan is building something alone or with a very small team. They have enough domain skill to define direction, review work, and make decisions, but they rely heavily on AI for execution. Jordan's constraint is that they wear every hat — strategist, architect, executor, reviewer — and need a tool that maximises AI assistance while maintaining quality.

In software development, Jordan is a solo founder or indie developer. More broadly, Jordan is any independent professional who needs to produce structured, quality work at a scale that would normally require a team.

### Demographics

- **Role:** Solo founder, indie developer, freelance consultant, independent researcher
- **Experience:** 3-10 years, generalist background
- **Technical comfort:** Moderate to high. Can direct and review work but prefers to define and oversee rather than execute everything personally.
- **Team context:** Solo or tiny team (1-3 people). AI agents are the "team."

### Goals

1. **Ship solo with quality** — Produce work that meets professional standards with AI agents handling the heavy execution
2. **Safety net without a team** — Quality checks, review gates, and learning loops substitute for the review culture a team would provide
3. **Governance as a force multiplier** — Define standards once, enforce them automatically across all future sessions
4. **Minimise ramp-up** — Get value from OrqaStudio within minutes, not hours of configuration
5. **Cost control** — Predictable AI costs so productivity isn't penalised by usage

### Pain Points

1. **Wearing every hat** — Quality suffers when one person is strategist, architect, executor, and reviewer simultaneously
2. **No safety net** — Without a team, there's no one to catch mistakes that the AI makes
3. **Governance feels like overhead** — Maintaining process artifacts manually feels like bureaucracy when you're also doing the work
4. **Learning curve** — New tools need to deliver value immediately; elaborate setup is a dealbreaker
5. **Scaling yourself** — The gap between "I know what I want" and "it's done well" is too wide without better tooling

### Typical Workflow

1. Open a project (or start a new one with automatic scanning and discovery)
2. Explore the problem space through conversation — shape understanding before committing
3. Define a task and review the AI's proposed plan
4. Let agents execute while monitoring progress
5. Check quality results
6. Capture lessons, move to the next task
7. Review learning metrics periodically to ensure quality trend is positive

### What Success Looks Like

Jordan ships quality work at a pace that would normally require a small team. The governance framework they build up over time becomes a compounding advantage — each session is better than the last because the rules, lessons, and standards accumulate. Jordan spends time on direction and decisions, not process management.

### Key Design Implications

- **Instant value** — First-run experience must deliver a working conversation within 1 minute
- **Progressive disclosure** — Don't show governance features until they're relevant
- **Automated governance backfill** — Point at existing work, answer questions, get a governance framework through conversation
- **Quality without a team** — Scanners, review gates, and learning loops are the safety net
- **Low ceremony** — Minimum viable governance that grows organically, not upfront configuration

---

## Persona Comparison

| Dimension | Alex (The Lead) | Sam (The Practitioner) | Jordan (The Independent) |
|-----------|----------------|----------------------|--------------------------|
| Primary goal | Structure complex work, delegate with governance | Structured AI-assisted execution | Ship solo with quality |
| Core need | Clarity and oversight | Consistency and depth | Speed and safety net |
| Domain depth | Broad, strategic | Deep, specialist | Moderate, generalist |
| Governance role | Defines and manages | Follows and benefits from | Builds up gradually |
| UI expectation | Visual, approachable | Keyboard-first, detailed | Minimal, progressive |
| Extensibility | Low priority | High priority | Low priority |
| Learning loop value | Strategic (quality trends, process improvement) | Tactical (stop repeating mistakes) | Survival (safety net for solo work) |
| Cost sensitivity | Budget-aware, prefers predictability | Employer pays, less sensitive | Highly sensitive, needs flat rate |
| First-run expectation | 5-10 min setup, configure governance | 1-2 min, connect to existing project | < 1 min to first conversation |

---

## Design Priorities by Persona

**Conflicts and resolutions:**

1. **Approachable vs. powerful** — Alex wants visual, Jordan wants minimal, Sam wants deep. Resolution: progressive disclosure with keyboard shortcuts. Default UI is approachable; power features are always accessible but not upfront.

2. **Governance upfront vs. organic** — Alex will configure governance before starting. Jordan needs it to grow organically. Resolution: conversation-first with auto-scanning and progressive feature gates. Alex can skip ahead; Jordan discovers features naturally.

3. **Detail level** — Sam wants full tool call details and streaming visibility. Alex wants summaries with drill-down. Resolution: collapsible detail panels. Summary by default, expand for full context.

---

## Related Documents

- [Product Vision](vision.md) — Core principles and pillars
- [User Journeys](journeys.md) — How each persona interacts with OrqaStudio
- [Information Architecture](information-architecture.md) — UI structure driven by persona needs

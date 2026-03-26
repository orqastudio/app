---
id: IDEA-c9a1979f
type: discovery-idea
title: "Native agent observability and artifact-correlated analytics"
description: "Build native observability into OrqaStudio rather than integrating AgentOps (architecture mismatch — see .state/agentops-research.md). Track session metrics, delegation outcomes, cost awareness, and correlate analytics to artifacts."
status: captured
created: 2026-03-24
updated: 2026-03-24
horizon: later
research-needed:
  - "What data is available from Claude Code's output? Token counts, cost, timing?"
  - "What schema extensions to the sessions SQLite table are needed for delegation metrics?"
  - "How should artifact-correlated analytics be modelled — foreign keys to artifact IDs, or a separate analytics table?"
  - "What governance health metrics are most actionable? (violation trends, enforcement gap detection, lesson promotion velocity)"
  - "How do session metrics feed into the learning loop — automatic lesson creation from repeated failures?"
  - "What visualisation patterns work for agent performance dashboards (timelines, heatmaps, trend charts)?"
  - "Can cost awareness be implemented incrementally — starting with token estimates, adding real costs when provider APIs expose them?"
  - "What privacy considerations apply to storing detailed agent interaction data locally?"
relationships:
  - target: PILLAR-2acd86c1
    type: grounded
    rationale: Observability directly feeds the learning loop — metrics become lessons, patterns become rules, enforcement improves over time
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: Governance health dashboard and agent performance tracking surface what would otherwise be hidden in session logs and terminal output
  - target: PERSONA-c4afd86b
    type: benefits
    rationale: Alex wants to track improvement and see evidence that work is getting better — quality trends, learning loop metrics, violation recurrence
  - target: PERSONA-477971bf
    type: benefits
    rationale: Sam values visible governance and wants structured metrics showing AI-assisted work quality over time
---

## Motivation

OrqaStudio has a strong governance pipeline (rules, enforcement, knowledge injection, learning loop) but no quantitative observability layer. Questions like "how much did this epic cost?", "which agents fail most often?", and "is our governance getting better over time?" cannot be answered.

The AgentOps research (.state/agentops-research.md) confirmed that third-party observability platforms target a different architecture — they instrument LLM SDK calls, which OrqaStudio doesn't make directly. OrqaStudio delegates to Claude Code, which handles the API calls internally. The hook points for AgentOps-style instrumentation don't exist.

The right approach is to build native observability at OrqaStudio's actual integration points: Tauri commands, agent delegations, tool calls, and session lifecycle events. The unique advantage is that OrqaStudio can correlate observability data with the artifact graph, creating analytics that no external tool can provide.

## Candidate Items

### a. Artifact-Correlated Analytics

The killer feature that external observability tools cannot provide. Because OrqaStudio has the artifact graph, session metrics can be correlated to specific artifacts:

- "EPIC-X took 5 sessions, spawned 23 agents, had 3 review failures before passing"
- "Tasks with more than 2 dependencies take 40% longer on average"
- "Research tasks have a 90% completion rate; implementation tasks have 72%"
- Epic-level cost aggregation: total tokens, total time, agent count per epic
- Task-level effort tracking: which tasks consumed the most agent delegations

### b. Session Metrics

Extend the existing session/SQLite infrastructure to track quantitative data per session:

- Delegation counts: how many agents spawned, by role (Implementer, Reviewer, Researcher, etc.)
- Delegation durations: wall-clock time per agent task
- Delegation outcomes: success, failure, partial completion rates
- Tool call counts: which tools used most frequently, which fail most
- Session duration and active time
- Stored in SQLite alongside existing session data (per AD-859ed163)

### c. Cost Awareness

Token and cost tracking, implemented incrementally:

- Phase 1: Capture token counts from Claude Code's output (it reports usage in some contexts)
- Phase 2: Map token counts to cost using provider pricing tables
- Phase 3: Aggregate costs per session, per epic, per milestone
- Display cost alongside session summaries and epic dashboards
- Alert when session cost exceeds a user-defined threshold

### d. Learning Loop Integration

This is the differentiator AgentOps completely lacks. OrqaStudio can close the loop from observation to enforcement:

- **Observe**: Session metrics capture agent failure patterns
- **Learn**: Repeated failures auto-create implementation lessons (IMPL entries)
- **Promote**: Lessons that recur get promoted to rules or knowledge updates
- **Enforce**: New rules are mechanically enforced via the enforcement pipeline
- **Measure**: Enforcement effectiveness is tracked by the same observability layer

The learning loop becomes self-referential — observability data feeds the learning loop, and the learning loop's effectiveness is measured by observability data.

### e. Agent Performance Tracking

Per-agent-role analytics over time:

- Success/failure rates by role (do Reviewers catch issues? do Implementers pass first review?)
- Average task duration by role and knowledge combination
- Common failure patterns: which acceptance criteria fail most often?
- Knowledge effectiveness: do agents with specific knowledge loaded perform better?
- Cross-session trends: is agent performance improving as governance matures?

### f. Governance Health Dashboard

Rule violation trends and enforcement gap detection over time:

- Rule violation frequency: which rules are violated most, and is the trend improving?
- Enforcement gap detection: rules without mechanical enforcement, tracked as a metric
- Lesson promotion velocity: how quickly do recurring lessons get promoted to rules?
- Schema validation pass/fail rates across artifact types
- Pipeline integrity score: percentage of rules with complete enforcement chains
- Historical trend charts showing governance maturity over milestones

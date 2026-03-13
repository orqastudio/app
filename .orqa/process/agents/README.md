---
description: "Universal roles that coordinate work through skill-based specialisation."
role: artifacts
label: "Agents"
icon: "bot"
sort: 5
---

# Agents

Agents are universal roles that define *how* work gets done. They specify process, ownership boundaries, required reading, and tool access. Domain expertise comes from skills loaded at runtime — the same role adapts to any project context by loading different skills.

## Pipeline Role

Agents are the **executors** of the knowledge maturity pipeline. They apply lessons, follow decisions, serve pillars, load skills, and comply with rules. They also feed the pipeline: reviewers surface observations that become lessons, implementers apply decisions that become skills, orchestrators promote recurring patterns into rules.

## The 7 Universal Roles

| Role | Purpose |
|------|---------|
| **Orchestrator** | Coordinates work, enforces process, delegates to other roles |
| **Researcher** | Investigates questions, gathers information, produces findings |
| **Planner** | Designs approaches, evaluates tradeoffs, produces plans |
| **Implementer** | Builds things — code, deliverables, artifacts |
| **Reviewer** | Checks quality, compliance, correctness — produces verdicts |
| **Writer** | Creates documentation, specifications, records |
| **Designer** | Designs experiences, interfaces, structures |

Roles specialise by loading skills at delegation time: `Implementer + rust-async-patterns + tauri-v2` becomes a backend engineer; `Reviewer + code-quality-review` becomes a code reviewer. The role defines the boundaries; skills provide the domain knowledge.

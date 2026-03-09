---
role: artifacts
label: "Agents"
description: "Universal roles that coordinate work through skill-based specialisation."
icon: "bot"
sort: 1
---

# Agents

Agents are universal roles that define *how* work gets done. They specify process, ownership boundaries, required reading, and tool access. Domain expertise comes from skills loaded at runtime — the same role adapts to different project contexts by loading different skills.

## The 7 Universal Roles

| Role | Purpose | Subagent Mapping |
|------|---------|-----------------|
| **Orchestrator** | Coordinates work, enforces process, delegates to other roles | N/A (is the coordinator) |
| **Researcher** | Investigates questions, gathers information, produces findings | Explore |
| **Planner** | Designs approaches, evaluates tradeoffs, produces plans | Plan |
| **Implementer** | Builds things — code, deliverables, artifacts | Backend/Frontend/Data/DevOps/Refactor/Debugger |
| **Reviewer** | Checks quality, compliance, correctness — produces verdicts | Code Reviewer/Test/QA/UX/Security |
| **Writer** | Creates documentation, specifications, records | Documentation Writer |
| **Designer** | Designs experiences, interfaces, structures | Designer/Frontend/UX Reviewer |

## How Skill-Based Specialisation Works

The orchestrator loads domain skills into universal roles at delegation time. For example:

- **Implementer + rust-async-patterns + tauri-v2** → becomes a Backend Engineer
- **Implementer + svelte5-best-practices + tailwind-design-system** → becomes a Frontend Engineer
- **Reviewer + code-quality-review** → becomes a Code Reviewer
- **Reviewer + security-audit** → becomes a Security Engineer

The `subagent_mapping` in each role's YAML frontmatter documents which Claude Code subagent type is used for each skill combination.

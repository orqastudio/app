# Agent Architecture and Prompt Generation Pipeline

> This is part of the OrqaStudio Architecture Reference. See ARCHITECTURE.md for the complete document.

---

## 6. Agent Architecture (Target State)

### 6.1 Base Roles

Eight base agent roles define high-level responsibilities, permissions, and behavioral boundaries. These are stable definitions provided by the methodology plugin:

| Role | Responsibility | Permission Scope |
|------|---------------|-----------------|
| **Orchestrator** | Coordinates work, delegates tasks, reads summaries | Read-only, delegation |
| **Implementer** | Writes code, runs tests | Source code, shell access |
| **Reviewer** | Verifies quality, produces verdicts | Read-only, checks only |
| **Researcher** | Investigates questions, gathers information | Read-only, creates research artifacts |
| **Writer** | Creates and edits documentation | Documentation only |
| **Planner** | Designs approaches, maps dependencies | Plans and delivery artifacts |
| **Designer** | Creates UI/UX designs, component structures | Design artifacts, component code |
| **Governance Steward** | Maintains governance artifacts, ensures process compliance | `.orqa/` artifacts only |

Each base role defines:

- Behavioral boundaries (what the agent may/may not do)
- Tool constraints (which tools it can use)
- Artifact scope (what it can create/edit)
- Permission scope (enforced by the generated tool-native plugin)

### 6.2 Task-Specific Agent Generation

The agents that get used in practice are **generated on a bespoke basis** for each task. The engine generates them by composing:

```
Base Role + Workflow Context + Domain Knowledge = Task-Specific Agent
```

**Base Role** (from methodology plugin): defines permissions and boundaries.

**Workflow Context** (from active workflow): provides workflow-specific instructions. An Implementer in the implementation workflow gets different context than an Implementer in the documentation workflow.

**Domain Knowledge** (from knowledge plugins): composed into the agent based on task scope, file paths, and subject matter. Selected at delegation time, not predefined.

The **engine** provides the generation functionality (prompt pipeline, knowledge injection, role composition). The **connector** provides translation so the generated agents are defined in the third-party tool's native format (e.g., Claude Code's agent structure).

Agents are **ephemeral** — generated and discarded, not tracked as artifacts in the graph. The base role definitions in the methodology plugin are the canonical source, and the prompt pipeline is the set process for composing them into runtime specialists. There is no agent artifact type, no agent workflow, and no AGENT-*.md files.

### 6.3 Token Budgets

Token budgets apply to the generated task-specific agents — they constrain the output of the prompt pipeline, not the base role definitions.

| Generated Agent Type | Total Budget |
|---------------------|-------------|
| Orchestrator | 2,500 tokens |
| Implementer | 2,800 tokens |
| Reviewer | 1,900 tokens |
| Researcher | 2,100 tokens |
| Writer | 1,800 tokens |
| Planner | 2,500 tokens |
| Designer | 1,800 tokens |
| Governance Steward | 1,800 tokens |

The generator balances what needs to be **embedded knowledge** (in the system prompt, within budget) vs what should be **instructions to retrieve documentation via MCP** (on-demand). The general principle is accuracy over speed — err on the side of giving agents what they need to be correct, even if it means more MCP lookups.

---

## 7. Prompt Generation Pipeline (Target State)

```
Plugin Registry -> Schema Assembly -> Section Resolution -> Token Budgeting -> Prompt Output
```

| Stage | What Happens |
|-------|-------------|
| **Plugin Registry** | All installed plugins register prompt contributions at install time |
| **Schema Assembly** | For a (base role, workflow, task) tuple, collect applicable prompt sections |
| **Section Resolution** | Resolve references to compressed summaries; follow cross-refs depth 1 |
| **Token Budgeting** | Measure against budget; trim P3 first, then P2, then P1; never trim P0 |
| **Prompt Output** | Static core at TOP (cached), dynamic content at BOTTOM (changes per turn) |

The pipeline balances embedded knowledge vs MCP retrieval instructions based on priority, token budget, and task relevance. Accuracy is preferred over speed — on-demand retrieval latency is acceptable if it produces better outcomes.

---
id: RES-7ee1a770
title: "Workflow Composition from Plugins"
type: research
status: active
category: architecture
description: "How workflows are composed from plugin contributions — contribution-point model, stage plugins, YAML+code format"
created: 2026-03-25
updated: 2026-03-25
tags:
  - agent-teams
  - plugin-architecture
---

# Research: Workflow Composition from Plugin Installations

## Executive Summary

This research investigates how mature systems compose end-to-end workflows from modular, pluggable components -- and what patterns OrqaStudio should adopt for building project workflows from plugin installations (core-framework + discovery + delivery plugins). The findings draw from workflow engines (Temporal, Prefect, Kestra), developer platforms (Backstage, Nx, VS Code), CI/CD systems (GitHub Actions), project management tools (Jira, Linear, Shortcut), and multi-agent AI frameworks (LangGraph, CrewAI, Semantic Kernel, AutoGen).

---

## Question 1: How Do Mature Plugin Systems Compose Workflows?

### Option A: Contribution-Point Composition (VS Code, Backstage)

Plugins declare what they contribute to a shared schema. The host system merges contributions.

**How it works:**
- VS Code uses `contributes` in `package.json` -- plugins declare commands, views, menus, languages, etc. The host merges all contributions into a unified UI/command palette.
- Backstage uses `extension points` -- plugins register interfaces (e.g., `addAction()` on the scaffolder), and modules extend those interfaces. Critically, "all modules that extend a plugin are completely initialized before the plugin gets initialized."

**Pros:**
- Declarative -- easy to validate, merge, and reason about
- Lazy loading -- plugins activate only when needed (VS Code's activation events)
- Additive by design -- extensions add to the system, they don't remove
- Clear ownership -- each plugin owns its contribution points

**Cons:**
- Limited expressiveness -- can't express complex conditional logic
- Merge conflicts -- two plugins contributing to the same slot may conflict
- Discovery overhead -- consumers must know which extension points exist

**Relevance to OrqaStudio:** High. The contribution-point model maps naturally to "a discovery plugin contributes discovery stages to the workflow skeleton." The core framework defines slots (discovery, delivery, learning); plugins fill them.

### Option B: Inferred Composition (Nx Project Crystal)

Plugins scan the workspace and automatically infer what they contribute based on what exists.

**How it works:**
- Nx 18+ plugins scan for configuration files (e.g., `webpack.config.js`) and automatically infer build targets. No explicit registration -- the plugin's presence + the config file's existence = the target exists.
- "Project Crystal automatically infers targets within a project, significantly reducing the size of project configuration files."

**Pros:**
- Zero-config for common cases
- Convention-over-configuration -- reduces boilerplate
- Single source of truth -- the config file IS the declaration

**Cons:**
- Magic -- harder to debug when inference is wrong
- Limited control -- hard to override inferred behavior
- Coupling -- the inference logic must understand every possible config format

**Relevance to OrqaStudio:** Medium. The "scan and infer" pattern could apply to project type detection (scan for Cargo.toml -> software project, scan for .orqa/ structure -> infer which plugins are needed). But workflow stages should be explicitly declared, not inferred.

### Option C: Pipeline Composition (GitHub Actions, Kestra)

Workflows are defined as pipelines of steps. Each step references a reusable action/plugin.

**How it works:**
- GitHub Actions: Reusable workflows are entire pipelines; composite actions are step-level building blocks. "Treat reusable workflows as pipeline templates and composite actions as shared task templates."
- Kestra: YAML-defined workflows reference plugin tasks. 500+ integrations available as plugins. "The YAML definition gets automatically adjusted any time you make changes from the UI or via an API call."

**Pros:**
- Explicit sequencing -- the pipeline order is visible in the definition
- Versioned references -- pin plugin versions for reproducibility
- Composable at two levels -- whole pipelines AND individual steps
- Wide ecosystem adoption -- developers understand this model

**Cons:**
- Static structure -- hard to express dynamic, conditional workflows
- YAML sprawl -- complex workflows become hard to read
- Limited inter-step communication -- steps pass data through artifacts/outputs

**Relevance to OrqaStudio:** High for the workflow definition format. The two-level composition (whole-pipeline plugins like "discovery" + step-level plugins like "research-methodology") maps well to OrqaStudio's needs.

### Recommendation for OrqaStudio

**Use a hybrid of A (contribution-point) and C (pipeline composition):**

1. The **core-framework plugin** defines a workflow skeleton with named **slots** (discovery, delivery, learning)
2. **Stage plugins** (discovery, delivery) contribute their stages to the appropriate slots via a declarative manifest
3. Within each stage, the plugin defines a **pipeline of steps** that can reference reusable actions
4. The installed combination is merged at install time into a resolved workflow definition

This avoids the magic of Option B while getting the declarative clarity of A and the sequencing expressiveness of C.

---

## Question 2: Best Practices for Workflow Definition Formats

### Option A: Workflow-as-Config (YAML/JSON)

Used by: GitHub Actions, Kestra, CircleCI, CloudSlang, Azure Logic Apps

**Format example for OrqaStudio:**
```yaml
workflow:
  id: software-project
  stages:
    discovery:
      plugin: "@orqastudio/discovery"
      steps:
        - id: research
          type: research
          gate:
            type: human
            question: "Research findings approved?"
        - id: plan
          type: planning
          gate:
            type: human
            question: "Implementation plan approved?"
        - id: document
          type: documentation
          gate:
            type: automated
            check: "docs-required satisfied"
    delivery:
      plugin: "@orqastudio/delivery"
      steps:
        - id: implement
          type: implementation
          parallel: false
          gate:
            type: review
            reviewer: code-reviewer
        - id: review
          type: review
          gate:
            type: human
            question: "Release approved?"
    learning:
      plugin: "@orqastudio/core-framework"
      steps:
        - id: retrospect
          type: learning
          trigger: on-epic-complete
```

**Pros:**
- Human-readable and auditable
- Schema-validatable (JSON Schema)
- Non-developers can understand and modify
- Serializable to/from disk -- fits OrqaStudio's file-based artifact model
- "DSL-based engines are more accessible to non-developers" (Kestra)

**Cons:**
- Limited expressiveness for conditional logic
- No IDE support for autocomplete/debugging
- "The moment you need anything dynamic or non-trivial, the complexity of DSL-based state machines grows quickly" (Temporal comparison)

### Option B: Workflow-as-Code

Used by: Temporal, Prefect, Durable Functions, LangGraph

**Format example for OrqaStudio:**
```typescript
const workflow = defineWorkflow("software-project", (ctx) => {
  // Discovery
  const research = await ctx.stage("discovery", "research", {
    gate: humanGate("Research findings approved?")
  });

  const plan = await ctx.stage("discovery", "plan", {
    gate: humanGate("Implementation plan approved?"),
    dependsOn: [research]
  });

  // Delivery -- conditional based on scope
  if (ctx.scope === "bug-fix") {
    await ctx.stage("delivery", "implement", { gate: reviewGate() });
  } else {
    await ctx.stage("delivery", "implement", { gate: reviewGate() });
    await ctx.stage("delivery", "review", { gate: humanGate("Release approved?") });
  }
});
```

**Pros:**
- Full expressiveness -- conditionals, loops, error handling
- IDE support -- autocomplete, type checking, debugging
- "You write Workflows as straightforward code in your language of choice" (Temporal)
- Testable -- unit tests for workflow logic
- "Prefect moved away from requiring workflows to be DAGs, fully embracing native Python control flow"

**Cons:**
- Requires developer expertise to modify
- Harder to visualize
- Harder to serialize/store as artifacts
- Not auditable by non-technical stakeholders

### Option C: Hybrid (Config + Code Hooks)

Used by: Azure Logic Apps (JSON + code actions), Backstage (YAML templates + custom actions), CrewAI (Flows + Processes)

**Format example for OrqaStudio:**
```yaml
# Declarative skeleton (YAML)
workflow:
  id: software-project
  stages:
    discovery:
      steps: [research, plan, document]
    delivery:
      steps: [implement, review]
    learning:
      steps: [retrospect]

  variants:
    bug-fix:
      skip: [research, plan, document]
    research-only:
      skip: [implement, review]
```

```typescript
// Code hooks for complex logic (TypeScript)
export const gates: GateDefinitions = {
  "research": {
    type: "human",
    canSkip: (ctx) => ctx.variant === "bug-fix",
    validate: (ctx) => ctx.hasArtifact("research", ctx.epic.id)
  },
  "implement": {
    type: "review",
    reviewer: "code-reviewer",
    validate: async (ctx) => {
      const result = await ctx.runCheck("make check");
      return result.exitCode === 0;
    }
  }
};
```

**Pros:**
- Best of both worlds -- simple cases are config, complex cases are code
- The YAML is the "what" (auditable), the code is the "how" (expressive)
- Fits OrqaStudio's existing artifact model (YAML frontmatter + body)
- Plugin authors can provide both config and code

**Cons:**
- Two formats to learn and maintain
- Potential for config/code drift
- Validation must span both formats

### Recommendation for OrqaStudio

**Option C: Hybrid.** The workflow skeleton and stage list should be YAML (declarative, auditable, file-based, consistent with the artifact graph). Gate logic and conditional behavior should be code hooks (TypeScript/Rust) that the YAML references by name. This matches OrqaStudio's existing pattern of YAML frontmatter (structure) + body (content) and allows non-developer governance auditors to read the workflow while developers implement the complex logic.

---

## Question 3: How Should Plugins Extend vs Override Workflow Stages?

### Pattern A: Slot-Based Composition

The core defines named slots. Plugins fill slots.

**How it works:**
- The core workflow has named phases: `discovery`, `delivery`, `learning`
- Each phase is a slot that accepts step contributions from plugins
- A discovery plugin contributes steps to the `discovery` slot
- A delivery plugin contributes steps to the `delivery` slot
- The core's `learning` phase has built-in steps (lesson capture, promotion)

**Example:**
```
Core Framework defines:
  [discovery] -> [delivery] -> [learning]

Discovery Plugin fills [discovery]:
  [research] -> [plan] -> [document]

Delivery Plugin fills [delivery]:
  [implement] -> [review] -> [release]

Resolved workflow:
  [research] -> [plan] -> [document] -> [implement] -> [review] -> [release] -> [retrospect]
```

**Pros:**
- Clear boundaries -- plugins can only contribute to their designated slots
- No override conflicts -- slots accept additions, not replacements
- Composable -- swap one discovery plugin for another without touching delivery
- Follows Backstage's principle: "extension points should support additions only"

**Cons:**
- Inflexible -- what if a plugin needs to add a step between existing steps from another plugin?
- The core must pre-define all possible slots

### Pattern B: Middleware/Hook Chain

Plugins register hooks at defined points. The core calls hooks in order.

**How it works:**
- The core defines lifecycle events: `before-stage`, `after-stage`, `on-gate`, `on-transition`
- Plugins register handlers for these events
- Handlers execute in priority order and can modify context, add steps, or block transitions

**Example:**
```typescript
// Discovery plugin registers hooks
registerHook("before-stage:delivery", async (ctx) => {
  // Ensure docs exist before delivery starts
  if (!ctx.hasDocsRequired()) {
    throw new GateError("Documentation gate not satisfied");
  }
});

registerHook("after-stage:delivery", async (ctx) => {
  // Auto-create retrospective after delivery
  await ctx.createArtifact("lesson", { epic: ctx.epic.id });
});
```

**Pros:**
- Maximum flexibility -- hooks can intercept any point
- Cross-cutting concerns -- logging, validation, metrics can be added without modifying stages
- Familiar pattern (Express middleware, Git hooks, WordPress hooks)

**Cons:**
- Order-dependent -- hook execution order matters and is hard to reason about
- Debugging complexity -- "what hooks run before stage X?" requires tracing
- Tight coupling through shared context

### Pattern C: Contribution Merging

Plugins declare their contributions in a manifest. The installer merges them.

**How it works:**
- Each plugin has a `workflow.yaml` that declares what stages/steps it provides
- At install time, all plugin manifests are merged into a single resolved workflow
- Merge rules handle conflicts (same stage contributed by two plugins -> error)
- The resolved workflow is written to disk as a project artifact

**Example:**
```yaml
# core-framework/workflow.yaml
phases:
  - id: discovery
    slot: true  # Other plugins can fill this
  - id: delivery
    slot: true
  - id: learning
    steps:
      - id: capture-lessons
      - id: promote-patterns

# discovery-software/workflow.yaml
fills: discovery
steps:
  - id: research
    agents: [researcher]
    gate: { type: human }
  - id: plan
    agents: [planner]
    gate: { type: human }
  - id: document
    agents: [writer]
    gate: { type: automated, check: docs-exist }

# delivery-software/workflow.yaml
fills: delivery
steps:
  - id: implement
    agents: [implementer]
    gate: { type: review, role: reviewer }
  - id: verify
    agents: [reviewer]
    gate: { type: automated, check: tests-pass }
```

**Pros:**
- Static analysis -- the resolved workflow is known at install time, not runtime
- Conflict detection -- merge failures surface immediately
- Auditable -- the resolved workflow is a file on disk
- Versioned -- the resolved workflow is committed to git

**Cons:**
- No runtime flexibility -- can't adapt the workflow based on runtime context
- Requires an install step -- changes to plugins require re-installing

### Recommendation for OrqaStudio

**Pattern C (Contribution Merging) as the primary mechanism, with Pattern B (Hooks) for cross-cutting concerns:**

1. Plugins declare their workflow contributions in their manifest (YAML)
2. `orqa install` merges all plugin manifests into a resolved workflow written to `.orqa/process/workflow.yaml`
3. The resolved workflow is the source of truth -- committed to git, readable by all agents
4. Hooks provide runtime extensibility for validation, logging, and conditional behavior
5. The core framework's slots are the ONLY extension points -- plugins cannot create new top-level phases

This matches OrqaStudio's existing `orqa install` pattern and keeps the resolved workflow as a file-based artifact (consistent with the artifact graph).

---

## Question 4: How Do Real-World Systems Handle Workflow Variants?

### Approach A: Multiple Workflow Definitions (Jira)

Each project type has its own complete workflow definition.

**How it works:**
- Jira allows per-project workflows with different states and transitions
- A "Bug Tracking" workflow has: Reported -> Under Investigation -> Fixed -> Verified
- A "Feature Development" workflow has: Backlog -> In Progress -> In Review -> Done
- Teams pick which workflow applies to their project

**Pros:**
- Maximum flexibility per project type
- Clear -- each workflow is self-contained
- No conditional complexity within a single workflow

**Cons:**
- Duplication -- shared stages are repeated across workflows
- Maintenance burden -- updating a common pattern requires touching all workflows
- Jira's complexity: "a simple change to a workflow might require navigating five different administration screens"

### Approach B: Single Workflow with Skip Conditions (Shortcut, SAP)

One workflow, but steps can be conditionally skipped.

**How it works:**
- Shortcut has "a single workflow per team with linear state progressions"
- SAP's Process Variants: "business rules may define variants where steps are skipped based on criteria"
- Steps have `canSkip` conditions evaluated at runtime
- Fast-track transitions allow jumping past intermediate states

**Pros:**
- Single source of truth -- one workflow to maintain
- Simpler -- variants are just conditions on the base workflow
- The full workflow is always visible, even if some steps are skipped

**Cons:**
- Complex conditions -- many variants make the single workflow hard to read
- "Linear starts to strain past about 15 teams" (Linear comparison)
- Fast-track logic can be hard to debug

### Approach C: Workflow Templates with Overrides

A base template that variants customize through overrides.

**How it works:**
- Define a base workflow with all stages
- Define named variants that override specific properties
- At project creation, select a variant that resolves to a concrete workflow

**Example for OrqaStudio:**
```yaml
base:
  stages: [research, plan, document, implement, review, release, retrospect]

variants:
  full:
    # Uses base as-is

  bug-fix:
    skip: [research, plan, document, release]
    # Resolved: [implement, review, retrospect]

  research-only:
    skip: [implement, review, release]
    # Resolved: [research, plan, document, retrospect]

  quick-fix:
    skip: [research, plan, document, review, release]
    # Resolved: [implement, retrospect]

  ad-hoc:
    # Defined by ad-hoc workflow rules (RULE-ad-hoc)
    # Dynamic: steps determined per-task based on scope
    custom: true
```

**Pros:**
- DRY -- base workflow defined once
- Clear variant definitions
- Easy to add new variants
- Resolved workflow is deterministic per variant

**Cons:**
- Limited expressiveness (only skip/add, not reorder)
- Must pre-define all variants

### Approach D: Difficulty-Adaptive Workflows (Emerging AI Pattern)

The workflow adapts its granularity based on task complexity.

**How it works:**
- Recent research (2025): "Task-level workflows over-process simple queries, wasting resources"
- A difficulty classifier determines the workflow path at runtime
- Simple tasks get abbreviated workflows; complex tasks get the full pipeline
- LangGraph uses conditional edges to route based on state

**Pros:**
- Optimal resource usage -- simple tasks don't pay the cost of complex workflows
- Adaptive -- handles novel task types without pre-defined variants
- Aligns with token efficiency goals

**Cons:**
- Harder to audit -- the workflow path isn't known until runtime
- Requires a reliable difficulty classifier
- Non-deterministic -- same input might get different paths

### Recommendation for OrqaStudio

**Approach C (Templates with Overrides) for the project-level workflow, with elements of D (adaptive) for per-task routing within stages:**

1. The project workflow is resolved at install time from the variant selection
2. Variants are declared by plugins and resolved during `orqa install`
3. Within a stage, the orchestrator can adapt the specific steps based on task scope (e.g., a bug fix skips research within the discovery stage)
4. The ad-hoc variant allows maximum flexibility for small tasks, with step selection determined per-task

This keeps the project-level workflow deterministic and auditable while allowing per-task adaptation within stages.

---

## Question 5: What is the Right Granularity for Workflow Stages?

### Fine-Grained (Individual Actions)

Stages like: `write-test`, `write-code`, `run-lint`, `run-test`, `format`, `commit`

**Pros:**
- Maximum control and observability
- Each step can have its own gate
- Easy to parallelize independent steps

**Cons:**
- Overhead -- too many transitions for simple tasks
- "Query-level workflows introduce input-specific adaptation, but their granularity is often insufficient" (Difficulty-Aware Agent Orchestration)
- Token-expensive -- each transition requires orchestrator context

### Coarse-Grained (Activity Phases)

Stages like: `discover`, `deliver`, `learn`

**Pros:**
- Simple mental model
- Low orchestration overhead
- Each phase is a complete unit of work

**Cons:**
- No visibility into progress within a phase
- Hard to define gates between sub-steps
- "Uniform multi-agent systems for entire task categories over-process simple queries"

### Medium-Grained (Recommended for AI Agent Orchestration)

Stages like: `research`, `plan`, `document`, `implement`, `review`, `release`, `retrospect`

**Pros:**
- Maps to natural agent specialization boundaries (researcher, planner, implementer, reviewer)
- Each stage corresponds to a single agent role -- clear delegation
- Gates between stages provide quality checkpoints
- "Define the roles that each agent will play, including planners, researchers, and executors" (Multi-Agent Systems Guide)
- Microsoft's patterns: Sequential for "draft, review, polish" workflows -- this IS medium granularity

**Cons:**
- May still be too coarse for complex stages (implementation might need sub-stages)
- May be too fine for simple tasks

### Recommendation for OrqaStudio

**Medium-grained stages as the primary workflow level, with sub-steps defined within stages:**

```
Project Workflow (medium-grained):
  research -> plan -> document -> implement -> review -> retrospect

Within "implement" stage (fine-grained, managed by the stage):
  create-worktree -> write-code -> run-checks -> commit -> request-review
```

The orchestrator manages transitions between medium-grained stages. Within each stage, the assigned agent manages its own fine-grained steps. This matches the existing OrqaStudio delegation model where the orchestrator coordinates and agents implement.

The medium granularity also maps perfectly to the universal roles:
| Stage | Primary Agent Role |
|-------|-------------------|
| research | Researcher |
| plan | Planner |
| document | Writer |
| implement | Implementer |
| review | Reviewer |
| retrospect | Orchestrator (with Writer) |

---

## Question 6: How Should the Workflow Be Represented for AI Orchestrator Consumption?

### Pattern A: State Machine with Transition API

The workflow is a state machine. The orchestrator queries current state and available transitions.

**How it works (from REST API workflow modeling):**
- Current state is a property on the workflow object
- Available transitions are dynamically computed based on current state + gate conditions
- The orchestrator calls a "transition" endpoint/function to move to the next state
- HATEOAS pattern: "the _links section automatically updates to show what workflow transitions are possible"

**Implementation for OrqaStudio:**
```typescript
interface WorkflowState {
  currentStage: string;           // "implement"
  currentStep: string | null;     // "write-code" (within stage)
  availableTransitions: Transition[];
  completedStages: string[];
  blockedBy: BlockReason | null;  // Gate not satisfied, dependency not met
}

interface Transition {
  target: string;                 // "review"
  gate: Gate;                     // { type: "review", satisfied: false }
  label: string;                  // "Submit for review"
}
```

**Pros:**
- Self-describing -- the orchestrator doesn't need to know the full workflow graph
- Dynamic -- transitions reflect current context (gates satisfied, dependencies met)
- Familiar pattern (Symfony Workflow, .NET State Machine, AWS Step Functions)

**Cons:**
- Requires a running service to compute transitions
- State must be persisted somewhere

### Pattern B: Static Graph Loaded into Context

The full workflow graph is loaded into the orchestrator's context at session start.

**How it works:**
- The resolved workflow YAML is read into the orchestrator's context
- The orchestrator traverses the graph using its own logic
- Progress is tracked in session state

**Implementation for OrqaStudio:**
```yaml
# Loaded into orchestrator context at session start
workflow:
  current: implement
  stages:
    - id: research
      status: done
      gate: { type: human, satisfied: true }
    - id: plan
      status: done
      gate: { type: human, satisfied: true }
    - id: document
      status: done
      gate: { type: automated, check: docs-exist, satisfied: true }
    - id: implement
      status: in-progress
      gate: { type: review, role: code-reviewer, satisfied: false }
    - id: review
      status: pending
      gate: { type: human, satisfied: false }
    - id: retrospect
      status: pending
```

**Pros:**
- No external service needed
- Full context in the LLM's window -- the orchestrator can reason about the entire workflow
- Simple -- just read a file
- Matches OrqaStudio's file-based artifact model

**Cons:**
- Context window cost -- the full graph takes tokens
- Stale -- if another agent changes something, the orchestrator's copy is outdated
- Orchestrator must implement transition logic itself

### Pattern C: Event-Driven with Workflow Tracker

State transitions emit events. A tracker maintains current state.

**How it works (from CrewAI Flows and LangGraph):**
- "CrewAI's 2025 addition of Flows provides a state-machine orchestration layer enabling conditional branching, parallel execution, and event-driven transitions between crews"
- LangGraph: "State is a shared data structure representing the current snapshot. Edges determine which Node to execute next based on current state."
- Events trigger transitions; a tracker (in-memory or persisted) maintains the current position

**Implementation for OrqaStudio:**
```typescript
class WorkflowTracker {
  private state: WorkflowState;

  transition(stageId: string, event: WorkflowEvent): TransitionResult {
    const gate = this.getGate(stageId);
    if (!gate.isSatisfied()) {
      return { blocked: true, reason: gate.getBlockReason() };
    }
    this.state.currentStage = stageId;
    this.emit("stage-entered", { stage: stageId });
    return { success: true, nextTransitions: this.getAvailableTransitions() };
  }
}
```

**Pros:**
- Real-time state -- always accurate
- Event-driven -- other systems can react to transitions
- Integrates with OrqaStudio's existing WorkflowTracker (ephemeral state)

**Cons:**
- Requires a running service
- More complex than file-based approaches
- Events can be lost if the service restarts

### Recommendation for OrqaStudio

**A hybrid of B (static graph) and C (event-driven tracker):**

1. The resolved workflow is a file-based artifact (`.orqa/process/workflow.yaml`) -- committed to git, loaded into orchestrator context
2. A `WorkflowTracker` (ephemeral, in-memory) maintains runtime state (current stage, gate satisfaction, progress)
3. The orchestrator reads the static graph for "what's the full workflow" and queries the tracker for "where are we now, what's next"
4. The tracker exposes a simple API: `currentState()`, `availableTransitions()`, `transition(target)`
5. On session restart, the tracker reconstructs state from artifact statuses (epic/task status fields)

This gives the orchestrator full workflow awareness (from the static graph) without paying the token cost of maintaining runtime state in context. The tracker is ephemeral but reconstructible -- consistent with OrqaStudio's existing ephemeral state model.

---

## Migration Considerations

### From Current Monolithic Workflow to Plugin-Composed Workflow

The current OrqaStudio workflow is defined implicitly through:
- The orchestrator's CLAUDE.md (process section)
- Rules that reference stages (RULE-dccf4226 plan-mode-compliance, RULE-b10fe6d1 artifact-lifecycle)
- The artifact lifecycle (idea -> epic -> task -> done)

**Migration steps:**

1. **Extract the implicit workflow** -- Read all rules and the orchestrator definition to identify the current stage sequence and gates
2. **Define the core-framework skeleton** -- Create the base workflow definition with slots for discovery, delivery, learning
3. **Package current stages as the "software" discovery/delivery plugins** -- The existing research/plan/document/implement/review stages become plugin contributions
4. **Resolve to `.orqa/process/workflow.yaml`** -- The installed workflow is written as an artifact
5. **Update the orchestrator** -- Instead of reading CLAUDE.md for process flow, the orchestrator reads the resolved workflow
6. **Rules become gate definitions** -- Rules like "documentation gate" (docs-required must exist) become gate definitions on the workflow stages

**Key risk:** The current workflow is deeply embedded in CLAUDE.md prose. Extracting it into a structured format will reveal implicit assumptions and undocumented transitions.

**Mitigation:** Run the extraction as a research task first -- map the current workflow before attempting to restructure it.

---

## Summary of Recommendations

| Question | Recommendation |
|----------|---------------|
| Composition model | Contribution-point slots + pipeline composition |
| Definition format | Hybrid YAML (skeleton) + code hooks (gates) |
| Extension mechanism | Contribution merging at install + hooks at runtime |
| Workflow variants | Templates with overrides + per-task adaptation |
| Stage granularity | Medium-grained (maps to agent roles) with sub-steps within |
| Orchestrator representation | Static graph file + ephemeral WorkflowTracker |

### Critical Design Principle

**The resolved workflow must be a file on disk.** This is the non-negotiable constraint that drives every recommendation above. OrqaStudio's artifact graph is file-based. The orchestrator reads files. Agents read files. The workflow must be a readable, committable, auditable artifact -- not runtime-only state.

---

## Sources

- [Nx Project Crystal - Inferred Tasks](https://nx.dev/docs/concepts/inferred-tasks)
- [Backstage Extension Points Architecture](https://backstage.io/docs/backend-system/architecture/extension-points/)
- [Temporal Workflow Engine Principles](https://temporal.io/blog/workflow-engine-principles)
- [Temporal Workflow Definition](https://docs.temporal.io/workflow-definition)
- [GitHub Actions - Reusable Workflows](https://docs.github.com/en/actions/concepts/workflows-and-actions/reusing-workflow-configurations)
- [Composite Actions vs Reusable Workflows](https://dev.to/n3wt0n/composite-actions-vs-reusable-workflows-what-is-the-difference-github-actions-11kd)
- [Kestra Workflow Components](https://kestra.io/docs/workflow-components)
- [Prefect - Second Generation Workflow Engine](https://www.prefect.io/blog/second-generation-workflow-engine)
- [Shortcut Custom Workflows](https://shortcut.com/blog/how-and-why-to-customize-your-clubhouse-workflows)
- [SAP Workflow Process Variants](https://help.sap.com/docs/workflow-management/sap-workflow-management/create-process-variant)
- [Microsoft AI Agent Orchestration Patterns](https://learn.microsoft.com/en-us/azure/architecture/ai-ml/guide/ai-agent-design-patterns)
- [Semantic Kernel Agent Orchestration](https://learn.microsoft.com/en-us/semantic-kernel/frameworks/agent/agent-orchestration/)
- [LangGraph Architecture Guide 2025](https://latenode.com/blog/ai-frameworks-technical-infrastructure/langgraph-multi-agent-orchestration/langgraph-ai-framework-2025-complete-architecture-guide-multi-agent-orchestration-analysis)
- [CrewAI Multi-Agent Guide](https://mem0.ai/blog/crewai-guide-multi-agent-ai-teams)
- [Difficulty-Aware Agent Orchestration](https://arxiv.org/html/2509.11079v1)
- [Multi-Agent Systems & AI Orchestration Guide 2026](https://www.codebridge.tech/articles/mastering-multi-agent-orchestration-coordination-is-the-new-scale-frontier)
- [Modeling Workflows in REST APIs](https://kennethlange.com/how-to-model-workflows-in-rest-apis/)
- [Symfony Workflow/State Machine](https://symfony.com/doc/current/workflow/workflow-and-state-machine.html)
- [Azure Durable Functions Fan-out/Fan-in](https://learn.microsoft.com/en-us/azure/azure-functions/durable/durable-functions-cloud-backup)
- [VS Code Extension Contribution Points](https://code.visualstudio.com/api/references/contribution-points)
- [VS Code Activation Events](https://code.visualstudio.com/api/references/activation-events)
- [DSL-Based Workflow Orchestration](https://medium.com/@nareshvenkat14/dsl-based-workflow-orchestration-part-1-introduction-architecture-9d0112f77e00)
- [Workflow Core JSON/YAML Definitions](https://workflow-core.readthedocs.io/en/latest/json-yaml/)
- [Microsoft Agent Framework Overview](https://learn.microsoft.com/en-us/agent-framework/overview/)

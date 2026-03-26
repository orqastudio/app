---
id: RES-25f55a8d
title: "State Machine Design with Human Gates"
type: discovery-research
status: active
category: architecture
description: "Deterministic state machines — plugin-owned, YAML-defined, human gate sub-workflows, ad-hoc variants"
created: 2026-03-25
updated: 2026-03-25
tags:
  - agent-teams
  - plugin-architecture
---

# Research: State Machine Design with Human Gates

## Research Questions

1. How do workflow engines define state machines?
2. What are best practices for human-in-the-loop gates?
3. How should state machines be defined in plugin configuration?
4. What patterns exist for ad-hoc/fast-track workflows?
5. How should state machines compose across plugins?
6. What are canonical artifact type definitions?
7. How do systems handle state machine migration?
8. What sub-workflow patterns work for review gates?

---

## 1. Workflow Engine Comparison

### XState v5 (JavaScript — Statecharts)

XState implements the Harel statechart formalism with hierarchical states, parallel regions, guards, actions, and invoked actors.

**Definition format:** JavaScript objects (serializable to JSON).

```javascript
createMachine({
  id: 'epic',
  initial: 'draft',
  context: { reviewCount: 0 },
  states: {
    draft: {
      on: {
        SUBMIT: {
          target: 'ready',
          guard: ({ context }) => context.docsExist,
          actions: assign({ submittedAt: () => Date.now() })
        }
      }
    },
    ready: {
      on: { START: 'inProgress' }
    },
    inProgress: {
      type: 'parallel',
      states: {
        implementation: {
          initial: 'coding',
          states: {
            coding: { on: { COMPLETE: 'testing' } },
            testing: { on: { PASS: 'done' } },
            done: { type: 'final' }
          }
        },
        documentation: {
          initial: 'writing',
          states: {
            writing: { on: { PUBLISH: 'done' } },
            done: { type: 'final' }
          }
        }
      },
      onDone: 'review'
    },
    review: {
      on: {
        APPROVE: 'done',
        REJECT: 'inProgress'
      }
    },
    done: { type: 'final' }
  }
});
```

**Key strengths:**
- Parallel states (type: 'parallel') for concurrent workflows
- Guards on transitions for conditional logic
- Actions (entry, exit, transition) for side effects
- Invoked actors for long-running sub-processes
- Hierarchical (compound) states for nesting
- Final states that trigger parent completion
- Serializable to JSON for storage/transport
- W3C SCXML-compatible semantics

**Key limitations:**
- Code-first — requires JavaScript runtime for evaluation
- Guards are functions, not declarative expressions (though they can be named)
- No built-in human-in-the-loop primitives

### Temporal (Durable Execution)

Temporal takes a fundamentally different approach — it replaces explicit state machines with durable function execution. Workflow state is implicitly captured through code execution history.

**Definition format:** Code (TypeScript, Go, Java, Python).

```typescript
async function epicWorkflow(input: EpicInput): Promise<EpicResult> {
  // State is implicit in execution position
  await activities.validateDocs(input.epicId);

  // Human approval gate — waits indefinitely
  const approval = await workflow.waitCondition(
    () => signals.approved !== undefined
  );

  if (!approval) {
    return { status: 'rejected' };
  }

  // Parallel execution
  await Promise.all([
    activities.runImplementation(input),
    activities.writeDocumentation(input)
  ]);

  // Another human gate
  const reviewResult = await workflow.waitCondition(
    () => signals.reviewComplete !== undefined
  );

  return { status: 'done' };
}
```

**Key strengths:**
- Code-as-workflow — full programming language power
- Automatic state persistence via event sourcing
- Human-in-the-loop via signals and conditions (can wait indefinitely)
- Workflow versioning with patching for live migrations
- Child workflows for composition
- Deterministic replay for fault tolerance

**Key limitations:**
- Requires a Temporal server (heavy infrastructure)
- Determinism constraints — no random, no current time, no external I/O in workflow code
- State machine is implicit (harder to visualize)
- Overkill for simple status tracking

### AWS Step Functions (Amazon States Language)

Step Functions uses a JSON-based domain-specific language (ASL) to define state machines declaratively.

**Definition format:** JSON (ASL).

```json
{
  "StartAt": "ValidateDocs",
  "States": {
    "ValidateDocs": {
      "Type": "Task",
      "Resource": "arn:aws:lambda:...:validateDocs",
      "Next": "WaitForApproval"
    },
    "WaitForApproval": {
      "Type": "Task",
      "Resource": "arn:aws:states:::sqs:sendMessage.waitForTaskToken",
      "Parameters": {
        "QueueUrl": "...",
        "MessageBody": { "taskToken.$": "$$.Task.Token" }
      },
      "Next": "ApprovalChoice"
    },
    "ApprovalChoice": {
      "Type": "Choice",
      "Choices": [
        { "Variable": "$.approved", "BooleanEquals": true, "Next": "Implementation" }
      ],
      "Default": "Rejected"
    },
    "Implementation": {
      "Type": "Parallel",
      "Branches": [
        { "StartAt": "Code", "States": { "Code": { "Type": "Task", "End": true } } },
        { "StartAt": "Docs", "States": { "Docs": { "Type": "Task", "End": true } } }
      ],
      "Next": "Review"
    },
    "Review": {
      "Type": "Task",
      "Resource": "arn:aws:states:::sqs:sendMessage.waitForTaskToken",
      "Next": "Done"
    },
    "Done": { "Type": "Succeed" },
    "Rejected": { "Type": "Fail" }
  }
}
```

**Key strengths:**
- Purely declarative — no code in the state machine definition
- Built-in human approval via `.waitForTaskToken` pattern
- Choice states for conditional branching
- Parallel states for concurrent execution
- Wait states for time-based delays
- Map states for dynamic parallelism
- Visual editor in AWS console

**Key limitations:**
- AWS-locked — not portable
- Verbose JSON format
- Limited expression language
- No hierarchical/compound states
- No guard functions (only Choice state comparisons)

### Camunda / Flowable (BPMN 2.0)

BPMN (Business Process Model and Notation) is the industry standard for process workflow modeling. Both Camunda and Flowable implement it.

**Definition format:** BPMN 2.0 XML.

**Key concepts:**
- **User Tasks** — human activities with assignment (individual, group, role)
- **Service Tasks** — automated activities
- **Gateways** — exclusive (XOR), parallel (AND), inclusive (OR)
- **Sub-processes** — embedded or call activities for composition
- **Events** — start, end, intermediate (timer, message, signal, error)
- **Lanes/Pools** — organizational responsibility

**Human gate patterns:**
- Maker-checker: two separate User Tasks assigned to different roles
- Multi-instance: same task assigned to N reviewers in parallel
- Escalation: non-interrupting timer triggers an escalation task alongside the approval
- Four-eyes principle: parallel gateway splits to two reviewers, both must approve

**Key strengths:**
- Industry standard (BPMN 2.0)
- Rich human task management (assignment, delegation, escalation, SLA)
- Sub-processes for composition and reuse
- Process variables for data flow
- Timer events for deadlines and escalation
- Graphical notation with formal semantics

**Key limitations:**
- XML-based — verbose and hard to author by hand
- Heavy runtime (Java-based engines)
- Steep learning curve
- Over-engineered for simple status tracking

### Symfony Workflow Component (PHP)

Symfony provides a lightweight, configuration-driven state machine and workflow component.

**Definition format:** YAML.

```yaml
framework:
  workflows:
    epic:
      type: state_machine
      marking_store:
        type: method
        property: currentStatus
      supports:
        - App\Entity\Epic
      initial_marking: draft
      places:
        - draft
        - ready
        - in_progress
        - review
        - done
      transitions:
        submit:
          from: draft
          to: ready
          guard: "subject.hasRequiredDocs()"
        start:
          from: ready
          to: in_progress
        complete:
          from: in_progress
          to: review
        approve:
          from: review
          to: done
        reject:
          from: review
          to: in_progress
```

**Key strengths:**
- Clean YAML configuration
- Guard expressions on transitions
- Event system (guard, leave, transition, enter, completed, announce)
- Metadata on states and transitions
- Distinction between "workflow" (multi-place) and "state_machine" (single-place)

**Key limitations:**
- PHP-specific runtime
- No parallel states
- No hierarchical states
- No built-in human task management

### CFlow (@cflow/core — TypeScript)

CFlow is a YAML-driven, domain-agnostic state machine engine specifically designed for workflow orchestration.

**Definition format:** YAML.

```yaml
version: "1.0"
entry: DRAFT
metadata_keys:
  - name: assigned_to
    type: string
  - name: priority
    type: number
global_permissions:
  - role: admin
    can_view_all_cases: true
    can_perform_actions: [approve, reject]
  - role: reviewer
    can_perform_actions: [approve, reject]
actions:
  - name: submit
    handler: SubmitHandler
  - name: approve
    handler: ApproveHandler
states:
  - code: DRAFT
    name: Draft
    actions:
      - name: submit
        on_success: state: READY
        on_failure: state: DRAFT
  - code: READY
    name: Ready
    actions:
      - name: start
        on_success: state: IN_PROGRESS
  - code: IN_PROGRESS
    name: In Progress
    actions:
      - name: complete
        on_success:
          conditions:
            - if: "review_required"
              then: state: REVIEW
            - if: "true"
              then: state: DONE
  - code: REVIEW
    name: Review
    actions:
      - name: approve
        on_success: state: DONE
      - name: reject
        on_success: state: IN_PROGRESS
```

**Key strengths:**
- YAML-native configuration — human-readable, diffable
- Module composition — sub-workflows that push/pop on a stack
- Built-in permissions model (role-based)
- Conditional transitions with if/then expressions
- Event store for audit trails
- Handler-based actions (TypeScript classes)
- Domain-agnostic — works for any artifact type

**Key limitations:**
- No parallel states
- No hierarchical states
- Newer/less mature ecosystem
- TypeScript runtime required for handlers

### Summary Comparison Table

| Feature | XState | Temporal | Step Functions | BPMN | Symfony | CFlow |
|---------|--------|----------|---------------|------|---------|-------|
| **Format** | JS/JSON | Code | JSON (ASL) | XML | YAML | YAML |
| **Parallel states** | Yes | Via Promise.all | Yes | Yes (AND gateway) | No | No |
| **Hierarchical states** | Yes | Via child workflows | No | Yes (sub-process) | No | Yes (modules) |
| **Guards** | Functions | Code conditions | Choice state | Gateway conditions | Expressions | If/then conditions |
| **Human gates** | Custom | Signals/conditions | waitForTaskToken | User Tasks | Events | Permissions + handlers |
| **Composition** | Invoked actors | Child workflows | Nested machines | Call activities | N/A | Module stack |
| **Persistence** | External | Built-in (event sourcing) | Built-in | Built-in | External | Event store |
| **Migration** | N/A | Patching + versioning | New execution | Process versioning | N/A | N/A |
| **Declarative** | Semi | No | Yes | Yes | Yes | Yes |

---

## 2. State Machine Definition Format Options

### Option A: YAML with JSON Schema Validation

```yaml
# epic.workflow.yaml
id: epic-lifecycle
version: 1
artifact_type: epic
initial: draft
states:
  draft:
    category: planning
    description: "Epic is being designed"
    on:
      submit:
        target: ready
        guard:
          type: all_docs_exist
          field: docs-required
  ready:
    category: planning
    description: "Ready for implementation"
    on:
      start:
        target: in-progress
        guard:
          type: dependencies_met
  in-progress:
    category: active
    on:
      complete:
        target: review
      block:
        target: blocked
  blocked:
    category: active
    on:
      unblock:
        target: in-progress
  review:
    category: review
    gate:
      type: human
      sub_workflow: epic-review
    on:
      approve:
        target: done
      reject:
        target: in-progress
  done:
    category: completed
    terminal: true
```

**Pros:**
- Human-readable, easy to review in PRs
- Diffable — state changes show as clean diffs
- Validated by JSON Schema (same pattern as existing artifact schemas)
- Non-developers can understand and modify
- Consistent with OrqaStudio's existing YAML frontmatter pattern
- Plugin authors write configuration, not code

**Cons:**
- Expression language for guards needs careful design
- No IDE autocompletion without language server support
- Complex conditional logic becomes awkward

### Option B: JSON Statechart (XState-compatible)

```json
{
  "id": "epic-lifecycle",
  "version": 1,
  "initial": "draft",
  "states": {
    "draft": {
      "meta": { "category": "planning" },
      "on": {
        "SUBMIT": {
          "target": "ready",
          "guard": "docsExist"
        }
      }
    },
    "ready": {
      "meta": { "category": "planning" },
      "on": { "START": "inProgress" }
    },
    "inProgress": {
      "meta": { "category": "active" },
      "type": "parallel",
      "states": {
        "implementation": { "initial": "coding", "states": {} },
        "review": { "initial": "pending", "states": {} }
      }
    }
  }
}
```

**Pros:**
- Battle-tested format (XState ecosystem)
- Supports parallel and hierarchical states natively
- Large ecosystem of tools (visualizers, editors, testing)
- Formal statechart semantics (W3C SCXML compatible)
- Can be rendered visually by existing tools

**Cons:**
- JSON is harder to read/write than YAML (no comments)
- XState-specific conventions may not map cleanly to OrqaStudio's needs
- Guard functions need a resolution layer (named guards → implementations)
- Heavier than needed for simple artifact lifecycles

### Option C: Hybrid YAML with Statechart Semantics

```yaml
# Combines YAML readability with statechart power
id: epic-lifecycle
version: 1
artifact_type: epic
initial: draft

# State categories map to UI indicators
categories:
  planning: { color: blue, icon: compass }
  active: { color: green, icon: play }
  review: { color: amber, icon: eye }
  completed: { color: purple, icon: check }
  terminal: { color: gray, icon: archive }

states:
  draft:
    category: planning
    transitions:
      - event: submit
        target: ready
        guards: [docs_required_exist]
        actions: [set_submitted_date]

  ready:
    category: planning
    transitions:
      - event: start
        target: in_progress
        guards: [dependencies_met]

  in_progress:
    category: active
    transitions:
      - event: submit_for_review
        target: review
      - event: block
        target: blocked

  blocked:
    category: active
    transitions:
      - event: unblock
        target: in_progress

  review:
    category: review
    gate:
      type: human
      workflow: epic-review
      required_roles: [user]
    transitions:
      - event: approve
        target: done
        guards: [all_tasks_done, docs_produced_verified]
      - event: reject
        target: in_progress
        actions: [log_rejection_reason]

  done:
    category: completed
    terminal: true

# Named guards — resolved by the engine at runtime
guards:
  docs_required_exist:
    type: field_check
    field: docs-required
    condition: all_paths_exist
  dependencies_met:
    type: relationship_check
    relationship: depends-on
    condition: all_status_done
  all_tasks_done:
    type: query
    query: { type: task, epic: $self, status: { not: done } }
    condition: count_equals_zero
  docs_produced_verified:
    type: field_check
    field: docs-produced
    condition: all_paths_exist

# Named actions — resolved by the engine at runtime
actions:
  set_submitted_date:
    type: set_field
    field: submitted
    value: $now
  log_rejection_reason:
    type: append_log
    field: review-history
```

**Pros:**
- YAML readability with statechart-level expressiveness
- Guards are declarative (field checks, queries) — no code required
- Actions are declarative (set field, append log) — no code required
- Categories provide UI-level metadata alongside state logic
- Gate definitions integrate human review workflows inline
- JSON Schema validates the entire structure
- Plugin authors only write YAML, not code
- Engine handles guard/action resolution

**Cons:**
- Custom format — no existing tooling ecosystem
- No parallel state support (would need to be added)
- Guard expression language needs careful design to avoid becoming a programming language
- More complex than Option A but still simpler than Option B

### Recommendation: Option C (Hybrid YAML)

Option C is the best fit for OrqaStudio because:

1. **YAML matches the existing pattern** — all artifacts use YAML frontmatter, all configuration is YAML
2. **Declarative guards avoid code in plugins** — plugin authors define conditions, not functions
3. **Categories provide UI integration** — the state machine knows how to render itself
4. **Human gates are first-class** — not an afterthought bolted onto a code-first system
5. **JSON Schema validation** — same enforcement pattern as existing artifact schemas
6. **Diffable** — state machine changes show as clean, reviewable YAML diffs

---

## 3. Human Gate Sub-Workflow Patterns

### Pattern 1: Simple Approval Gate

The simplest human gate — a single reviewer approves or rejects.

```yaml
gate:
  type: human
  workflow: simple-approval
  steps:
    - role: user
      action: review
      inputs: [summary, changes, acceptance_criteria]
      outputs: [verdict, feedback]
      verdict_options: [approve, reject]
  on_approve:
    transition: done
  on_reject:
    transition: in_progress
    actions: [log_feedback]
```

**Use case:** Task completion, small scope changes.

### Pattern 2: Structured Review Gate (Maker-Checker)

Two-phase review: an AI agent reviews first, then a human reviews.

```yaml
gate:
  type: human
  workflow: structured-review
  steps:
    - role: ai_reviewer
      action: automated_review
      inputs: [acceptance_criteria, code_changes, docs_changes]
      outputs: [review_report, pass_fail, findings]
      auto_proceed_on: pass

    - role: user
      action: human_review
      inputs: [review_report, summary, scope_changes]
      outputs: [verdict, feedback, scope_decisions]
      verdict_options: [approve, approve_with_changes, reject]

  on_approve:
    transition: done
    actions: [update_docs_produced, log_completion]
  on_approve_with_changes:
    transition: in_progress
    actions: [create_change_tasks, log_feedback]
  on_reject:
    transition: in_progress
    actions: [log_rejection, create_fix_tasks]
```

**Use case:** Epic completion, milestone gates.

### Pattern 3: Multi-Reviewer Gate (Four-Eyes)

Multiple independent reviews required before proceeding.

```yaml
gate:
  type: human
  workflow: four-eyes-review
  parallel: true
  required_approvals: 2
  steps:
    - role: code_reviewer
      action: code_review
      inputs: [code_changes, coding_standards]
      outputs: [code_verdict, code_findings]

    - role: ux_reviewer
      action: ux_review
      inputs: [ui_changes, design_specs]
      outputs: [ux_verdict, ux_findings]

    - role: user
      action: stakeholder_review
      inputs: [feature_summary, demo_evidence]
      outputs: [stakeholder_verdict, feedback]

  aggregation: all_must_pass
  on_all_pass:
    transition: done
  on_any_fail:
    transition: in_progress
    actions: [create_fix_tasks_from_findings]
```

**Use case:** Major feature releases, compliance-sensitive changes.

### Pattern 4: Escalation Gate

Approval with time-based escalation.

```yaml
gate:
  type: human
  workflow: approval-with-escalation
  steps:
    - role: assigned_reviewer
      action: review
      timeout: 48h
      on_timeout:
        escalate_to: team_lead
        action: escalated_review
      outputs: [verdict, feedback]

  on_approve:
    transition: done
  on_reject:
    transition: in_progress
```

**Use case:** Time-sensitive approvals where a reviewer might be unavailable.

### Pattern 5: Scope Decision Gate

Not approve/reject — a decision point where the user chooses direction.

```yaml
gate:
  type: human
  workflow: scope-decision
  steps:
    - role: user
      action: scope_review
      inputs: [current_scope, proposed_changes, impact_analysis]
      outputs: [decision, rationale]
      decision_options:
        - id: proceed
          label: "Proceed as planned"
          transition: in_progress
        - id: descope
          label: "Remove items from scope"
          transition: in_progress
          actions: [update_scope, create_deferred_ideas]
        - id: expand
          label: "Add items to scope"
          transition: planning
          actions: [update_scope, create_new_tasks]
        - id: cancel
          label: "Cancel this work"
          transition: cancelled
          actions: [archive_tasks, log_cancellation]
```

**Use case:** Mid-epic scope changes, idea promotion decisions, partially-delivered triage.

### Sub-Workflow Structure (Generalized)

Every human gate follows this structure:

```
┌─────────────────────────────────────────────┐
│ Gate: <name>                                │
│                                             │
│  1. GATHER inputs                           │
│     - Collect data from artifact fields     │
│     - Run automated pre-checks              │
│     - Generate summary/report               │
│                                             │
│  2. PRESENT to reviewer(s)                  │
│     - Show inputs in structured format      │
│     - Display verdict options               │
│     - Provide context (related artifacts)   │
│                                             │
│  3. COLLECT decision                        │
│     - Reviewer provides verdict + feedback  │
│     - Record timestamp, reviewer, rationale │
│                                             │
│  4. EXECUTE outcome                         │
│     - Apply the transition                  │
│     - Run post-transition actions            │
│     - Log the decision in audit trail       │
│                                             │
│  5. LEARN (if applicable)                   │
│     - On FAIL: create/update lesson         │
│     - On pattern: suggest rule promotion    │
│     - Update recurrence tracking            │
└─────────────────────────────────────────────┘
```

---

## 4. Ad-Hoc Workflow Patterns

### The Problem

Not all work follows the full discovery-to-delivery pipeline. Bug reports, UX tweaks, small fixes, and quick improvements need fast-track workflows that still maintain quality gates but skip heavy planning.

### Pattern 1: Workflow Variants per Artifact Type

Define multiple workflow variants for the same artifact type, selected by a discriminator field.

```yaml
# task.workflow.yaml — standard workflow
id: task-standard
artifact_type: task
variant: standard
initial: todo
states:
  todo: { transitions: [{ event: start, target: in_progress }] }
  in_progress: { transitions: [{ event: complete, target: review }] }
  review:
    gate: { type: human, workflow: task-review }
    transitions:
      - { event: approve, target: done }
      - { event: reject, target: in_progress }
  done: { terminal: true }

---
# task-quickfix.workflow.yaml — expedited workflow
id: task-quickfix
artifact_type: task
variant: quickfix
initial: in_progress
states:
  in_progress:
    transitions:
      - event: complete
        target: verify
  verify:
    gate:
      type: automated
      checks: [lint, test, typecheck]
    transitions:
      - { event: pass, target: done }
      - { event: fail, target: in_progress }
  done: { terminal: true }
```

**How it works:**
- When creating a task, the user or orchestrator selects the workflow variant
- Standard tasks get full planning + review
- Quickfix tasks skip planning, use automated verification instead of human review
- Both produce the same "done" terminal state

### Pattern 2: Workflow Selection Rules

The plugin defines rules for automatic workflow selection based on artifact properties.

```yaml
workflow_rules:
  - match:
      priority: P1
      labels: [bug, hotfix]
    workflow: task-quickfix
  - match:
      priority: P1
      labels: [security]
    workflow: task-security-expedited
  - default:
    workflow: task-standard
```

### Pattern 3: Skip-to-State Transitions

Instead of separate workflows, allow certain transitions to skip intermediate states with appropriate guards.

```yaml
states:
  captured:
    transitions:
      - event: promote
        target: in_progress  # Skip exploring + shaping for urgent items
        guards: [is_urgent, user_approved_skip]
      - event: explore
        target: exploring  # Standard path
```

### Recommended Approach for OrqaStudio

Use **workflow variants** (Pattern 1) with **selection rules** (Pattern 2):

1. Each plugin defines a default workflow and optional variant workflows
2. Selection rules determine which variant applies based on artifact properties
3. All variants for the same artifact type share the same terminal states (so downstream dependencies work)
4. Quick-fix variants skip planning states but keep quality gates (automated checks replace human review for small changes)

**Ad-hoc workflow examples for OrqaStudio:**

| Scenario | Workflow | Key Difference from Standard |
|----------|----------|------------------------------|
| Bug fix (small) | `task-quickfix` | Skip planning, automated review only |
| UX tweak | `task-quickfix` | Skip planning, automated review only |
| Security fix | `task-security` | Skip planning, mandatory human review |
| Documentation fix | `task-docs-only` | Skip review entirely (no code changes) |
| Hotfix (production) | `task-hotfix` | Skip planning, expedited review, auto-deploy |

---

## 5. Plugin Composition Patterns for State Machines

### The Challenge

If the core framework defines an artifact's base lifecycle and a plugin extends it with additional states/transitions, how does composition work without conflicts?

### Pattern 1: Plugin Owns the Entire State Machine

**The plugin that defines an artifact type owns its state machine completely.** The core framework provides the state machine engine but does not define any artifact-specific states.

```
Core Framework:
  - State machine engine (evaluates transitions, guards, gates)
  - State category vocabulary (planning, active, review, completed, terminal)
  - Guard primitives (field_check, relationship_check, query)
  - Action primitives (set_field, append_log, create_artifact)
  - Human gate infrastructure

Plugin (e.g., software-kanban):
  - Defines: task artifact type + task workflow states + task transitions
  - Defines: epic artifact type + epic workflow states + epic transitions

Plugin (e.g., software-discovery):
  - Defines: idea artifact type + idea workflow states + idea transitions
  - Defines: research artifact type + research workflow states + research transitions
```

**Pros:**
- No inheritance conflicts — each plugin is self-contained
- Clear ownership — the plugin author controls the entire lifecycle
- No "base + override" complexity

**Cons:**
- No shared states across plugins (duplication if multiple plugins have similar lifecycles)
- Core framework can't enforce any lifecycle patterns

### Pattern 2: Category-Based Composition (Recommended)

The core framework defines state **categories** (not states). Plugins define states that map to categories. The framework uses categories for cross-cutting concerns (UI rendering, dashboard aggregation, health checks).

```yaml
# Core framework defines categories
categories:
  planning: { description: "Work being designed/scoped", ui: { color: blue } }
  active: { description: "Work in progress", ui: { color: green } }
  review: { description: "Work being reviewed", ui: { color: amber } }
  completed: { description: "Work finished", ui: { color: purple } }
  terminal: { description: "Final state, no further transitions", ui: { color: gray } }

# Plugin maps its states to categories
# software-kanban/task.workflow.yaml
states:
  backlog: { category: planning }
  todo: { category: planning }
  in_progress: { category: active }
  code_review: { category: review }
  testing: { category: review }
  done: { category: completed, terminal: true }
  cancelled: { category: terminal }
```

**This is exactly the pattern Azure DevOps uses** — they define four state categories (Proposed, In Progress, Resolved, Completed) and plugins/custom processes map their states into those categories. The categories drive board columns, reporting, and aggregation.

**Pros:**
- Plugins have full control over states and transitions
- Core framework can render any artifact type generically using categories
- Dashboard aggregation works across artifact types ("show me everything in review")
- No inheritance conflicts
- New plugins work immediately with existing UI

**Cons:**
- Category vocabulary is fixed (adding new categories is a breaking change)
- Some cross-cutting behaviors (like "all review states need gates") must be enforced by convention, not by the engine

### Pattern 3: Extension Points (Hooks)

The plugin defines the base state machine, and extension points allow other plugins to add transitions or modify guards.

```yaml
# Base plugin defines the state machine with extension points
states:
  review:
    transitions:
      - event: approve
        target: done
    extension_points:
      - id: pre_approve
        type: guard  # Other plugins can add guards
      - id: post_approve
        type: action  # Other plugins can add actions

# Extension plugin hooks into the extension point
extensions:
  - target: epic-lifecycle.review.pre_approve
    guard:
      type: custom
      handler: SecurityReviewRequired
  - target: epic-lifecycle.review.post_approve
    action:
      type: custom
      handler: NotifyStakeholders
```

**Pros:**
- Base workflow is stable; extensions add without modifying
- Multiple plugins can extend the same workflow
- Clean separation of concerns

**Cons:**
- Extension point discovery is hard (what can I extend?)
- Ordering conflicts when multiple plugins extend the same point
- Debugging becomes harder (where did this guard come from?)

### Recommendation for OrqaStudio

**Use Pattern 2 (Category-Based Composition) as the primary model, with Pattern 1 (Plugin Owns) as the ownership principle.**

- The core framework defines state categories and the state machine engine
- Each plugin defines its own artifact types with complete state machines
- States map to categories for cross-cutting UI and reporting
- No inheritance, no extension points initially (add if needed later)
- The plugin's `schema.json` defines the valid status values (already exists)
- The plugin's `workflow.yaml` defines the transitions and gates (new)

---

## 6. Canonical Artifact Type Definitions

Each artifact type has a distinct purpose in the governance system. Here are clear criteria for what makes each type unique:

### Decision (AD-NNN)

**What it is:** A record of a significant technical or architectural choice.

**Distinguishing criteria:**
- Records a specific choice that was made (not a rule to follow)
- Has alternatives that were considered and rejected
- Has consequences (what this enables, what this constrains)
- Is immutable once accepted — superseded by new decisions, never modified
- Has a lifecycle: proposed → accepted → superseded/deprecated

**Is NOT:**
- A rule (rules prescribe behavior; decisions record choices)
- A guideline (guidelines suggest; decisions commit)
- Knowledge (knowledge explains; decisions justify a specific choice)

**Example:** "We chose SQLite for conversation persistence because..." (AD-859ed163)

### Rule (RULE-NNN)

**What it is:** An enforceable constraint on how work is done.

**Distinguishing criteria:**
- Prescribes behavior ("do this", "don't do that")
- Has enforcement mechanisms (automated or behavioral)
- Is active or inactive (not superseded — that's for decisions)
- Can be violated — and violations are detectable
- Often promoted from lessons that recurred

**Is NOT:**
- A decision (decisions record choices; rules enforce them)
- Knowledge (knowledge explains how; rules mandate what)
- A lesson (lessons observe patterns; rules mandate behavior)

**Example:** "No `unwrap()` in production Rust code" (RULE-9814ec3c)

### Knowledge (KNOW-NNN)

**What it is:** Reusable domain expertise that agents need to do work correctly.

**Distinguishing criteria:**
- Explains how something works or how to do something
- Is injected into agent context before relevant work
- Is factual/procedural, not prescriptive (though it may reference rules)
- Has a layer (core = universal, project = this project only)
- Is consumed by agents, not directly by users

**Is NOT:**
- A rule (rules mandate; knowledge informs)
- A decision (decisions choose; knowledge explains)
- Documentation (docs are for humans; knowledge is for agents)

**Example:** "How IPC patterns work in Tauri v2" (KNOW-ipc-patterns)

### Lesson (IMPL-NNN)

**What it is:** An observation from a specific incident that may become a pattern.

**Distinguishing criteria:**
- Records something that went wrong (or right) in a specific context
- Has a recurrence count — tracks how many times this has been observed
- Has a promotion path (lesson → rule, lesson → knowledge, lesson → decision)
- Is temporary by nature — either promoted or obsoleted
- Is created by review agents during verification

**Is NOT:**
- A rule (yet — it becomes one if it recurs enough)
- Knowledge (yet — it becomes knowledge if it's a reusable pattern)
- A bug report (bugs are tasks; lessons are patterns)

**Example:** "Agent forgot to rebuild binary after Rust change — 3rd time" (IMPL-xyz)

### Epic (EPIC-NNN)

**What it is:** A body of work with defined scope, acceptance criteria, and deliverables.

**Distinguishing criteria:**
- Has a scope (what's in, what's out)
- Has tasks (breakdown of the work)
- Has documentation gates (docs-required, docs-produced)
- Has a human gate for completion (user approves "done")
- Lives within a milestone
- Contains the implementation design in its body

**Is NOT:**
- A task (epics contain tasks; tasks are atomic work items)
- An idea (ideas become epics when promoted)
- A milestone (milestones contain epics; milestones measure progress)

### Task (TASK-NNN)

**What it is:** An atomic unit of work within an epic.

**Distinguishing criteria:**
- Has specific acceptance criteria (verifiable conditions)
- Is assigned to a role (implementer, reviewer, writer, etc.)
- Has dependencies (depends-on other tasks)
- Is small enough to fit in one agent context window
- Is verified by an independent reviewer

**Is NOT:**
- An epic (tasks are atomic; epics are collections)
- A bug (bugs are tasks with a quickfix workflow variant)
- An idea (ideas propose; tasks execute)

### Idea (IDEA-NNN)

**What it is:** A captured possibility that may or may not become work.

**Distinguishing criteria:**
- Starts as a raw capture ("what if we...")
- Goes through a shaping process (exploring → shaped)
- May be promoted to an epic or discarded
- Has research-needed items that must be investigated before promotion
- Is the entry point for all new work

**Is NOT:**
- A task (ideas propose; tasks execute)
- A decision (ideas explore; decisions commit)
- A bug (bugs are immediate; ideas are future possibilities)

### Research (RES-NNN)

**What it is:** An investigation that produces findings to inform decisions.

**Distinguishing criteria:**
- Has specific research questions
- Produces findings (factual results of investigation)
- Feeds into epics (via research-refs) and decisions
- Is historical — preserved even when surpassed by newer research
- Does not prescribe action (findings inform; decisions prescribe)

**Is NOT:**
- A decision (research informs decisions; it doesn't make them)
- Knowledge (research is one-time investigation; knowledge is reusable)
- Documentation (research is historical; docs describe current state)

### Documentation (DOC-NNN)

**What it is:** A description of the current target state of a feature, system, or process.

**Distinguishing criteria:**
- Describes what IS (or should be) — not what was
- Is deleted and replaced when outdated (not preserved like research)
- Is the source of truth for implementation
- Has pillar alignment (every doc page serves at least one pillar)
- Is human-readable (unlike knowledge, which is agent-readable)

**Is NOT:**
- Research (docs describe current state; research describes findings)
- Knowledge (docs are for humans; knowledge is for agents)
- A rule (docs describe; rules prescribe)

### Milestone (MS-NNN)

**What it is:** A strategic goal marker that aggregates epics.

**Distinguishing criteria:**
- Has a gate question ("Can we answer yes to: ...?")
- Contains epics (via epic.milestone references)
- Has a lifecycle: planning → active → complete
- Completion requires all P1 epics to be done
- Measures strategic progress, not task-level progress

---

## 7. State Migration Strategies

When a plugin updates its state machine definition (adds states, removes states, renames states), existing artifacts need migration.

### Strategy 1: Forward-Compatible Addition

**When:** Adding a new state to an existing workflow.

**Approach:** New states are added without affecting existing artifacts. Artifacts in old states remain valid. New transitions to/from the new state are only available going forward.

```yaml
# v1: draft → ready → in_progress → done
# v2: draft → ready → in_progress → review → done (added "review")

migration:
  version: 2
  changes:
    - type: add_state
      state: review
      category: review
    - type: add_transition
      from: in_progress
      event: submit_for_review
      target: review
    - type: add_transition
      from: review
      event: approve
      target: done
    # Existing in_progress → done transition remains for v1 artifacts
```

**No data migration needed.** Existing artifacts in `in_progress` can still transition directly to `done` (the old path) or go through `review` (the new path).

### Strategy 2: Status Mapping

**When:** Renaming a state or reorganizing the state machine.

**Approach:** Define a mapping from old status values to new status values. Run a migration script that updates all affected artifacts.

```yaml
migration:
  version: 3
  mappings:
    # Old status → New status
    "todo": "backlog"
    "in-progress": "active"
    "done": "completed"
  script: |
    for each artifact where status in mappings.keys:
      artifact.status = mappings[artifact.status]
      artifact.updated = now()
```

### Strategy 3: Dual-Write Period

**When:** Complex state machine restructuring.

**Approach:** During a transition period, the engine accepts both old and new status values. Old values are automatically mapped to new values on read. After all artifacts are migrated, the old values are removed.

```yaml
migration:
  version: 4
  dual_write:
    period: "until all artifacts migrated"
    accept_old: true
    read_mapping:
      "todo": "backlog"
      "in-progress": "active"
    write_format: new  # New writes always use new values
```

### Strategy 4: Temporal's Patching Approach

**When:** Running workflows that cannot be interrupted.

**Approach:** Inspired by Temporal, each state machine definition is versioned. Active workflows continue on the version they started with. New workflows use the latest version. This avoids migrating in-flight work entirely.

```yaml
# The engine stores the workflow version with each artifact
# artifact.workflow_version: 1

# When evaluating transitions:
# if artifact.workflow_version < current_version:
#   use the state machine definition from artifact.workflow_version
# else:
#   use the current state machine definition
```

### Recommended Strategy for OrqaStudio

Use **Strategy 1 (forward-compatible addition) as the default**, with **Strategy 2 (status mapping) for breaking changes**.

Rationale:
- Most state machine changes are additive (adding states, adding transitions)
- OrqaStudio artifacts are file-based with YAML frontmatter — easy to batch-update
- The `orqa migrate` CLI command can run status mappings
- Version pinning (Strategy 4) is over-engineered for file-based artifacts that aren't "running"

**Migration protocol:**
1. Plugin updates its `workflow.yaml` with a `migration` section
2. `orqa migrate` reads all artifacts of the affected type
3. For each artifact with an old status, applies the mapping
4. Updates the artifact's frontmatter
5. Commits the changes with a descriptive message

---

## 8. Review Sub-Workflow Design

### The Review Pipeline (Generalized)

Every review gate follows a pipeline structure:

```
TRIGGER (artifact reaches review state)
  │
  ▼
GATHER (collect review inputs)
  │ - Read acceptance criteria from artifact
  │ - Read related changes (files, artifacts)
  │ - Read relevant standards (rules, knowledge)
  │ - Generate summary report
  │
  ▼
AI REVIEW (automated first pass)
  │ - Run quality checks (lint, test, typecheck)
  │ - Evaluate acceptance criteria programmatically
  │ - Check for known patterns (lesson matches)
  │ - Produce structured review report
  │
  ├── AUTO-PASS (all checks pass, criteria met)
  │   │
  │   ▼
  │   HUMAN REVIEW (user reviews AI findings)
  │   │ - Present AI review report
  │   │ - Present acceptance criteria status
  │   │ - Present scope checklist
  │   │ - Collect: approve / approve-with-changes / reject
  │   │
  │   ├── APPROVE → transition to done
  │   ├── APPROVE-WITH-CHANGES → create change tasks, stay in review
  │   └── REJECT → log feedback, transition to in_progress
  │
  └── AUTO-FAIL (checks fail or criteria unmet)
      │
      ▼
      FAIL WORKFLOW
      │ - Log failure findings
      │ - Create/update lesson (IMPL-NNN)
      │ - Create fix tasks
      │ - Transition to in_progress
      │ - Increment lesson recurrence if pattern matches
```

### Review Roles and Their Responsibilities

| Role | What They Review | Verdict Produces |
|------|-----------------|------------------|
| **Code Reviewer** (AI) | Code quality, standards compliance, test coverage | PASS/FAIL with findings list |
| **QA Tester** (AI) | Functional correctness, end-to-end wiring, smoke tests | PASS/FAIL with test results |
| **UX Reviewer** (AI) | UI compliance with specs, component usage, accessibility | PASS/FAIL with UX findings |
| **User** (Human) | Business value, scope completeness, strategic alignment | Approve/Reject with rationale |

### Review Order

The AI reviews run first (they're fast and catch mechanical issues). Only if AI reviews pass does the human review gate activate. This prevents humans from reviewing work that has known defects.

```
AI Reviews (parallel):
  ├── Code Review Agent → PASS/FAIL
  ├── QA Test Agent → PASS/FAIL
  └── UX Review Agent → PASS/FAIL
      │
      All PASS?
      ├── Yes → Human Review Gate activates
      └── No → Fail workflow (fix and resubmit)
```

### Evidence Requirements in Reviews

A review is only valid if it includes evidence. The sub-workflow definition specifies what evidence is required:

```yaml
review_evidence:
  code_review:
    required:
      - lint_output: "make lint output showing zero warnings"
      - test_output: "make test output showing all pass"
      - type_check: "make typecheck output showing no errors"
    optional:
      - coverage_report: "test coverage percentage"

  functional_review:
    required:
      - smoke_test: "description of what user would see"
      - end_to_end_trace: "chain from UI → store → IPC → backend"
    optional:
      - screenshot: "visual evidence of working feature"

  human_review:
    required:
      - scope_checklist: "all scope items checked"
      - decision_rationale: "why approve/reject"
    optional:
      - feedback: "additional comments for the team"
```

### Fail Path: Learning Integration

When a review fails, the sub-workflow includes lesson creation:

```yaml
on_fail:
  steps:
    - action: search_existing_lessons
      query: { tags: $failure_tags }

    - action: update_or_create_lesson
      conditions:
        - if: existing_lesson_found
          then: increment_recurrence
        - if: no_existing_lesson
          then: create_new_lesson

    - action: check_promotion_threshold
      conditions:
        - if: recurrence >= 2
          then: flag_for_promotion

    - action: create_fix_tasks
      from: review_findings

    - action: transition
      target: in_progress
```

---

## Recommended Architecture for OrqaStudio

### Summary

1. **Format:** Hybrid YAML (Option C) — readable, declarative, schema-validated
2. **Ownership:** Plugin owns the complete state machine for its artifact types
3. **Composition:** Category-based — core defines categories, plugins define states that map to categories
4. **Human gates:** First-class sub-workflows with gather → AI review → human review pipeline
5. **Ad-hoc workflows:** Workflow variants with selection rules
6. **Migration:** Forward-compatible addition by default, status mapping for breaking changes
7. **Engine:** Core framework provides the state machine evaluation engine; plugins provide definitions

### File Structure

```
plugins/
  software-kanban/
    artifacts/
      task/
        schema.json           # Artifact schema (already exists)
        workflow.yaml          # State machine definition (NEW)
        workflow-quickfix.yaml # Variant workflow (NEW)
      epic/
        schema.json
        workflow.yaml

  software-discovery/
    artifacts/
      idea/
        schema.json
        workflow.yaml
      research/
        schema.json
        workflow.yaml

  governance/
    artifacts/
      rule/
        schema.json
        workflow.yaml
      decision/
        schema.json
        workflow.yaml
      lesson/
        schema.json
        workflow.yaml
      knowledge/
        schema.json
        workflow.yaml
```

### Core Framework Provides

```yaml
# core/workflow-engine.yaml (or equivalent code)
categories:
  planning:
    description: "Work being designed or scoped"
    ui: { color: blue, icon: compass }
  active:
    description: "Work in progress"
    ui: { color: green, icon: play }
  review:
    description: "Work being reviewed or verified"
    ui: { color: amber, icon: eye }
  completed:
    description: "Work finished successfully"
    ui: { color: purple, icon: check }
  terminal:
    description: "Final state — no further transitions"
    ui: { color: gray, icon: archive }

guard_primitives:
  field_check:
    description: "Check an artifact field against a condition"
    params: [field, condition]
  relationship_check:
    description: "Check related artifacts meet a condition"
    params: [relationship, condition]
  query:
    description: "Run a graph query and check results"
    params: [query, condition]
  role_check:
    description: "Check if current user has a required role"
    params: [required_roles]

action_primitives:
  set_field:
    description: "Set a field on the artifact"
    params: [field, value]
  append_log:
    description: "Append to an audit log field"
    params: [field, entry]
  create_artifact:
    description: "Create a new artifact"
    params: [type, fields]
  notify:
    description: "Send a notification"
    params: [channel, message]

gate_types:
  human:
    description: "Requires human approval"
    params: [workflow, required_roles]
  automated:
    description: "Requires automated checks to pass"
    params: [checks]
  ai_then_human:
    description: "AI reviews first, human approves"
    params: [ai_workflow, human_workflow]
```

### Complete Example: Epic Workflow

```yaml
id: epic-lifecycle
version: 2
artifact_type: epic
initial: draft

states:
  draft:
    category: planning
    description: "Epic is being designed and scoped"
    transitions:
      - event: submit
        target: ready
        guards:
          - type: field_check
            field: docs-required
            condition: all_paths_exist
          - type: field_check
            field: description
            condition: not_empty
        actions:
          - type: set_field
            field: submitted
            value: $now

  ready:
    category: planning
    description: "Ready for implementation — all prerequisites met"
    transitions:
      - event: start
        target: in_progress
        guards:
          - type: relationship_check
            relationship: depends-on
            condition: all_status_done

  in_progress:
    category: active
    description: "Implementation underway"
    transitions:
      - event: submit_for_review
        target: review
        guards:
          - type: query
            query: { type: task, epic: $self, status: { not: done } }
            condition: count_equals_zero
      - event: block
        target: blocked

  blocked:
    category: active
    description: "Blocked by external dependency"
    transitions:
      - event: unblock
        target: in_progress

  review:
    category: review
    description: "Under review — human approval required"
    gate:
      type: ai_then_human
      ai_workflow:
        checks:
          - type: code_review
            acceptance: $self.acceptance
          - type: docs_check
            field: docs-produced
      human_workflow:
        role: user
        inputs: [summary, task_status, docs_produced, lessons_logged, scope_changes]
        outputs: [verdict, feedback]
        verdict_options: [approve, reject, approve_with_changes]
    transitions:
      - event: approve
        target: done
        guards:
          - type: field_check
            field: docs-produced
            condition: all_paths_exist
        actions:
          - type: set_field
            field: completed
            value: $now
      - event: reject
        target: in_progress
        actions:
          - type: append_log
            field: review-history
            entry: { verdict: reject, feedback: $feedback, date: $now }
      - event: approve_with_changes
        target: in_progress
        actions:
          - type: append_log
            field: review-history
            entry: { verdict: changes_requested, feedback: $feedback, date: $now }

  done:
    category: completed
    terminal: true
    description: "Epic completed and approved"

migration:
  from_version: 1
  changes:
    - type: add_state
      state: blocked
      category: active
    - type: add_transition
      from: in_progress
      event: block
      target: blocked
```

### Complete Example: Task Quick-Fix Workflow

```yaml
id: task-quickfix
version: 1
artifact_type: task
variant: quickfix
initial: in_progress

states:
  in_progress:
    category: active
    description: "Fix in progress"
    transitions:
      - event: submit
        target: verify

  verify:
    category: review
    description: "Automated verification"
    gate:
      type: automated
      checks:
        - lint
        - test
        - typecheck
    transitions:
      - event: pass
        target: done
      - event: fail
        target: in_progress

  done:
    category: completed
    terminal: true
```

---

## Key Design Principles

1. **Plugin owns the state machine.** The plugin that defines an artifact type defines its workflow. The core framework provides the engine, not the definitions.

2. **States are deterministic.** Given a current status and an event, there is exactly one valid next status (after guard evaluation). No ambiguity.

3. **Categories bridge plugins and UI.** The core framework knows about categories (planning, active, review, completed, terminal). Plugins map their states to categories. The UI renders categories, not states.

4. **Human gates are sub-workflows, not boolean flags.** A gate isn't just approve/reject — it's a structured process with inputs, reviewers, outputs, and paths for both outcomes.

5. **Guards are declarative, not code.** Guards are field checks, relationship checks, and graph queries — not arbitrary functions. This keeps state machines portable across plugins.

6. **Variants handle ad-hoc work.** Instead of one-size-fits-all workflows, plugins define variant workflows for different work patterns (standard, quickfix, security, docs-only).

7. **Migration is additive by default.** Adding states and transitions is non-breaking. Renaming or removing states requires explicit migration mappings.

8. **The learning loop is built into review gates.** Every FAIL path creates or updates a lesson. Promotion thresholds are checked automatically.

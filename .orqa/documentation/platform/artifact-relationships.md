---
id: DOC-070
type: doc
name: Artifact Relationships Reference
status: active
layer: core
relationships:
  - target: SKILL-050
    type: synchronised-with
---

# Artifact Relationships Reference

OrqaStudio connects artifacts through typed relationships. Each relationship uses a specific verb that describes what the connection means. The verb constrains which types of artifacts can participate — if the sentence doesn't read naturally, the relationship shouldn't exist.

## How to Read This

Every relationship is shown as: **From** → verb → **To**

Example: "Decision drives Epic" means a Decision artifact can have a `drives` relationship pointing to an Epic artifact. The inverse is automatic: the Epic gets a `driven-by` relationship pointing back to the Decision.

## The Flow

Everything traces back to the vision through a natural chain:

```
Vision
  ↑ upholds
Pillar
  ↑ grounded-by
Idea ──→ benefits ──→ Persona
  │
  ├── crystallises ──→ Decision ──→ drives ──→ Epic
  │                       │                      ↑
  │                       └── governs ──→ Rule   │ delivers
  │                                        │     Task
  ├── spawns ──→ Research                  │       │
  │                │                       │       └── yields ──→ Lesson
  │                ├── produces → Wireframe │                       │
  │                ├── informs → Decision   │                       ├── teaches → Decision
  │                └── guides → Epic        │                       ├── cautions → Epic
  │                                         │                       └── codified-by → Rule
  └── realises ──→ Epic / Task              │
                                            └── enforces → Decision
```

## Foundation Relationships

These anchor everything to the project's principles.

| Relationship | Meaning | Example |
|---|---|---|
| Pillar **upholds** Vision | A pillar supports the project vision | "Clarity Through Structure upholds the OrqaStudio vision" |
| Idea **grounded-by** Pillar | An idea is anchored to a principle | "Plugin system idea grounded by Clarity Through Structure" |
| Idea **benefits** Persona | An idea serves a user type | "Plugin browser idea benefits the Plugin Developer persona" |
| Pivot **revises** Vision/Persona/Pillar | A foundational change | "Pivot-001 revises the product vision after user research" |

**Key insight:** If an idea can't be grounded by a pillar or doesn't benefit a persona, it's a signal. Either the idea should be discarded, or the vision/personas/pillars need to change — that's a pivot.

## Lineage Relationships

These track how ideas become other things.

| Relationship | Meaning |
|---|---|
| Idea **crystallises** into Decision | An idea becomes a concrete choice |
| Idea **spawns** Research | An idea triggers investigation |
| Idea **merged-into** Idea | Multiple ideas consolidated into one |

## Governance Relationships

These connect decisions to the work they motivate and the rules they establish.

| Relationship | Meaning |
|---|---|
| Decision **drives** Epic | A choice motivates a body of work |
| Decision **governs** Rule | A choice establishes a governance rule |
| Rule **enforces** Decision | A rule guards a decision |
| Rule **codifies** Lesson | A rule makes a lesson enforceable |

**Two paths from decisions:** `drives` leads into delivery (building things). `governs` leads into the learning loop (establishing standards). This is an important distinction — not all decisions produce work, some produce governance.

## Knowledge Flow Relationships

These move knowledge between artifacts.

| Relationship | Meaning |
|---|---|
| Research **informs** Decision | Findings shape a choice |
| Research **guides** Epic | Findings shape delivery work |
| Lesson **teaches** Decision | Past experience shapes a future choice |
| Lesson **cautions** Epic | Past experience warns current work |
| Doc **documents** Epic/Decision/Rule/Milestone | Human-readable description |

**Specific verbs matter:** Research `informs` decisions (factual input) but `guides` epics (directional input). Lessons `teach` decisions (learning from mistakes) but `caution` epics (warnings during execution).

## Agent Relationships

| Relationship | Meaning |
|---|---|
| Agent **observes** Epic/Task/Decision/Rule/Milestone | Agent monitors an artifact |
| Agent **employs** Skill | Agent uses a skill capability |
| Skill **synchronised-with** Doc | Agent-facing and human-facing versions |

## Plugin Relationships

The software-project plugin adds delivery-specific relationships. See the Software Delivery Guide (DOC-SW-001) for the complete reference. Key additions:

| Relationship | Meaning |
|---|---|
| Task **delivers** to Epic | Work rolls up to parent |
| Epic **fulfils** Milestone | Work completes a checkpoint |
| Task **depends-on** Task | Sequencing constraint |
| Task **yields** Lesson | Work produces learning |
| Idea **realises** Epic/Task | Idea becomes delivery work |
| Research **produces** Wireframe | Investigation yields spec |
| Bug **reports** against Epic/Task/Milestone | Issue reported |
| Task **fixes** Bug | Corrective work |
| Bug **affects** Persona | Issue impacts user type |

## Automatic Status Transitions

The graph computes status transitions from relationship state:

- **Blocked**: a task with unmet `depends-on` targets automatically becomes `blocked`
- **Unblocked**: when all dependencies complete, the task returns to `ready`
- **Review**: when all child tasks complete, the parent epic moves to `review`
- **Milestone review**: when all child epics complete, the milestone moves to `review`
- **Unaddressed lessons**: lessons without `codified-by` are surfaced as unresolved learning
- **Ungrounded ideas**: ideas without `grounded-by` or `benefits` are surfaced as warnings

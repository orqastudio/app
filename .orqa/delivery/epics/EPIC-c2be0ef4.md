---
id: "EPIC-c2be0ef4"
type: "epic"
title: "Inference-time decision tree: self-navigating knowledge discovery"
description: "Implement the self-navigating knowledge discovery model. Replace pre-injection with a reasoning protocol that agents work through to find their own knowledge. Create the decision tree as a governance artifact, update the hook to inject it, and make it plugin-extensible."
status: "review"
created: 2026-03-21T00:00:00.000Z
updated: 2026-03-21T00:00:00.000Z
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Epic fulfils this milestone"
---

# EPIC-101: Inference-Time Decision Tree — Self-Navigating Knowledge Discovery

## Problem

The current `UserPromptSubmit` hook pre-injects knowledge into agents using an INTENT_MAP — a hardcoded keyword-to-knowledge mapping that guesses what knowledge is relevant before the agent has reasoned about the prompt. This wastes context tokens on irrelevant knowledge, provides no visibility into why knowledge was selected, and cannot adapt to novel prompts.

## Design

See AD-064 for the architecture decision.

### Phase 1 — Create the decision tree artifact

Create the core decision tree in `.orqa/process/knowledge/decision-tree.md`:

- Structured questions for Level 1 (intent), Level 2 (domain), Level 3 (specific knowledge)
- Each leaf references knowledge files by path only — no inline content
- Three-level structure: intent → domain → specific knowledge
- Software plugin extends with delivery-specific branches (UI component, Rust command, Store, Artifact)
- Schema: tree nodes have `question`, `answers`, and `leaf` (array of knowledge file paths)
- Validation: every leaf path must point to a real file in `.orqa/process/knowledge/` or plugin knowledge directories

### Phase 2 — Update the prompt-injector hook

Rewrite `prompt-injector.ts` (`plugins/claude/hooks/`):

- On `UserPromptSubmit`, inject the decision tree (not knowledge content)
- The tree is concise — fits in approximately 200 tokens
- Remove INTENT_MAP fallback entirely (replaced by the tree)
- Keep semantic search as an enhancement: the agent can call `search_semantic` to refine its Level 2 or Level 3 navigation when the tree alone is ambiguous
- Inject format: a compact markdown block with the tree structure and navigation instructions

### Phase 3 — Agent-specific trees

Each agent role gets questions tailored to its boundary:

| Agent | Focus questions |
|-------|----------------|
| Orchestrator | Who does this work? What rules apply? What pillar does it serve? |
| Implementer | What layer does this touch? What patterns apply? What tests are needed? |
| Reviewer | What is the acceptance criteria? What standards apply? What to verify? |
| Governance Steward | What schema applies? What inverses are required? What artifacts are related? |

Base tree is shared. Role-specific extensions are declared in each agent's frontmatter under `decision-tree-extensions`. Agent definitions reference the base tree plus their extensions.

### Phase 4 — Plugin extension model

Plugins declare decision tree branches in their manifest under `provides.decision_tree`:

```yaml
provides:
  decision_tree:
    - intent: implementation
      domain: frontend
      answers:
        - match: "UI component"
          knowledge: ["plugins/svelte/knowledge/svelte5-best-practices.md", "plugins/svelte/knowledge/component-extraction.md"]
        - match: "Store"
          knowledge: ["plugins/svelte/knowledge/orqa-store-patterns.md"]
```

Merged at install time using the same merge mechanism as all other plugin schemas. Tree leaves are validated against knowledge file existence at install time — a plugin cannot declare a leaf that points to a non-existent file.

## Acceptance Criteria

- [ ] Decision tree artifact exists at `.orqa/process/knowledge/decision-tree.md` and is browsable in the app
- [ ] `UserPromptSubmit` hook injects tree instead of knowledge content
- [ ] INTENT_MAP removed from `prompt-injector.ts`
- [ ] Token usage for injection decreases (tree fits in ~200 tokens vs current injection)
- [ ] Agent reasoning shows tree navigation in thinking blocks
- [ ] Orchestrator, Implementer, and Reviewer agents have role-specific tree extensions declared in their frontmatter
- [ ] Software plugin extends the tree with implementation-specific branches
- [ ] At least one additional plugin (cli or svelte) extends the tree
- [ ] Tree leaf validation runs at install time — every leaf points to a real file
- [ ] Graceful degradation: tree works without the search server (agent's own reasoning suffices for classification)
- [ ] `make check` passes after all changes

## Risks

- **Tree verbosity** — a deeply branched tree may exceed the 200-token target. Mitigation: keep Level 3 leaves as file paths only (one line each), collapse rarely-used branches.
- **Agent compliance** — agents may not follow the tree consistently. Mitigation: the injection format includes explicit instructions to work through the tree before acting; thinking blocks provide inspection.
- **Plugin merge conflicts** — two plugins adding branches for the same Level 2 domain. Mitigation: merge strategy uses last-write-wins per match string, documented in the tree schema.
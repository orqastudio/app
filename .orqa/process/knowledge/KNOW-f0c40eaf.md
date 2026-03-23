---
id: KNOW-f0c40eaf
title: Composability
description: |
  Universal composability philosophy: building systems from small, pure, swappable units.
  Shapes how agents think about code structure at both the function level and the feature level.
  Use when: Writing any new code, reviewing architecture, planning features, or refactoring
  existing modules. This is the meta-skill — it informs all other patterns.
status: active
created: 2026-03-01
updated: 2026-03-13
category: methodology
version: 1.0.0
user-invocable: true
relationships:
  - target: AGENT-c5284fde
    type: employed-by
  - target: AGENT-ff44f841
    type: employed-by
  - target: AGENT-cc255bc8
    type: employed-by
  - target: AGENT-1dab5ebe
    type: employed-by
  - target: AGENT-caff7bc1
    type: employed-by
  - target: AGENT-fb0ce261
    type: employed-by
  - target: AGENT-b0774726
    type: employed-by
  - target: AGENT-ec1b3785
    type: employed-by
  - target: DOC-a1b2c3d4
    type: synchronised-with

---

Software is built from small, composable units at every level: functions, modules, features, and integrations. This skill teaches the composability philosophy that shapes all code in a well-structured project.

Composability is not just a coding pattern — it is a platform principle. Every project should be composable by default, because composable systems are dramatically easier to pivot, extend, and maintain as requirements change.

## The Core Idea

Every piece of the system should be:

1. **Small enough to understand in isolation**
2. **Pure enough to test without the world**
3. **Typed enough to compose safely**
4. **Swappable enough to replace without cascading changes**

This applies at every scale: individual functions, modules, features, integrations, and even the app itself.

## Principle 1: Pure Over Stateful

Functions take inputs and return outputs. No hidden side effects, no global mutation, no reading from distant state.

Pure functions:
- Can be tested without a database, filesystem, or network
- Have no hidden dependencies
- Produce the same output for the same input every time
- Can be composed freely with other pure functions

Where side effects are necessary (database writes, API calls, filesystem access), isolate them at the boundary layer. Domain logic stays pure; adapters handle the outside world.

### Generic Composition Pattern

Design generic functions that work with any compatible type, then create typed entry points that compose the generic function with specific types:

```
parse_document<T>(content) -> (T, body)   // generic core
parse_rule(content) -> (RuleFrontmatter, body)  // typed entry point
parse_task(content) -> (TaskFrontmatter, body)  // typed entry point
```

One generic core. Multiple typed entry points. Each type is independently defined but all compose through the same pipeline.

## Principle 2: Small Composable Units

Prefer 5 small functions over 1 large one. Each function has a single job.

Guidelines:
- Domain functions: 20–30 lines
- Service/command functions: 30–50 lines
- Utilities: 10–20 lines

The top-level orchestrating function composes the small units and reads like a table of contents:

```
scan():
  for each rule:
    paths = collect_glob_paths(rule.scope)
    for each path:
      findings = scan_file(path, rule)
      collect(findings)
```

Reading `scan` tells you *what* happens without drowning in *how*.

## Principle 3: Type Boundaries and Interfaces

Define what something *does*, not what it *is*.

- Use traits (Rust), interfaces (TypeScript/Go), or protocols (Python) to define contracts
- Consumers depend on the interface, not the concrete implementation
- This enables swapping implementations without changing consumers

**Error composition:** Define a single error type for a layer. Implement conversions from dependency error types into your error type. This enables transparent error propagation with `?` (Rust), `await` with typed errors, or consistent exception hierarchies.

**Discriminated unions / tagged types:** When a value can be one of several variants, use a tagged type:
```
type Event =
  | { type: "stream_start" }
  | { type: "text_delta"; content: string }
  | { type: "tool_use"; tool_id: string; name: string }
```
Consumers switch on the tag with full type narrowing — no runtime ambiguity.

## Principle 4: Pluggable by Default

Every integration point is swappable. The system defines *what* it needs; providers implement *how*.

Design the interface first, then implement it:

```
interface MessageProvider {
  stream(prompt, onDelta): Promise<void>
}

interface FileStore {
  read(path): Promise<string>
  write(path, content): Promise<void>
}
```

Replacing the AI provider, storage backend, or search engine means implementing the interface — zero changes to domain logic.

## Principle 5: Pipeline Composition

Complex operations are chains of simple transforms, not monolithic functions.

```
results =
  items
  .filter(is_relevant)
  .map(transform)
  .collect()
```

Data flows through stages. No intermediate mutable variables. Each stage is independently testable.

## Principle 6: Feature as Module

A feature is a self-contained unit: domain logic + service handler + types + state + UI component. Features can be added or removed without touching other features.

Each feature owns:
- Its domain logic (pure functions, types)
- Its service handler (thin layer connecting domain to the outside world)
- Its type definitions (shared with any consumers)
- Its state management (subscribable reactive state)
- Its UI components (receive data via props, emit events)

Dependency direction is strictly one-way: UI component → state → service → domain.

## Principle 7: Functional Paradigm

Prefer `map`/`filter`/`collect` over loops with mutations. Prefer derived state over imperative updates. Prefer data transformations over mutations.

```
build_items(raw_data) =
  raw_data
  .map(item -> normalize(item))
  .filter(item -> item.is_valid)
  .collect()
```

No mutations. No loop variables. Input data in, transformed data out.

## Anti-Patterns

### God Components / God Functions

A single component or function that fetches data, manages state, AND renders/processes it.

**Fix:** Split into smaller units. Each has its own responsibility. A page or handler composes them.

### Thick Command Handlers

Business logic inside the outermost service handler (controller, command, route handler).

**Fix:** Move logic into domain functions. The handler becomes a thin delegation layer.

### Circular Dependencies

Module A imports module B; module B imports module A.

**Fix:** Extract shared state or logic into a third module that neither depends on the other. Coordinate via a parent layer.

### Tightly Coupled Features

Feature A calls directly into Feature B's internals.

**Fix:** Features communicate through public interfaces, not internal functions. The handler layer is the composition point.

## Composability Checklist

Before writing or reviewing code, verify:

- [ ] **Pure functions:** Can each function be tested without a database, filesystem, or network?
- [ ] **Small units:** Is every function within the project's size guidelines?
- [ ] **Single responsibility:** Does each function do exactly one thing?
- [ ] **Typed boundaries:** Are interfaces defined by contracts (traits, interfaces), not concrete implementations?
- [ ] **Swappable integrations:** Could external dependencies (AI provider, database, search) be replaced without changing domain logic?
- [ ] **Pipeline flow:** Are complex operations expressed as chains of transforms, not nested conditionals?
- [ ] **Feature isolation:** Could this feature be removed without breaking other features?
- [ ] **Unidirectional dependencies:** Do modules flow in one direction (UI → state → service → domain)?

## Platform Principle

Composability is a principle that every project managed by OrqaStudio should follow. When OrqaStudio initializes a new project, the governance framework, agent definitions, and coding standards guide developers toward composable architecture by default. In a world of ever-changing demands, composable software is dramatically easier to pivot and extend. This skill encodes that belief into the development process.

## Related Skills

- **orqa-domain-services** — how domain services compose without framework deps (project-layer skill)
- **orqa-repository-pattern** — how persistence composes via consistent repo interfaces (project-layer skill)
- **orqa-store-orchestration** — how frontend stores coordinate without coupling (project-layer skill)
- **orqa-error-composition** — how errors compose across layers (project-layer skill)
- **orqa-ipc-patterns** — the feature structure and service contracts (project-layer skill)

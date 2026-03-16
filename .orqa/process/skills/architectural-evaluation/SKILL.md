---
id: SKILL-001
title: Architectural Evaluation
description: |
  Architectural compliance evaluation methodology: boundary verification,
  domain model integrity checks, data flow mapping, schema evolution review,
  and structured compliance report format.
  Use when: Evaluating architectural compliance of plans or implementations,
  reviewing boundary integrity, mapping data flows, or assessing schema changes.
status: active
created: 2026-03-01
updated: 2026-03-10
layer: core
category: methodology
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Teaches boundary verification and compliance checklists that make architectural decisions visible and enforceable
---


Architectural compliance evaluation methodology. This skill teaches the *evaluation process and checklist structure* — the specific architectural decisions, layer names, and technology constraints come from the project's architecture skills and documentation.

## Evaluation Domains

### 1. Boundary Correctness

- Every frontend capability maps to a well-defined backend interface
- Handlers/commands are thin wrappers delegating to domain logic
- Types at boundaries are explicit and serializable
- No layer-skipping (e.g., UI accessing database directly)

### 2. Domain Model Integrity

- Each domain concept has its own module
- Domain models are typed structures — no stringly-typed data
- Domain services encapsulate business rules
- Repositories handle persistence — domain logic doesn't touch storage directly
- Cross-domain dependencies flow in one direction (no circular modules)

### 3. Data Flow Mapping

For any feature, map the complete data flow:

```
User Action
  → UI Component (event handler)
    → State Container (API call)
      → API Boundary (serialization)
        → Command Handler (thin wrapper)
          → Domain Service (business logic)
            → Repository / External Service
              → Response flows back up
```

Verify:
- Data transforms only happen at appropriate layers
- No layer skips
- Error handling exists at every boundary
- Types are consistent across boundaries

### 4. Schema Evolution

- Storage schema changes go through versioned migrations
- Referential integrity constraints are enforced
- Indexes exist for frequently queried columns
- Backwards compatibility considered for data already in storage

### 5. External Service Integration

- External API calls originate from backend, never from frontend
- API keys managed in backend, never exposed to frontend
- Structured communication protocols between processes (not ad-hoc shell pipes)

## Compliance Checklist

### Boundary Analysis

- [ ] Every frontend capability maps to backend handler(s)
- [ ] Handlers are thin wrappers delegating to domain services
- [ ] Types at boundaries are explicit and serializable
- [ ] Error handling converts domain errors to serializable format at the boundary
- [ ] No direct storage or external service access from UI

### Domain Model Assessment

- [ ] Each domain concept has its own module
- [ ] Domain models are typed structures
- [ ] Business rules live in domain services
- [ ] Repositories own persistence logic
- [ ] No circular module dependencies

### Streaming / Event Assessment (if applicable)

- [ ] Events flow through the defined pipeline
- [ ] Parsing/processing happens in the correct layer
- [ ] Event types are exhaustively handled

## Compliance Report Format

```markdown
## Architectural Compliance Report: [Feature/Module]

### Summary
[1-2 sentence architectural assessment]

### Boundary Analysis
- Interfaces: [list of handlers/commands involved]
- Data Flow: [layers traversed]
- Violations: [none / list]

### Domain Model Assessment
- Module Structure: COMPLIANT / NEEDS WORK
- Separation of Concerns: COMPLIANT / NEEDS WORK
- Dependency Direction: COMPLIANT / NEEDS WORK

### Recommendations
1. [Priority] Description of architectural improvement

### Verdict: COMPLIANT / NEEDS REMEDIATION / REVIEW REQUIRED
```

## Critical Rules

- NEVER approve domain logic in UI components — it belongs in domain services
- NEVER approve direct storage access from the wrong layer
- NEVER approve external API calls from the frontend
- NEVER approve features that violate the project's architectural decisions
- Architectural violations are blocking — they must be resolved before merge
- When recommending changes, provide the specific target pattern from the architecture docs

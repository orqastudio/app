---
role: artifacts
label: "Research"
description: "Investigation artifacts that validate ideas before implementation."
icon: "flask-conical"
sort: 5
---

# Research

Research documents capture investigations, design explorations, and architecture spikes. They produce findings that inform decisions and epics. When research resolves a question, it is promoted to an Architecture Decision (`AD-NNN`).

## Lifecycle

```
draft → complete → surpassed
```

- **Draft**: Investigation in progress
- **Complete**: Findings documented, recommendation made
- **Surpassed**: Newer research supersedes this document (set `surpassed-by` field)

Research documents are historical records. When surpassed, they are preserved — not deleted.

## Structure

Each research document includes:

- **Question**: What is being investigated
- **Findings**: What was discovered
- **Options evaluated**: Alternatives considered with trade-offs
- **Recommendation**: What to do and why

## Research-to-Decision Pipeline

```
Research → Findings → Decision → AD-NNN → Implementation
```

1. Research captures options, trade-offs, and evidence
2. A recommendation is made when enough evidence exists
3. User approves the recommendation
4. An Architecture Decision is created in **Governance > Decisions**

## Related

- Research informs epics via the `research-refs` field in the **Epics** section
- Research produces decisions in the **Decisions** section under Governance
- Ideas reference research via `research-needed` items in the **Ideas** section

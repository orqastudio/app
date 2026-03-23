---
id: KNOW-1ab0e715
title: "Thinking Mode: Documentation"
thinking-mode: documentation
description: "The user wants docs written, updated, or organised — capturing knowledge for humans, not building features."
status: active
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: DOC-632df5ad
    type: synchronised-with
---

# Thinking Mode: Documentation

The user wants docs written, updated, or organised. This is about capturing knowledge for humans browsing the project, not building features. The agent writes documentation artifacts, not code.

## Example Signals

"document how X works", "update the README", "write a guide for Y", "the docs are out of date", "add documentation for the new command", "write the platform doc for this feature", "the doc page is missing"

## What the Agent Needs

- Project documentation standards and the documentation-before-code rule
- Current state of the feature being documented — read the code and existing artifacts first
- Doc schema: `type: doc`, correct `category`, relationships to relevant artifacts
- Documentation before code rule: docs define intent, code implements it

## Distinguishing from Similar Modes

- Not **Implementation**: agent writes markdown artifacts, not code
- Not **Learning Loop**: structured, human-readable docs — not observations or lessons
- Not **Planning**: describes existing state or agreed design, not future work scope

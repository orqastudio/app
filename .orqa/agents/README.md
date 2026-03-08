# Agents

Agents are specialized AI personas with domain-specific knowledge, skills, and behavioral rules. Each agent definition specifies what the agent can do, what documentation it must read before working, and what skills it loads.

## How Agents Work

The orchestrator delegates implementation tasks to agents based on their specialization. Each agent works in isolation with its own context, reads its required documentation, loads its declared skills, and reports results back to the orchestrator.

## Agent Types

- **Implementation agents**: Write code in specific domains (backend, frontend, data, design)
- **Review agents**: Verify quality, compliance, and correctness (code reviewer, QA tester, UX reviewer)
- **Support agents**: Handle cross-cutting concerns (documentation, security, architecture, debugging)

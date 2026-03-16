---
id: DOC-080
title: Getting Started
description: "Introduction to OrqaStudio, what it does, and how to set up your first project."
created: 2026-03-14
updated: 2026-03-14
sort: 1
---

## What Is OrqaStudio?

OrqaStudio is an AI-assisted clarity engine that helps you turn messy situations into structured understanding and evolving plans. It combines a desktop application with an opinionated governance framework built on markdown artifacts, relationship graphs, and AI-powered reasoning.

OrqaStudio is not a code editor or an IDE. It is a structured thinking tool that happens to integrate with AI providers and development workflows. You can use it for software projects, research planning, strategy development, or any domain where structured reasoning improves outcomes.

## Core Concepts

**Artifacts** are markdown files with YAML frontmatter that represent structured knowledge: tasks, epics, decisions, rules, research, ideas, and more. They live in the `.orqa/` directory and form a relationship graph that OrqaStudio scans, validates, and visualises.

**The artifact graph** connects artifacts through typed relationships. An epic delivers a milestone. A task depends on another task. A rule enforces a decision. These edges make the structure of your project navigable and auditable.

**Pillars** are the guiding principles for your project. Every feature and artifact should trace back to at least one pillar. OrqaStudio ships with three default pillars but they can be customised per project.

**The knowledge pipeline** tracks how knowledge flows through your project: from observations and research through understanding, principles, practices, enforcement, and verification. The dashboard visualises pipeline health and highlights bottlenecks.

## Installation

OrqaStudio is a Tauri v2 desktop application. To build from source:

```bash
# Prerequisites: Rust toolchain, Node.js 18+, npm
git clone https://github.com/orqastudio/orqa-studio.git
cd orqa-studio
make install   # Install all dependencies
make dev       # Start the development environment
```

The app opens automatically when the build completes. On first launch, OrqaStudio scans the `.orqa/` directory and builds the artifact graph.

## Project Structure

Every OrqaStudio project has an `.orqa/` directory at its root:

```
.orqa/
  project.json          # Project configuration
  process/              # Governance artifacts
    pillars/            # Guiding principles
    rules/              # Enforcement rules
    skills/             # Reusable knowledge
    agents/             # Agent role definitions
    decisions/          # Architecture decisions
    lessons/            # Implementation lessons
  delivery/             # Planning artifacts
    milestones/         # Strategic goals
    epics/              # Feature scopes
    tasks/              # Individual work items
    ideas/              # Future possibilities
    research/           # Investigation documents
  documentation/        # Project documentation
    guide/              # User-facing guides (you are here)
    product/            # Product vision and governance
    architecture/       # Technical architecture
    development/        # Development standards
    process/            # Process documentation
    ui/                 # UI specifications
```

## Next Steps

- Read [Artifact Framework](../product/artifact-framework.md) to understand what each artifact type is for
- Read [Workflow](workflow.md) to understand how the governance workflow operates
- Explore the dashboard to see your project's artifact graph and pipeline health

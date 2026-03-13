---
id: IDEA-023
title: Community Inbox
description: |
  Manage incoming communication from multiple channels in a structured way, applying OrqaStudio's structured thinking to community triage.
status: captured
created: "2026-03-07"
updated: "2026-03-07"
pillars:
  - PILLAR-001
  - PILLAR-002
research-needed:
  - Channel integration architecture (GitHub, email, blog, social)
  - Artifact schema for communication signals
  - Automated acknowledgement patterns (GitHub Actions, email autoresponders)
  - Response queue UX and scheduling model
  - Signal classification taxonomy
  - Privacy and data handling for external messages
promoted-to: null
---
## Problem

When a project becomes public, communication can arrive from many sources simultaneously.

Typical channels include:

- GitHub issues
- pull requests
- GitHub discussions
- email
- blog comments
- Reddit / Hacker News threads
- social media messages

Without structure, this creates several problems:

- constant interruption
- anxiety about missed messages
- inconsistent responses
- difficulty tracking important conversations

Maintainers often feel pressure to be **always online**, which is unsustainable.

---

## Concept

Community Inbox treats incoming communication as **structured signals** rather than interruptions.

The flow becomes:

```
incoming message
  → signal capture
  → structured artifact
  → response queue
  → intentional response
  → retrospective insight
```

The system separates **acknowledgement** from **response**.

Messages can be acknowledged quickly while deeper responses happen later.

---

## Core Principles

### 1. Capture First

All incoming messages are captured and structured before any action is taken.

This ensures nothing is lost while avoiding the pressure to respond immediately.

### 2. Acknowledge Automatically

Simple automated acknowledgements reassure contributors that their message has been seen.

Example acknowledgement:

> Thanks for your message.
> This input has been captured and will be reviewed.

### 3. Queue Responses

Messages are added to a response queue.

Maintainers can review and respond in scheduled sessions rather than reacting continuously.

### 4. Classify Signals

Each message becomes an artifact that can be categorized.

Example categories:

- bug report
- feature request
- question
- collaboration opportunity
- community discussion
- content idea

### 5. Enable Reflection

Over time the system can surface patterns such as:

- repeated questions
- common feature requests
- areas of confusion
- emerging community interests

This supports learning and product improvement.

---

## Example Workflow

Example flow for a GitHub issue:

```
GitHub Issue Created
  → Automated acknowledgement
  → Artifact created in Community Inbox
  → Classified as "feature request"
  → Added to response queue
  → Maintainer review session
  → Response posted
```

The same model applies to other channels such as email or blog comments.

---

## Potential Integrations

Community Inbox could integrate with:

- GitHub Issues
- GitHub Discussions
- email inbox
- blog comment systems
- social media mentions

Each integration simply converts incoming messages into artifacts.

---

## Early Dogfooding Use Case

The initial use case for this capability is managing communication for the OrqaStudio project itself.

It would help manage:

- community feedback
- contributor discussions
- collaboration requests
- consulting enquiries

This provides a real-world test of the system.

---

## Long-Term Vision

If successful, Community Inbox could become a general capability within OrqaStudio.

Possible applications include:

- open-source project management
- community moderation
- support inboxes
- organisational communication triage

In this model, OrqaStudio helps orchestrate not only structured thinking but also **structured conversation**.

---

## Relationship to OrqaStudio Philosophy

Community Inbox embodies several core principles of the platform:

- artifact-driven knowledge
- structured workflows
- intentional response cycles
- learning through retrospection

It demonstrates that OrqaStudio can support not only project development but also the **human systems that surround it**.

---

## Status

Conceptual roadmap item.

Potential future plugin or capability once the core artifact and orchestration systems are stable.

## Motivation

See description in frontmatter.

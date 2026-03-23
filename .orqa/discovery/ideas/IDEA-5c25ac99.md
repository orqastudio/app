---
id: IDEA-5c25ac99
type: idea
title: "Git hosting as cloud sync — multi-user without servers"
description: "Use GitHub/GitLab/Bitbucket as the sync layer for multi-user OrqaStudio projects. Git already tracks changes and history — a hosting provider adds authentication, access control, and cloud storage without running custom infrastructure."
status: captured
created: 2026-03-19
updated: 2026-03-19
research-needed:
  - How do .orqa/ artifacts handle merge conflicts when two users edit the same artifact?
  - Can we auto-merge non-conflicting frontmatter changes (different relationship entries)?
  - What's the UX for pull/push/sync in the app? Should it be automatic or explicit?
  - How does branch-based work (feature branches) interact with the artifact graph?
  - Can we detect and surface "your colleague changed this artifact since you last synced"?
  - What happens when two users create artifacts with the same ID prefix simultaneously?
  - Should .orqa/ be in the same repo as the code, or a separate repo for non-code projects?
  - How do we handle orqa-plugin.json merges when two users install different plugins?
  - What's the minimum viable sync — just git push/pull, or do we need real-time?
relationships:
  - target: PILLAR-94b281db
    type: grounded
    rationale: Multi-user sync preserves intent across contributors — no one's thinking is lost between sessions
  - target: PERSONA-cda6edd6
    type: benefits
---

# Git Hosting as Cloud Sync

OrqaStudio already uses git for file storage and change tracking. The insight is that a git hosting provider (GitHub, GitLab, Bitbucket) already provides everything needed for multi-user cloud sync:

- **Authentication** — who has access
- **Authorization** — who can read/write
- **Cloud storage** — the repo IS the cloud
- **Change history** — git log IS the audit trail
- **Conflict resolution** — git merge IS the sync mechanism
- **Notifications** — webhooks, email, activity feeds

No custom servers. No database sync. No subscription infrastructure. The user's existing GitHub account IS their OrqaStudio cloud account.

## What This Enables

- **Team projects** — multiple users contributing to the same .orqa/ artifact graph
- **Cross-device sync** — same project on laptop and desktop via git
- **Backup** — the repo IS the backup
- **Access control** — GitHub teams/permissions manage who sees what
- **Organisation mode** — an org's repos map naturally to OrqaStudio projects

## Key Design Questions

The hard part isn't the sync mechanism (git handles that). The hard part is:

1. **Merge UX** — when two users edit the same artifact, how does the app present the conflict?
2. **Real-time vs explicit** — auto-sync on save, or manual push/pull?
3. **ID collision** — two users creating TASK-afe17917 simultaneously
4. **Branch model** — do artifact changes happen on branches with PRs, or directly on main?
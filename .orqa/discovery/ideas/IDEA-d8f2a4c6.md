---
id: IDEA-d8f2a4c6
type: idea
title: "Git hosting platform evolution — theming, public engagement, contribution flow, auth"
status: captured
description: Future enhancements for the self-hosted git platform covering branding, community engagement for non-technical users, contribution workflow design, and authentication strategy.
created: 2026-03-23
relationships:
  - target: EPIC-f2b9e7d3
    type: follows
    rationale: Built on the git infrastructure established in this epic
  - target: IDEA-7c3d9f2e
    type: related
    rationale: Forgejo cloud hosting vision — this idea captures near-term evolution
---

## 1. Custom Theming

Brand the self-hosted instance as OrqaStudio rather than generic Forgejo. Custom CSS, logo, colour scheme, landing page. The platform should feel like part of OrqaStudio, not a third-party tool bolted on.

Research needed:
- What theming options does the platform expose? (CSS overrides, template injection, full theme system)
- Can the landing page be customised to explain OrqaStudio and direct users to the right place?
- How do themes survive upgrades?

## 2. Public Issues / Discussion Board Without Code Visibility

Non-technical users (designers, product people, governance practitioners) need a place to report bugs, request features, and discuss — without needing to navigate a code repository. They shouldn't need to understand git, repos, or PRs.

Options to explore:
- A public-facing project board / issue tracker that syncs to the git platform's issues
- A separate lightweight frontend (could be an OrqaStudio plugin view) that talks to the platform API
- Platform-native project boards with restricted repo access (issues visible, code hidden)
- Integration with existing discussion tools (forum, Discord) with bidirectional issue sync

Key constraint: issues created by non-technical users must land in the same tracking system developers use — no separate silos.

## 3. GitHub PR Syncing vs Contribution Flow

The sync bridge built in EPIC-f2b9e7d3 handles basic PR mirroring, but the contribution flow needs a clear decision:

**Option A: Accept PRs on both platforms (sync bridge)**
- GitHub PRs sync to local server, merged there
- Complex, potential for sync conflicts
- Lowest barrier for GitHub-native contributors

**Option B: GitHub as read-only mirror, PRs on the hosted platform only**
- Simpler — one source of truth for PRs
- Contributors need to register on the hosted platform
- GitHub README points contributors to the right place
- GitHub auth (item 4) lowers the registration barrier

**Option C: GitHub PRs only for external, hosted platform for internal**
- External contributors use GitHub (familiar)
- Internal/core contributors use the hosted platform
- Sync bridge only needs one direction (GitHub → hosted)

Decision depends on contributor volume and the auth story.

## 4. Authentication Strategy

### Near-term: GitHub as an OAuth provider
- Contributors log in to the hosted platform with their GitHub account
- Zero separate registration needed
- Lowers the barrier if we direct contributors to the hosted platform (Option B above)

### Future cloud hosting: Enterprise auth
- SAML/SSO for organisations (note: not natively supported yet — open feature request)
- LDAP for enterprise directory integration
- OpenID Connect for generic SSO providers
- Per-organisation auth configuration — each org chooses their provider

### Multi-tenant considerations
- When the hosted platform becomes a cloud service, each organisation needs isolated auth
- The management layer (IDEA-7c3d9f2e) would handle org-level auth configuration
- Self-hosted instances vs managed cloud have different auth requirements

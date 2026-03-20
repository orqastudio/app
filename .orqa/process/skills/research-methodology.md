---
id: SKILL-f7476f0a
title: Research Methodology
description: |
  Research best practices for agents investigating external sources: source
  verification, credibility assessment, cross-referencing, and structured
  documentation of findings with confidence levels.
status: active
category: methodology
user-invocable: true
relationships:
  - target: AGENT-caff7bc1
    type: employed-by
  - target: AGENT-fb0ce261
    type: employed-by
  - target: DOC-a1b2c3d4
    type: synchronised-with

---

# Research Methodology

## Purpose

When agents use `WebSearch` and `WebFetch` to investigate external sources, they
must verify what they find to an acceptable level of certainty before incorporating
it into artifacts. This skill defines what "acceptable certainty" means and how to
achieve it.

## Source Verification Protocol

### Before Citing Any External Source

1. **Identify the source type** and apply the appropriate verification level
2. **Cross-reference** with at least one independent source for non-trivial claims
3. **Check currency** — is this information current or outdated?
4. **Assess relevance** — does this apply to our specific context (versions, platforms)?
5. **Document confidence** — state how certain you are and why

### Source Credibility Tiers

| Tier | Source Type | Verification Required | Confidence |
|------|-----------|----------------------|------------|
| **T1 — Authoritative** | Official documentation, RFCs, language/framework specs, vendor docs | Verify version matches project stack | High |
| **T2 — Reliable** | Well-maintained GitHub repos (>100 stars), established blogs (official team blogs), conference talks by core contributors | Cross-reference with T1 source | Medium-High |
| **T3 — Community** | Stack Overflow (accepted + upvoted), GitHub issues/discussions, dev.to/Medium articles, tutorials | Cross-reference with T1 or T2, check date | Medium |
| **T4 — Unverified** | Personal blogs, forum posts, AI-generated content, single-source claims | Must verify with T1/T2 before citing | Low |

### Cross-Referencing Rules

- **Single-source claims**: If only one source says something, note it explicitly:
  "Single source — not independently verified"
- **Contradicting sources**: Document both positions and the contradiction. Do not
  silently pick one.
- **Version-specific information**: Always check which version the source applies to.
  An answer for version N may be wrong for version N+1.
- **Date-sensitive information**: Library APIs change. Check the publication date.
  Anything older than 12 months needs verification against current docs.

## Structured Research Documentation

### Research Document Format

When creating research artifacts:

```yaml
sources:
  - url: "https://example.com/official-docs/v2/api"
    description: "Official API documentation"
    tier: T1
    accessed: "2026-03-12"
  - url: "https://github.com/example/project/issues/123"
    description: "Known issue discussion"
    tier: T2
    accessed: "2026-03-12"
```

### Confidence Levels in Findings

Every finding or recommendation should state its confidence:

| Level | Meaning | Basis |
|-------|---------|-------|
| **Confirmed** | Verified against T1 source + tested or independently corroborated | Multiple sources agree, version-matched |
| **Likely** | Verified against T1/T2 source but not independently tested | Single authoritative source, no contradictions |
| **Uncertain** | Based on T3/T4 sources or extrapolated from related information | Community consensus but no official confirmation |
| **Speculative** | Informed guess based on system understanding | No external source — reasoning from first principles |

### What NOT To Do

- **Never present T4 claims as fact** without verification
- **Never omit the source** when making a factual claim about an external system
- **Never assume version compatibility** — check explicitly
- **Never cite a single Stack Overflow answer as authoritative** — it's a starting
  point, not a conclusion
- **Never ignore contradicting evidence** — document it, even if inconvenient
- **Never claim "best practice" without attribution** — whose best practice? Says who?

## Web Search Patterns

### Effective Queries

| Intent | Query Pattern | Example |
|--------|-------------|---------|
| Official docs | `site:[docs-domain] [library] [function]` | `site:docs.python.org asyncio gather` |
| GitHub issues | `site:github.com [repo] [keyword]` | `site:github.com org/project connection-pool` |
| Version-specific | `[library] v[version] [topic]` | `react 19 server components` |
| Comparison | `[A] vs [B] [context]` | `SQLite vs PostgreSQL embedded use` |

### When to Search

- **Before making technology choices** — compare options with evidence
- **When encountering unfamiliar errors** — check if it's a known issue
- **When implementing external integrations** — verify current API contracts
- **When a claim feels uncertain** — verify rather than guess

### When NOT to Search

- **For internal project conventions** — use the artifact graph, not the web
- **For code that exists in the codebase** — use `search_regex` / `search_semantic`
- **For trivial language syntax** — if you know the language, don't search for basics
- **When a T1 source is already loaded** — don't re-search what you've already read

## Integration with Artifact Graph

Research findings connect to the graph through typed relationships:

1. **`sources` field** on research documents — structured external references
2. **Relationship links** to delivery artifacts — which research informed which work
3. **Lessons** — when external information was wrong or misleading, capture it

The graph makes research discoverable: when a future task touches the same area,
the relevant research (with its verified sources) is found via graph traversal.

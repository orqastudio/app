---
id: KNOW-12ed4953
type: knowledge
name: Licensing Decisions
status: active
category: domain
user-invocable: true
relationships:
  - target: DOC-4ed362fb
    type: synchronised-with
  - target: DOC-4b4fbc0f
    type: synchronised-with

---

# Licensing Decisions

When licensing questions arise — choosing a license for a new component, evaluating whether a use case is permitted, or advising contributors — use this skill.

## The Governing Decision

AD-fc646168 defines the ethical licensing model. All licensing answers derive from it.

## Quick Reference

### What license should a new component use?

| Component type | License | Why |
|---------------|---------|-----|
| App, library, plugin, template, tool | BSL-1.1 with Ethical Use Addendum | Core product and infrastructure — protected for 4 years, then Apache 2.0 (+ Addendum) |
| Connector | Apache-2.0 | Must interop with third-party tools that may require permissive licensing |
| Registry | MIT | Community infrastructure should be fully open |

The Ethical Use Addendum is uniform across all BSL repos — there is no BSL variant without it. It includes explicit gender identity protection and remains in force permanently (even after the Apache 2.0 change date).

### Is this use permitted?

| User type | Production use? | Needs license? |
|-----------|----------------|----------------|
| Individual (personal use) | Yes | No — Additional Use Grant |
| Aligned charity/non-profit | Yes | No — Additional Use Grant |
| Educational institution | Yes | No — Additional Use Grant |
| Open-source project | Yes | No — Additional Use Grant |
| Corporation (evaluation) | Yes | No — non-production use is unrestricted |
| Corporation (production) | After change date | Yes — commercial license or wait 4 years |
| SaaS hosting | No | Not available before change date |

### Is this charity/non-profit "aligned"?

The grant applies if the organisation's mission serves:
- Equal access to healthcare, education, housing, or emergency services
- Democratisation of technology and AI
- Environmental sustainability
- Social mobility and equal opportunity
- Community empowerment and civic participation

The grant does NOT apply if the organisation actively:
- Lobbies against universal healthcare or education
- Works to restrict workers' rights or democratic participation
- Concentrates AI capabilities in ways that harm equal access

When in doubt, check whether the organisation's mission aligns with the principle: **AI should help build a society where everyone is looked after and treated equally.**

## Contributor Questions

- **Do contributors sign a CLA?** No. DCO (Developer Certificate of Origin) only.
- **Who owns contributions?** Contributors retain copyright. The project license applies to the combined work.
- **Can anyone contribute?** Yes, as long as they share the project's philosophy.

## Edge Cases

### A company wants to use OrqaStudio internally

If it's a for-profit company using it for commercial work → they need a commercial license or must wait for the 4-year change date. If they're evaluating/testing → unrestricted.

### A non-profit has corporate sponsors

The non-profit's own use is granted if their mission aligns. The corporate sponsor's use is not covered by the non-profit's grant — the sponsor must have their own license.

### Someone forks and removes the license

BSL-1.1 requires the license to be preserved in derivative works. Removing it is a license violation.

### A community plugin author wants a different license

Community plugins in the registry can use any license the author chooses. The registry itself is MIT. The author's plugin is their work.

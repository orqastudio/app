---
id: IDEA-c1364d60
title: Automated community registry maintenance
description: "How can we automate the ongoing maintenance of the community plugin registry — compatibility CI, staleness detection, and platform version testing? Version pinning and SHA verification are already in place as policy."
status: captured
created: 2026-03-18
updated: 2026-03-18
research-needed:
  - Can we run orqa validate against registered plugins automatically on a schedule (CI)?
  - Can we detect when a plugin's published release changes artifact types or relationship keys vs what was verified?
  - Can we auto-flag plugins that haven't published a release in N months?
  - Can we diff a plugin's orqa-plugin.json between versions to detect breaking changes before they reach users?
  - Can we run the ethical use check programmatically (scan README/LICENSE for policy compliance)?
  - How do we handle plugins that become incompatible after a core platform update?
  - Can GitHub Actions on the registry repo automate any of this?
  - The installer already checks SHA on download — can we add a CI step that re-verifies pinned hashes periodically?
relationships:
  - target: PILLAR-cdf756ff
    type: grounded
    rationale: Automated registry maintenance is the learning loop applied to the plugin ecosystem
  - target: PERSONA-cda6edd6
    type: benefits
---

# Automated Community Registry Maintenance

How can we reduce the manual overhead of maintaining the community plugin registry while ensuring quality, compatibility, and ethical compliance?

The submission terms define what's expected of plugin authors. The question is: how much of the enforcement can be automated vs requiring human review?

Areas to explore:
Version pinning (release tag + SHA256) and SHA verification on install are already policy — documented in the registry CONTRIBUTING.md. This idea is about automating the ongoing maintenance:

- **Compatibility CI** — scheduled runs of `orqa validate` against registered plugins
- **Breaking change detection** — diff `orqa-plugin.json` between releases
- **Staleness monitoring** — flag plugins with no releases in 6+ months
- **Platform compatibility** — auto-test plugins against new core versions before release
- **Ethical compliance scanning** — programmatic checks on LICENSE and README content

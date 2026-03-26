---
id: IDEA-6c8ed8c7
type: discovery-idea
title: "Plugin installation ecosystem — dependency management, setup automation"
description: "Define what plugins need to ensure they install/download all dependencies and set themselves up correctly on install. Includes npm/cargo deps, binary downloads (LSP servers, ONNX models), config file generation, and post-install verification."
status: captured
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---

# IDEA-143: Plugin Installation Ecosystem

What's needed for reliable plugin self-setup:

1. **Dependency declaration** — npm packages, cargo crates, system binaries
2. **Install hooks** — post-install scripts that run `npm install`, download models, generate configs
3. **Verification** — post-install check that all deps are available and correctly configured
4. **Binary management** — LSP servers, ONNX models, other binaries that aren't npm packages
5. **Rollback** — if installation fails, clean up partial state
6. **Update path** — how to handle breaking changes between plugin versions

Related: IDEA-142 (lifecycle events), Claude Code's `${CLAUDE_PLUGIN_DATA}` for persistent data.
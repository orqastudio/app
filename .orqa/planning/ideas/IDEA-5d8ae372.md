---
id: IDEA-5d8ae372
type: planning-idea
title: "Evaluate zeroclawed for plugin safety / sandboxing"
description: "Research bglusman/zeroclawed as a candidate model for OrqaStudio plugin safety. The goal is to harden plugin install and execution so a malicious or buggy plugin cannot escape its intended capability set."
status: captured
priority: P2
created: 2026-04-11
updated: 2026-04-11
horizon: next
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "Plugin-composed everything is only safe if plugins have enforceable boundaries. Without sandboxing, the plugin model is a trust-the-author system — a single bad plugin could compromise the project."
---

## Source

User research lead, captured 2026-04-11 in conversation: "look into https://github.com/bglusman/zeroclawed for plugin safety".

## What

[bglusman/zeroclawed](https://github.com/bglusman/zeroclawed) is a GitHub project the user flagged as a candidate approach for plugin safety / sandboxing. The lead is: evaluate its model and decide whether OrqaStudio should adopt, adapt, or draw inspiration from it for the plugin execution sandbox.

## Why it's relevant

OrqaStudio is built on P1: **Plugin-Composed Everything**. Every governance pattern, artifact type, relationship, workflow stage, enforcement generator, and connector lives in a plugin. This is the central architectural decision — and it's also the biggest open safety question:

- Plugins can declare hooks that run on every tool call (`PreToolUse`, `PostToolUse`).
- Generator plugins execute as daemon subprocesses with the working directory and env of the daemon.
- CLI-tool plugins register commands that can shell out.
- Content-sync plugins can write to `.orqa/` and `.claude/`.

Today, trust is binary: if you install a plugin, it has the same capabilities as any other part of the project. There is no capability declaration, no enforced scope, no sandbox. A malicious plugin on the registry could exfiltrate `.state/orqa.db`, edit arbitrary `.orqa/` artifacts, or inject commands into the daemon. The plugin-dev workflow captured this risk in passing but never closed the loop.

This is a P2 not P3 because the plugin marketplace is a stated product direction, and "users can install third-party plugins" is a feature that cannot ship safely without this problem solved.

## What to investigate

- **Model**: what is zeroclawed's threat model and capability model? Does it sandbox at the filesystem, process, syscall, or WASM layer?
- **Maturity**: is it production-ready, an experiment, or a thought piece? GitHub README and commit activity will tell us.
- **Integration surface**: is it a library (Rust crate? npm module?), a standalone sandbox binary, or just a design document?
- **Performance cost**: does enforcing the sandbox add latency to every plugin call, and if so how much?
- **Cross-platform**: Windows, macOS, Linux? A plugin safety model that only works on Linux is half a solution for a dev tool that ships as a Tauri app.
- **Licensing**: open source, permissive? Compatible with BSL-1.1?
- **Adjacent prior art**: compare against Deno permissions, WASI capabilities, Landlock, seccomp, sandbox-exec, `firejail`, `bubblewrap`, `nsjail`, capability-based security in Capsicum, and the WebAssembly Component Model.

## Decision criteria

- Must express capabilities declaratively in the plugin manifest (`purpose: "connector"` → filesystem scope `.orqa/` only; `purpose: "sidecar"` → subprocess + network; etc.).
- Must fail closed — a plugin that doesn't declare a capability doesn't get it.
- Must be enforceable at both install time (manifest validation) and runtime (syscall / filesystem boundary).
- Must be auditable — the user should be able to see what every installed plugin is allowed to do.
- Must work on Windows, macOS, and Linux to match the dev environment's platform coverage.

## Relationship to existing work

- Tightens the existing plugin install constraints pipeline (`engine/plugin/src/installer.rs`, `PluginInstallConstraints`) by turning declarative metadata into enforced limits.
- Complements the sidecar architecture (`project_sidecar_architecture.md` user memory) — sidecars especially need a sandbox because they run arbitrary LLM code.
- Feeds the plugin-dev plugin (`plugins/knowledge/plugin-dev/`) which already has opinions about how plugins should be structured.
- Informs the "three-way diff model" for plugin updates (`project_plugin_diff_model.md` user memory) — a capability change between versions is a security-relevant diff that needs explicit user acceptance.

## Not in scope

- Sandboxing the OrqaStudio daemon itself or the Tauri app. The daemon is trusted code; the scope is plugins loaded at runtime.
- Building an OS-level process jail from scratch. The answer should leverage existing primitives (OS, WASM, or library) — we are not in the sandbox-research business.
- Ad-hoc per-plugin exception lists. The model must be declarative and generic.

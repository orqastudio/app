![License](https://img.shields.io/badge/license-Apache--2.0-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# Claude Code Connector

Dual-manifest connector plugin bridging OrqaStudio's governance system with Claude Code's plugin framework. Serves as both an OrqaStudio plugin (`orqa-plugin.json`) and a Claude Code plugin (`.claude-plugin/plugin.json`).

## What It Does

- **Agent mapping** — maps OrqaStudio's 9 universal agents to Claude Code subagent definitions
- **Rule enforcement** — evaluates governance rules via PreToolUse hooks (block/warn/inject)
- **Skill injection** — classifies user intent and injects relevant domain skills
- **Artifact bridge** — syncs `.claude/` symlinks to `.orqa/` artifact graph
- **Validation hooks** — validates artifact writes, preserves context on compaction
- **Slash commands** — `/orqa`, `/orqa-validate`, `/orqa-create`

## Architecture

```
connectors/claude-code/
├── .claude-plugin/plugin.json  ← Claude Code sees this
├── orqa-plugin.json            ← OrqaStudio sees this
├── hooks/                      ← Claude Code hook scripts
├── skills/                     ← Claude Code-native skills
├── agents/                     ← Claude Code subagent definitions
├── commands/                   ← Claude Code slash commands
└── src/                        ← TypeScript library (bridge, rule engine, prompt injector)
```

## Installation

Installed alongside the Claude Integration plugin (`@orqastudio/plugin-claude`).

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

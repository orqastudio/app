![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# Core Framework Plugin

OrqaStudio plugin providing the agent execution model — universal roles, delegation, session management, knowledge loading, and thinking mode classification. Every project gets this plugin automatically.

## What It Does

- **Agent roles** — defines universal roles: Orchestrator, Researcher, Planner, Implementer, Reviewer, Writer, Designer
- **Thinking mode classification** — ONNX-based prompt classification that injects the right context before each agent response
- **Knowledge loading** — session-aware injection of domain knowledge into agent context
- **Enforcement mechanisms** — behavioural rules, JSON Schema validation, lifecycle hooks, and semantic inference
- **Session management** — workflow tracking, session state, and process gates

## Installation

This plugin is installed automatically with every OrqaStudio project. It cannot be uninstalled.

## Development

```bash
npm install
npm run build
```

## License

BSL-1.1 — see [LICENSE](LICENSE) for details.

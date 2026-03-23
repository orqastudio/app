![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/.github/blob/main/assets/banners/banner-1680x240.png?raw=1)

# Git Hooks Plugin

OrqaStudio plugin providing pre-commit enforcement via git hooks — schema validation, filename-to-ID matching, and relationship integrity. Schema-driven: reads validation rules from installed plugin manifests.

## What It Does

- **Pre-commit hook** — validates governance artifacts against plugin schemas before every commit
- **Schema validation** — checks artifact frontmatter against the JSON Schemas defined by installed plugins
- **Filename integrity** — ensures artifact filenames match their declared IDs
- **Relationship integrity** — verifies relationship targets exist in the artifact graph

## Installation

```bash
orqa plugin install @orqastudio/plugin-githooks
orqa hooks-install
```

## Development

```bash
npm install
npm run build
```

## License

BSL-1.1 — see [LICENSE](LICENSE) for details.

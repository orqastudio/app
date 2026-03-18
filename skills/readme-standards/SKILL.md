---
id: SKILL-CLI-007
type: skill
name: README Standards
status: active
plugin: "@orqastudio/plugin-cli"
relationships:
  - target: DOC-CLI-004
    type: synchronised-with
---

# README Standards

Every OrqaStudio repository must have a README.md with canonical structure. The `orqa repo readme` command audits compliance.

## Required Sections

| Section | Required | Pattern |
|---------|----------|---------|
| Title | Yes | `# PackageName` matching the display name |
| Description | Yes | Opening paragraph (at least 20 characters) |
| Installation | No* | `## Installation` (required for publishable packages) |
| Usage | No* | `## Usage` (required for packages with a public API) |
| License | Yes | `## License` with the license name |

*Installation and Usage are required for libraries and plugins but optional for the app and dev repo.

## Auditing

```bash
# Check all README files
orqa repo readme

# JSON output for CI
orqa repo readme --json
```

## Results

Each repo gets a status:
- **ok** — README exists with all required sections
- **missing** — no README.md found
- **incomplete** — README exists but missing required sections

## Template

When creating a new repo, use this structure:

```markdown
# Package Display Name

One-paragraph description of what this package does and why it exists.

## Installation

\`\`\`bash
npm install @orqastudio/package-name
\`\`\`

## Usage

Brief usage example showing the primary API.

## Development

How to set up for local development (if applicable).

## License

BSL-1.1 — see [LICENSE](LICENSE) for details.
```

## Naming Convention

The README title should match the `displayName` from `package.json` or `orqa-plugin.json`, not the npm package name. For example:
- Package: `@orqastudio/types` → Title: `# OrqaStudio Types`
- Plugin: `@orqastudio/plugin-software-project` → Title: `# Software Project`

---
id: DOC-CLI-004
title: README Standards
description: "Canonical README structure for all OrqaStudio repositories — required sections, naming conventions, and audit process."
category: reference
created: 2026-03-18
updated: 2026-03-18
relationships:
  - target: SKILL-CLI-007
    type: synchronised-with
---

# README Standards

Every OrqaStudio repository must have a `README.md` with consistent structure. This ensures discoverability, onboarding clarity, and professional presentation.

## Required Sections

| Section | Always Required | Description |
|---------|----------------|-------------|
| **Title** | Yes | `# Display Name` matching the package's displayName |
| **Description** | Yes | Opening paragraph explaining what and why (minimum 20 characters) |
| **Installation** | For published packages | How to install (`npm install`, `cargo add`, etc.) |
| **Usage** | For packages with APIs | Primary usage example |
| **License** | Yes | `## License` with the license name and link to LICENSE file |

## Naming Convention

The README title should match `displayName` from the manifest, not the package name:
- `@orqastudio/types` → `# OrqaStudio Types`
- `@orqastudio/plugin-software-project` → `# Software Project`

## Auditing

```bash
orqa repo readme        # Human-readable audit
orqa repo readme --json # Machine-readable for CI
```

## Template

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

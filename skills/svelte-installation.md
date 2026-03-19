---
id: SKILL-SVE-90dd73ab
type: skill
name: Svelte Plugin Installation
status: active
plugin: "@orqastudio/plugin-svelte"
relationships:
  - target: DOC-SVE-f7ed7a62
    type: synchronised-with  - target: DOC-SVE-5d832d1d
    type: synchronised-with

---

# Svelte Plugin Installation

This skill is consumed by the core installer agent when setting up the Svelte plugin.

## Detection

Identify sub-projects that should receive this plugin by checking for:
- `tsconfig.json` or `.ts`/`.tsx` files → TypeScript project
- `.svelte` files → Svelte project
- `package.json` with svelte in dependencies → Svelte project
- `svelte.config.js` or `svelte.config.ts` → SvelteKit project

## Dependencies

Add to the target project's `package.json` devDependencies:

```json
{
  "eslint": "^9.0.0",
  "@typescript-eslint/eslint-plugin": "^8.0.0",
  "@typescript-eslint/parser": "^8.0.0",
  "eslint-plugin-svelte": "^3.0.0",
  "svelte-check": "^4.0.0",
  "vitest": "^3.0.0",
  "@testing-library/svelte": "^5.0.0",
  "typescript": "^5.9.0"
}
```

Only add dependencies that aren't already present. Run `npm install` after adding.

## Initial Config Generation

After dependencies are installed, generate config files from the project's coding standards rules. If no rules exist yet, create a default coding standards rule (RULE-xxx) with sensible defaults.

## Organisation Mode

When installing to an org project:
1. Scan all sub-projects for TypeScript/Svelte files
2. Recommend sub-projects based on detected languages
3. Present selection UI (checkboxes with AI recommendations)
4. Install dependencies to each selected sub-project
5. Generate config in each selected sub-project from org-level rules

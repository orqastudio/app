---
id: "KNOW-6cfacbb2"
type: "knowledge"
name: "ESLint Config Management"
status: "active"
plugin: "@orqastudio/plugin-svelte"
relationships:
  - target: "DOC-fd1d12bb"
    type: "synchronised-with"
---

# ESLint Config Management

## How Enforcement Works

Coding standards rules in `.orqa/process/rules/` define enforcement entries for this plugin. Each entry specifies an ESLint rule and its severity:

```yaml
enforcement:
  - plugin: "@orqastudio/plugin-svelte"
    tool: eslint
    config:
      - rule: "@typescript-eslint/no-explicit-any"
        severity: error
      - rule: "svelte/no-reactive-reassign"
        severity: error
```

## Config Generation

When rules change, the configurator agent reads all enforcement entries for the `eslint` tool and generates `.eslintrc.json`:

1. Collect all enforcement entries from all rules targeting this plugin's `eslint` tool
2. Merge with any sub-project overrides
3. Generate the ESLint flat config
4. Write to the project root

## Adding a New Standard

To add a new coding standard:
1. Add an enforcement entry to the coding standards rule
2. The config generator picks it up and regenerates `.eslintrc.json`
3. `orqa check` runs ESLint with the updated config

## Sub-Project Overrides

A sub-project can override a specific rule by creating an override file that redeclares the config line with a different severity and a rationale.

---
id: TASK-f6b8d0ec
type: task
name: "Implement extends strategy in content sync"
status: done
description: Add a new content strategy type 'extends' alongside the existing 'copy' strategy. For extends, orqa install sets up an extends reference in the target config file rather than copying the source. The base config remains in the plugin.
relationships:
  - target: EPIC-d4a8c1e5
    type: delivers
    rationale: Phase 2 — config extends strategy
acceptance:
  - "Content entries in orqa-plugin.json can declare strategy: 'extends' or 'copy' (default)"
  - "Extends strategy writes/updates the extends reference in the target config file"
  - "Supports tsconfig.json (JSON extends field) and eslint flat config (JS import)"
  - "Plugin source remains the live base config — not copied"
  - "make check passes"
---

## Scope

### Manifest schema extension

```json
"config": {
  "tsconfig": {
    "source": "tsconfig.base.json",
    "target": "tsconfig.json",
    "strategy": "extends",
    "mechanism": "json-extends"
  },
  "eslint": {
    "source": "eslint.base.config.js",
    "target": "eslint.config.js",
    "strategy": "extends",
    "mechanism": "js-import"
  }
}
```

### Mechanisms

- `json-extends`: Sets the `"extends"` field in a JSON config (tsconfig, prettierrc)
- `js-import`: Adds an import + spread in a JS config (eslint flat config)

### Key files

- `libs/cli/src/lib/content-lifecycle.ts` — add extends handling to copyPluginContent or new function
- `libs/types/src/plugin.ts` — update PluginManifest content mapping type if needed

---
id: IDEA-8cee938b
title: "Core plugin enforcement — require one per lifecycle category"
status: captured
created: "2026-03-22"
relationships:
  - target: PILLAR-569581e0
    type: grounded
    rationale: "Clarity Through Structure — projects need all three lifecycle layers"
---

# Core Plugin Enforcement

## The Rule

The app must have at least one plugin installed for each lifecycle category:

| Category | Role tag | Purpose |
|----------|----------|---------|
| Framework | `core:framework` | Agent execution model (always present, `uninstallable: true`) |
| Governance | `core:governance` | Rules, decisions, lessons, enforcement |
| Discovery | `core:discovery` | Reasoning methodology, thinking modes |
| Delivery | `core:delivery` | Work planning and tracking |

Each category must have **at least one active plugin at all times**. Disabling a
core plugin is only allowed if another plugin with the same role tag is already
enabled. This ensures the end-to-end lifecycle always works — you can swap
agile-governance for a different governance model, but you can't have zero governance.

If any category drops to zero, the app shows a **global error banner** explaining
which category is missing and blocking further work until it's resolved.

## Implementation

1. On app startup and on any plugin enable/disable, scan for `role` field in `orqa-plugin.json`
2. Group by role prefix (`core:framework`, `core:governance`, `core:discovery`, `core:delivery`)
3. If any group has zero enabled plugins, show persistent error banner
4. **Disable gate**: when a user attempts to disable a core plugin, check if another
   plugin with the same role tag is enabled. If not, block the disable with:
   "Cannot disable {plugin} — it is the only {category} plugin. Enable an alternative first."
5. Same gate applies to `orqa plugin uninstall` in the CLI
6. `core:framework` has the additional `uninstallable: true` flag — cannot be removed at all

## Plugin Manifest Fields

```json
{
  "role": "core:framework",
  "uninstallable": true
}
```

- `role` — declares what lifecycle category this plugin serves
- `uninstallable` — when `true`, the app UI hides the uninstall button and the CLI refuses `orqa plugin uninstall`

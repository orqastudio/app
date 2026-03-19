---
id: SKILL-TAU-002
type: skill
name: Clippy Config Management
status: active
plugin: "@orqastudio/plugin-tauri"
relationships:
  - target: DOC-TAU-001
    type: synchronised-with
---

# Clippy Config Management

## How Enforcement Works

Coding standards rules define enforcement entries for the clippy tool:

```yaml
enforcement:
  - plugin: "@orqastudio/plugin-tauri"
    tool: clippy
    config:
      - lint: "clippy::unwrap_used"
        level: deny
      - lint: "clippy::expect_used"
        level: deny
      - lint: "clippy::pedantic"
        level: warn
```

## Config Generation

The configurator reads enforcement entries and generates `clippy.toml`:

```toml
[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
pedantic = "warn"
```

## rustfmt Config

Formatting rules generate `.rustfmt.toml`:

```yaml
enforcement:
  - plugin: "@orqastudio/plugin-tauri"
    tool: rustfmt
    config:
      - key: "max_width"
        value: 100
      - key: "edition"
        value: "2021"
```

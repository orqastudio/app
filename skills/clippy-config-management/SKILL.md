---
name: clippy-config-management
description: "clippy-config-management"
user-invocable: false
---

# Clippy Config Management

## How Enforcement Works

Coding standards rules define enforcement entries for the clippy tool:

```yaml
enforcement:
  - plugin: "@orqastudio/plugin-rust"
    tool: clippy
    config:
      - lint: "clippy::unwrap_used"
        level: deny
      - lint: "clippy::expect_used"
        level: deny
      - lint: "clippy::pedantic"
        level: warn
      # Ban bare println/eprintln — use the tracing crate or structured logging
      - lint: "clippy::print_stdout"
        level: deny
      - lint: "clippy::print_stderr"
        level: deny
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
  - plugin: "@orqastudio/plugin-rust"
    tool: rustfmt
    config:
      - key: "max_width"
        value: 100
      - key: "edition"
        value: "2021"
```

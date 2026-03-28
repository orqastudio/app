---
id: KNOW-d4095bd9
type: knowledge
name: Clippy Config Management
summary: "How Rust clippy and rustfmt configuration is managed through coding standards rules. Enforcement entries in rule artifacts define lint levels (deny, warn) for specific clippy lints like unwrap_used, expect_used, pedantic, print_stdout. The configurator reads these entries and generates clippy.toml and .rustfmt.toml files. All lint policy is governed by rules, not manually configured."
status: active
plugin: "@orqastudio/plugin-rust"
relationships:
  - target: DOC-2372ed36
    type: synchronised-with
  - target: AGENT-26e5029d
    type: employed-by
  - target: AGENT-065a25cc
    type: employed-by
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
```text

## Config Generation

The configurator reads enforcement entries and generates `clippy.toml`:

```toml
[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
pedantic = "warn"
```text

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
```text

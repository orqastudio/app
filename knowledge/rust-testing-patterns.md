---
id: KNOW-RST-214b7cdc
type: knowledge
name: Rust Testing Patterns
status: active
plugin: "@orqastudio/plugin-rust"
relationships:
  - target: DOC-RST-27becb92
    type: synchronised-with
  - target: AGENT-RST-4241392c
    type: employed-by
  - target: AGENT-RST-spec-a3f7d2b1
    type: employed-by
---

# Rust Testing Patterns

## Test Organisation

Tests live in `#[cfg(test)]` modules at the bottom of each source file. Integration tests in a `tests/` directory.

## Convention

- Unit tests in the same file as the code they test
- Use `#[test]` attribute, not a test framework
- Use `assert!`, `assert_eq!`, `assert_ne!` — no custom assertion macros
- Helper functions in test modules prefixed with `make_` for fixtures

## Running Tests

```bash
cargo test                    # All tests
cargo test -- --test-threads=1  # Sequential (for tests with shared state)
cargo test my_module          # Filter by module name
```

## Enforcement

Test requirements are defined in coding standards rules with enforcement entries for the `cargo-test` tool.

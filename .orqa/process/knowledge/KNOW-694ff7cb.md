---
id: KNOW-694ff7cb
type: knowledge
name: Rust Testing Patterns
summary: "Rust Testing Patterns. Tests live in `#[cfg(test)]` modules at the bottom of each source file. Integration tests in a `tests/` directory."
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

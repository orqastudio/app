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

Unit tests live in `#[cfg(test)]` modules at the bottom of each source file. Integration tests go in a top-level `tests/` directory with one file per test scenario.

## Conventions

- Unit tests: same file as the code they test, inside `#[cfg(test)] mod tests { ... }`
- Use `#[test]` attribute — no external test framework
- Assertions: `assert!`, `assert_eq!`, `assert_ne!` — no custom assertion macros
- Fixture helpers prefixed with `make_` (e.g., `make_artifact()`, `make_config()`)
- Error case tests: use `#[should_panic(expected = "...")]` or match on `Result::Err`

## Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn make_artifact() -> Artifact {
        Artifact { id: "TEST-001".into(), title: "Test".into() }
    }

    #[test]
    fn parse_valid_artifact() {
        let artifact = make_artifact();
        let result = parse(&artifact.to_string());
        assert_eq!(result.unwrap().id, "TEST-001");
    }

    #[test]
    fn parse_missing_id_returns_error() {
        let result = parse("---\ntitle: No ID\n---\n");
        assert!(result.is_err());
    }
}
```text

## Running Tests

```bash
orqa test                          # Preferred — runs all test suites
cargo test                         # All Rust tests (use orqa test instead)
cargo test -- --test-threads=1     # Sequential (shared state)
cargo test my_module               # Filter by module name
```text

## Integration Tests

Integration tests in `tests/` can access the crate's public API. Each file is compiled as a separate crate. Use `tests/common/mod.rs` for shared helpers across integration test files.

## Enforcement

Test requirements are defined in coding standards rules with enforcement entries for the `cargo-test` tool. The `orqa check` runner includes test execution as part of the quality gate.

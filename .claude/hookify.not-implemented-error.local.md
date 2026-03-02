---
name: block-not-implemented-error
enabled: true
event: file
action: block
conditions:
  - field: file_path
    operator: regex_match
    pattern: src-tauri/src/.*\.rs$
  - field: new_text
    operator: regex_match
    pattern: (todo!\(|unimplemented!\(|panic!\(|\.unwrap\(\)|\.expect\()
---

**BLOCKED: `todo!()`, `unimplemented!()`, `panic!()`, `.unwrap()`, and `.expect()` are forbidden in production Rust code.**

- Use `Result<T, E>` with `thiserror` for error propagation
- Use `?` operator to propagate errors
- Use `.ok_or()` or `.ok_or_else()` to convert `Option` to `Result`
- `unwrap()` and `expect()` are only permitted in test code (`#[cfg(test)]`)

```rust
// WRONG
let value = config.get("key").unwrap();

// RIGHT
let value = config.get("key").ok_or_else(|| AppError::MissingConfig("key".into()))?;
```

See: `.claude/rules/no-stubs.md` — No stubs, placeholders, or hardcoded fake data
See: `.claude/rules/coding-standards.md` — "NO `unwrap()`, `expect()`, or `panic!()` in production code"

---
name: warn-mock-in-tests
enabled: true
event: file
action: warn
conditions:
  - field: file_path
    operator: regex_match
    pattern: src-tauri/.*test.*\.rs$
  - field: new_text
    operator: regex_match
    pattern: (mock!|#\[mockall|MockAll|mockall::)
---

**Mock library usage detected in tests — use trait-based test doubles instead.**

`mockall` macros are discouraged. Implement the trait directly as a test double struct. This keeps tests readable, explicit, and avoids macro-generated code that hides behavior.

**Correct:**

```rust
struct MockSessionRepository {
    sessions: Vec<Session>,
}

impl SessionRepository for MockSessionRepository {
    fn list(&self) -> Result<Vec<Session>> {
        Ok(self.sessions.clone())
    }
}
```

**Discouraged:**

```rust
use mockall::automock;

#[automock]
trait SessionRepository { ... }
```

See: `.claude/rules/testing-standards.md` — "Mock ONLY at the adapter/boundary layer."

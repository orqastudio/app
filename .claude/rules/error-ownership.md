# Error Ownership

**Source of Truth:** `@TODO.md` -> "HOW TO WORK" section

## Rule (NON-NEGOTIABLE)

**ALL errors are YOUR responsibility. No exceptions.**

- Do NOT claim "this error existed before"
- Do NOT skip or ignore failures
- Do NOT commit with failing checks
- Pre-existing errors: Fix them as part of your commit

## Integration Verification

**NEVER assume. ALWAYS verify.**

Before calling ANY existing function or API:

1. **Read the source** — Check actual function signature
2. **Check the types** — Verify parameter names and types
3. **Run checks** — `cargo clippy` for Rust, `npm run check` for TypeScript — catch mismatches immediately

**NO backwards compatibility shims.** Fix ALL callers in same commit.

## ChunkHound Integration

Use `search_regex` to find function definitions before calling them — faster and more thorough than manual file reading. Use `search_semantic` for "how does X work" questions.

## Related Rules

- `coding-standards.md` — defines *what* to verify (specific checks and patterns)
- `chunkhound-usage.md` — tools for finding and verifying code before modifying it
- `end-to-end-completeness.md` — the full chain that must be verified

---
id: KNOW-1a4f41f7
type: knowledge
name: Dependency License Compatibility
status: active
category: domain
user-invocable: true
relationships:
  - target: DOC-1a4f41f7
    type: synchronised-with
  - target: DOC-9814ec3c
    type: synchronised-with

---

# Dependency License Compatibility

Before adding any dependency (npm package, Rust crate, or vendored code), verify its license is compatible with ours.

## Quick Decision

| Dependency license | Action |
|-------------------|--------|
| MIT, Apache-2.0, BSD, ISC, 0BSD, Unlicense | Safe. Proceed. |
| MPL-2.0 | OK if importing, not if modifying source files. |
| LGPL-2.1/3.0 | OK as a linked dependency, not if copying source. |
| GPL-2.0/3.0, AGPL-3.0, SSPL | **Stop. Do not use.** Find an alternative. |
| No license / custom | **Stop. Do not use.** |

## How to Check

Before `npm install` or adding to `Cargo.toml`:

1. Look at the package's `license` field or `LICENSE` file
2. If MIT/Apache/BSD → proceed
3. If GPL/AGPL/SSPL/none → find an alternative
4. If unsure → check the full dependency tree for copyleft contamination

```bash
# npm — check all dependency licenses
npx license-checker --summary

# Rust — check all dependency licenses
cargo license
```

## Transitive Dependencies

A dependency's dependencies count too. If package A is MIT but depends on package B which is GPL, you have a GPL problem. Always check the full tree, not just the direct dependency.

## Why This Matters

Our project uses BSL-1.1 with ethical use grants. GPL is copyleft — it would require our entire project to be relicensed as GPL, which would remove our ethical protections and SaaS restrictions. This is not negotiable.

## When a Dependency Changes License

Pin the version before the change. The old version keeps its old license. Then evaluate: migrate to an alternative, or accept the new terms if compatible.
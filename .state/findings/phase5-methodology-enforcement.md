# Phase 5 Finding: One-Methodology Enforcement

**Date:** 2026-03-29
**Task:** P5-30

## Finding

The CLI enforces a one-methodology constraint during plugin installation.

## Evidence

- `cli/src/lib/installer.ts:128`: `enforceOneMethodology()` function
- Reads `manifest.purpose` array, checks for "methodology" value
- Scans installed plugins for existing methodology plugin
- Returns conflict object if another methodology is already installed
- `cli/src/commands/plugin.ts:424-431`: Blocks install with error message on conflict

## Conclusion

One-methodology constraint is mechanically enforced at install time. No runtime bypass possible.

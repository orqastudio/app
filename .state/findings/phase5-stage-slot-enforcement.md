# Phase 5 Finding: One-Per-Stage-Slot Enforcement

**Date:** 2026-03-29
**Task:** P5-31

## Finding

The CLI enforces a one-per-stage-slot constraint during plugin installation.

## Evidence

- `cli/src/lib/installer.ts:166`: `enforceOnePerStage()` function
- Reads `manifest.stage_slot` from the new plugin
- Scans installed plugins for existing stage_slot match
- Returns conflict object if another plugin claims the same slot
- `cli/src/commands/plugin.ts`: Calls enforcement during install

## Conclusion

One-per-stage-slot constraint is mechanically enforced at install time.

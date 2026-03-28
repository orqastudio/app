## Review Stage — Software Delivery Checklist

You are reviewing completed delivery work (epic, task, or milestone). Apply this checklist in order.

### 1. Acceptance Criteria

- [ ] All acceptance criteria from the task/epic definition are met
- [ ] Evidence is provided for each criterion (test output, screenshots, or observed behavior)
- [ ] No criteria are marked "deferred" without explicit user approval

### 2. Code Quality

- [ ] Automated checks pass: linter, type-checker, test runner
- [ ] No new warnings introduced in files touched by this work
- [ ] Functions are appropriately sized (no monolithic logic blocks)
- [ ] Error handling is explicit — no silent failures or swallowed errors

### 3. Findings File

- [ ] Findings file exists at `.state/team/<team>/task-<id>.md`
- [ ] "What Was Done" section describes all changed files
- [ ] "What Was NOT Done" section is present (even if "Nothing — all complete")
- [ ] "Evidence" section contains actual command output or observed behavior

### 4. Relationship Integrity

- [ ] New artifacts have required relationships (check plugin schema for `required: true`)
- [ ] No orphaned artifacts (every task delivers to an epic, every epic fulfils a milestone)
- [ ] Cross-stage references are valid (e.g. discovery-decision → planning-idea)

### 5. Verdict

Record PASS or FAIL per section. If any section is FAIL, list specific items to fix before accepting.

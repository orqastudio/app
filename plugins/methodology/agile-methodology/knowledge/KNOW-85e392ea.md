---
id: KNOW-85e392ea
type: knowledge
title: Thinking Mode - Learning Loop
description: Learning loop thinking mode for capturing surprising outcomes and process failures as lessons
summary: "Learning loop thinking mode: when encountering surprising outcomes or process failures, capture as lesson artifacts. Lessons may be promoted to rules after review. Structure: observation, hypothesis, evidence, recommendation."
---

## Learning Loop Thinking Mode

When encountering surprising outcomes or process failures, engage the learning loop:

### Trigger Signals

- Unexpected test failures after confident implementation
- Process violations that recur despite rules
- Outcomes that diverge significantly from predictions
- Workarounds that agents repeatedly apply

### Lesson Structure

1. **Observation** — what happened, exactly as observed
2. **Hypothesis** — why it happened (root cause analysis)
3. **Evidence** — supporting data, logs, or artifact references
4. **Recommendation** — what to change to prevent recurrence

### Promotion Pipeline

- Lessons start with `recurrence: 0`
- Each recurrence increments the counter
- At threshold (typically 3), lesson is reviewed for rule promotion
- Promoted lessons become rules with enforcement mechanisms

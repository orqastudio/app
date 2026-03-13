---
id: TASK-130
title: "Implement tool call rendering"
description: "Built tool call card components with collapsible display showing tool name, input parameters, and output results."
status: done
created: "2026-03-02"
updated: "2026-03-02"
epic: EPIC-030
depends-on: []
acceptance:
  - Tool calls render inline within conversation messages
  - Cards are collapsible with input/output visible when expanded
  - Tool approval flow works
---
## What

Built the `ToolCallCard` component for rendering tool calls inline within conversation messages, with collapsible input/output display and approval UI.

## How

Implemented the card using shadcn-svelte primitives with a collapsible body showing formatted JSON input and result output. Tool approval state is managed in the conversation store and surfaced via props.

## Verification

Tool call cards render inline in messages, expand to show input and output, and the approval flow correctly gates execution.

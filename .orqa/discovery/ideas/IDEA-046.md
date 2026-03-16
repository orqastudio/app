---
id: IDEA-046
title: Full-Width Chat Input with Floating Controls
description: "Redesign the chat input area so the textarea fills the entire chat footer region. Action buttons, character count, model indicator, and other metadata are absolutely positioned over or around the input, reducing visual clutter and making the input feel spacious rather than cramped."
status: captured
created: 2026-03-10
updated: 2026-03-13
horizon: later
research-needed:
  - "Inventory of current footer elements (buttons, labels, hints) and their placement"
  - "Reference UX patterns from ChatGPT, Claude.ai, Linear, Notion AI for full-width input areas"
  - Accessibility implications of absolutely positioned controls overlapping the textarea
  - Responsive behavior when input grows multi-line
relationships:
  - target: PILLAR-001
    type: grounded-by
---
## Motivation

The current chat footer feels crowded — the textarea shares horizontal and vertical space with action buttons, description text, and other controls. This makes the input area feel smaller than it needs to be and adds visual noise when the user is focused on composing a message.

By making the textarea fill the full footer region and floating the controls (send button, model selector, etc.) as absolutely positioned overlays, the input becomes the dominant element. The controls are still accessible but visually subordinate, creating a calmer, more spacious feel.

## Sketch

- Textarea fills 100% of the chat footer container (no side margins for buttons)
- Send button floats in the bottom-right corner of the textarea
- Model indicator / token count floats in the bottom-left as subtle text
- Any description or hint text floats above the textarea as a subtle overlay that fades on focus
- Controls use pointer-events and z-index so they don't interfere with typing
- Multi-line expansion pushes the footer height up naturally, controls stay anchored to their corners

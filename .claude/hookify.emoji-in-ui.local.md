---
name: warn-emoji-in-ui
enabled: true
event: file
action: warn
conditions:
  - field: file_path
    operator: regex_match
    pattern: src/.*\.svelte$
  - field: new_text
    operator: regex_match
    pattern: "[^\\x00-\\x7F]{2,}"
---

**Emoji detected in UI component.**

Use Lucide icons for all visual indicators, not emoji. Emoji is only permitted for emotional reactions in conversational text (e.g., chat messages).

```svelte
// WRONG
<span>&#x2705; Connected</span>

// RIGHT
import Check from "lucide-svelte/icons/check";
<Check class="text-green-500" />
```

See: `.claude/rules/coding-standards.md` — "NO emoji in UI — use Lucide icons for all visual indicators."

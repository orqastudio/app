---
name: block-invoke-in-components
enabled: true
event: file
action: block
conditions:
  - field: file_path
    operator: regex_match
    pattern: ui/lib/components/.*\.svelte$
  - field: new_text
    operator: regex_match
    pattern: invoke\s*\(|from\s+['"]@tauri-apps/api
---

**BLOCKED: Display components must not call Tauri commands or import Tauri APIs.**

Pages and containers fetch data via `invoke()`. Display components receive data via props only. No `invoke()` calls or `@tauri-apps/api` imports in `$lib/components/`.

**Correct pattern:**

```svelte
<!-- ui/routes/sessions/+page.svelte (page fetches) -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import SessionList from '$lib/components/SessionList.svelte';
  let sessions = $state([]);
  $effect(() => { invoke('list_sessions').then(s => sessions = s); });
</script>
<SessionList {sessions} />

<!-- ui/lib/components/SessionList.svelte (component displays) -->
<script lang="ts">
  let { sessions } = $props();
</script>
```

See: `.claude/rules/architecture-decisions.md` — "Component purity: Display components receive props only."
See: `.claude/rules/end-to-end-completeness.md` — "Store binding manages state; components display it."

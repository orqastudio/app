<!-- IndentedBlock — left-border indented content container used in collapsible tool call cards.
     Provides a visual hierarchy cue: a 2px left border with margin and padding to indent
     nested content below a collapsible trigger. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let { children }: { children?: Snippet } = $props();
</script>

<div class="border-border mt-1 ml-3 border-l-2 pl-4">
	{@render children?.()}
</div>

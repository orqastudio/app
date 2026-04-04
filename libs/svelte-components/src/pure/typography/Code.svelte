<!-- Inline code element with muted background and monospace font. Renders as <code> with visual pill styling for short inline code snippets within prose. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	export interface CodeProps {
		children?: Snippet;
	}

	let { children }: CodeProps = $props();
</script>

<code class="text-xs font-mono rounded bg-muted px-1.5 py-0.5">
	{@render children?.()}
</code>

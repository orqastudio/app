<!-- PreText — preformatted error/diagnostic text block.
     Renders a <pre> with white-space pre-wrap and caption-sized monospace font,
     inheriting color from the parent (used inside Callout for error details). -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let { children }: { children?: Snippet } = $props();
</script>

<pre class="mt-1 font-mono text-xs whitespace-pre-wrap">{@render children?.()}</pre>

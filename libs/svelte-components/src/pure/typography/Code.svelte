<!-- Code element. Inline mode renders a <code> pill with muted background for short snippets within prose. Block mode renders a <pre> for multi-line code or log output. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	export interface CodeProps {
		// When true, renders as a <pre> block suitable for multi-line code or log output.
		block?: boolean;
		children?: Snippet;
	}

	let { block = false, children }: CodeProps = $props();
</script>

{#if block}
	<pre
		class="bg-muted overflow-x-auto rounded p-3 font-mono text-xs whitespace-pre-wrap">{@render children?.()}</pre>
{:else}
	<code class="bg-muted rounded px-1.5 py-0.5 font-mono text-xs">{@render children?.()}</code>
{/if}

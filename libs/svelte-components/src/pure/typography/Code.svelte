<!-- Code element. Inline mode renders a <code> pill with muted background for short snippets within prose. Block mode renders a <pre> for multi-line code or log output.
     The compact variant of block mode applies a max-height with scroll for use in dense devtools panels. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	export interface CodeProps {
		// When true, renders as a <pre> block suitable for multi-line code or log output.
		block?: boolean;
		// When true (requires block=true), limits height to 15rem with overflow scroll for compact devtools panels.
		compact?: boolean;
		// When true, truncates inline code with ellipsis and hides overflow. Use inside a constrained container.
		truncate?: boolean;
		children?: Snippet;
	}

	let { block = false, compact = false, truncate = false, children }: CodeProps = $props();
</script>

{#if block}
	<pre
		class={compact
			? "bg-muted max-h-60 overflow-auto rounded p-3 font-mono text-[11px] whitespace-pre-wrap"
			: "bg-muted overflow-x-auto rounded p-3 font-mono text-xs whitespace-pre-wrap"}>{@render children?.()}</pre>
{:else}
	<code
		class={truncate
			? "bg-muted block max-w-full overflow-hidden rounded px-1.5 py-0.5 font-mono text-xs text-ellipsis whitespace-nowrap"
			: "bg-muted rounded px-1.5 py-0.5 font-mono text-xs"}>{@render children?.()}</code
	>
{/if}

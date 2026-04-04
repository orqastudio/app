<script lang="ts">
	import { cn } from "../../utils/cn.js";

	interface CardContentProps {
		/** Reduce padding for compact layouts. */
		compact?: boolean;
		/** Remove all padding (flush content against card edges). */
		flush?: boolean;
		children?: import("svelte").Snippet;
	}

	let {
		compact = false,
		flush = false,
		children,
	}: CardContentProps = $props();
</script>

<div
	data-slot="card-content"
	class={cn(
		flush ? "px-0 pb-0" : compact ? "px-4 pb-3" : "px-6 pb-6",
	)}
>
	{@render children?.()}
</div>

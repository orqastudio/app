<!-- LogViewport — scrollable viewport for virtualised log tables.
     Provides the scrollable container that the virtualiser's ResizeObserver binds to
     via the ref prop. The ref must be bound for virtualisation to function correctly.

     Exposes role="table", aria-label, and aria-rowcount for accessibility.
     Also exposes onscroll for the virtualiser's scroll position tracking.

     This component exists to keep the bind:this div (required for virtualisation)
     inside the library where raw HTML is permitted. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		ref = $bindable<HTMLDivElement | null>(null),
		ariaLabel,
		ariaRowCount,
		onscroll,
		children,
	}: {
		/** Bindable reference to the underlying div element. Required for virtualisation. */
		ref?: HTMLDivElement | null;
		/** Accessible label for the table region. */
		ariaLabel?: string;
		/** Total number of rows in the virtualised dataset (for aria-rowcount). */
		ariaRowCount?: number;
		/** Scroll event handler for virtualiser scroll position tracking. */
		onscroll?: (e: Event) => void;
		children?: Snippet;
	} = $props();
</script>

<!-- position:relative for absolute-positioned child rows; flex-1 fills remaining height. -->
<div
	data-slot="log-viewport"
	bind:this={ref}
	class="relative flex-1 overflow-x-hidden overflow-y-auto"
	role="table"
	aria-label={ariaLabel}
	aria-rowcount={ariaRowCount}
	{onscroll}
>
	{@render children?.()}
</div>

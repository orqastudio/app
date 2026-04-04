<script lang="ts">
	import { cn } from "../../utils/cn.js";

	interface CardFooterProps {
		/** Reduce top padding for compact layouts. */
		compact?: boolean;
		children?: import("svelte").Snippet;
	}

	let {
		compact = false,
		children,
	}: CardFooterProps = $props();
</script>

<div
	data-slot="card-footer"
	class={cn(
		"flex items-center px-6",
		compact ? "pt-2" : "[.border-t]:pt-6",
	)}
>
	{@render children?.()}
</div>

<script lang="ts">
	import { cn } from "../../utils/cn.js";

	interface CardHeaderProps {
		/** Reduce vertical padding for compact layouts. */
		compact?: boolean;
		/** Center content horizontally. */
		center?: boolean;
		children?: import("svelte").Snippet;
	}

	let { compact = false, center = false, children }: CardHeaderProps = $props();
</script>

<div
	data-slot="card-header"
	class={cn(
		"@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6",
		compact ? "pt-3 pb-1" : "pt-6",
		center && "justify-items-center text-center",
	)}
>
	{@render children?.()}
</div>

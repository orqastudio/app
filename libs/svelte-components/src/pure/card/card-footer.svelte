<script lang="ts">
	import { cn } from "../../utils/cn.js";
	import type { HTMLAttributes } from "svelte/elements";
	import type { Snippet } from "svelte";

	interface CardFooterProps extends Omit<HTMLAttributes<HTMLDivElement>, "class" | "style"> {
		/** Reduce top padding for compact layouts. */
		compact?: boolean;
		children?: Snippet;
	}

	let { compact = false, children, ...restProps }: CardFooterProps = $props();
</script>

<div
	data-slot="card-footer"
	class={cn("flex items-center px-6", compact ? "pt-2" : "[.border-t]:pt-6")}
	{...restProps}
>
	{@render children?.()}
</div>

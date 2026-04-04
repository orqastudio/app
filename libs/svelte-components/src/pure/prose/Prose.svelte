<!-- Container for rendered markdown/rich text with consistent typography. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	let {
		size = "base",
		children,
	}: {
		size?: "sm" | "base" | "lg";
		children?: Snippet;
	} = $props();
</script>

<div
	class={cn(
		"prose dark:prose-invert max-w-none",
		size === "sm" && "prose-sm",
		size === "lg" && "prose-lg",
	)}
>
	{@render children?.()}
</div>

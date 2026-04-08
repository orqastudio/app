<!-- Fixed-width sidebar panel with a right border divider.
     Use inside an HStack to pair it with a flex-1 main content area. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Closed set of sidebar widths matching the design system's spacing scale.
	const widthMap: Record<string, string> = {
		sm: "w-48",
		md: "w-60",
		lg: "w-72",
		xl: "w-80",
	};

	let {
		width = "md",
		border = "right",
		children,
	}: {
		/** Width preset for the sidebar. */
		width?: "sm" | "md" | "lg" | "xl";
		/** Which side to show a border divider on. */
		border?: "right" | "left" | "none";
		children?: Snippet;
	} = $props();

	const borderClass = $derived(
		border === "right"
			? "border-r border-border"
			: border === "left"
				? "border-l border-border"
				: "",
	);
</script>

<div class={cn("h-full shrink-0 overflow-hidden", widthMap[width] ?? widthMap.md, borderClass)}>
	{@render children?.()}
</div>

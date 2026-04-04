<!-- Vertical stack (flex column) layout primitive. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Maps numeric gap values to Tailwind gap classes.
	const gapMap: Record<number, string> = {
		0: "gap-0",
		0.5: "gap-0.5",
		1: "gap-1",
		1.5: "gap-1.5",
		2: "gap-2",
		3: "gap-3",
		4: "gap-4",
		6: "gap-6",
		8: "gap-8",
	};

	let {
		gap = 2,
		align = "start",
		children,
	}: {
		gap?: 0 | 0.5 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8;
		align?: "start" | "center" | "end" | "stretch";
		children?: Snippet;
	} = $props();

	const gapClass = $derived(gapMap[gap] ?? "gap-2");
</script>

<div
	class={cn(
		"flex flex-col",
		gapClass,
		align === "center" && "items-center",
		align === "end" && "items-end",
		align === "stretch" && "items-stretch",
		align === "start" && "items-start",
	)}
>
	{@render children?.()}
</div>

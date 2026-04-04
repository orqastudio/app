<!-- Horizontal stack (flex row) layout primitive. Supports semantic interactive
     attributes (onclick, role, tabindex, aria-*) for accessible interactive rows. -->
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

	// Maps justify prop values to Tailwind justify-content classes.
	const justifyMap: Record<string, string> = {
		start: "justify-start",
		center: "justify-center",
		end: "justify-end",
		between: "justify-between",
		around: "justify-around",
	};

	// Maps align prop values to Tailwind align-items classes.
	const alignMap: Record<string, string> = {
		start: "items-start",
		center: "items-center",
		end: "items-end",
		baseline: "items-baseline",
		stretch: "items-stretch",
	};

	let {
		gap = 2,
		align = "center",
		justify = "start",
		wrap = false,
		full = false,
		role,
		tabindex,
		"aria-selected": ariaSelected,
		"aria-label": ariaLabel,
		onclick,
		onkeydown,
		style,
		children,
	}: {
		gap?: 0 | 0.5 | 1 | 1.5 | 2 | 3 | 4 | 6 | 8;
		align?: "start" | "center" | "end" | "baseline" | "stretch";
		justify?: "start" | "center" | "end" | "between" | "around";
		wrap?: boolean;
		/** Fill available width. */
		full?: boolean;
		role?: string;
		tabindex?: number;
		"aria-selected"?: boolean;
		"aria-label"?: string;
		onclick?: (e: MouseEvent) => void;
		onkeydown?: (e: KeyboardEvent) => void;
		/** Inline style for dynamic positioning (e.g. indentation). */
		style?: string;
		children?: Snippet;
	} = $props();

	const gapClass = $derived(gapMap[gap] ?? "gap-2");
	const justifyClass = $derived(justifyMap[justify] ?? "justify-start");
	const alignClass = $derived(alignMap[align] ?? "items-center");
</script>

<div
	class={cn(
		"flex",
		gapClass,
		justifyClass,
		alignClass,
		wrap && "flex-wrap",
		full && "w-full",
		onclick && "cursor-pointer",
	)}
	{role}
	{tabindex}
	aria-selected={ariaSelected}
	aria-label={ariaLabel}
	{onclick}
	{onkeydown}
	{style}
>
	{@render children?.()}
</div>

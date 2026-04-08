<!-- SidePanel — a fixed-width side drawer that anchors to the right edge of its
     nearest positioned ancestor. Used for help panels, detail panels, and other
     persistent overlays that sit alongside the main content area.

     Renders as a flex-column container with full viewport height, a left border,
     surface background, and a drop shadow. The width is fixed by the `width` prop.

     Callers are responsible for positioning the panel (typically inside a
     Box position="fixed" right={0} top={0}) and providing a scrollable interior
     via ScrollArea. This component only establishes the visual shell. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Closed-set width tokens for the panel. Adding values means extending
	// the design token vocabulary first.
	const widthMap: Record<string, string> = {
		xs: "w-64", // 256px
		sm: "w-72", // 288px
		md: "w-80", // 320px — default, matches w-80 convention
		lg: "w-96", // 384px
	};

	let {
		width = "md",
		role,
		"aria-label": ariaLabel,
		children,
	}: {
		/** Panel width preset. */
		width?: "xs" | "sm" | "md" | "lg";
		role?: string;
		"aria-label"?: string;
		children?: Snippet;
	} = $props();

	const widthClass = $derived(widthMap[width] ?? widthMap.md);
</script>

<div
	data-slot="side-panel"
	class={cn(
		"border-border bg-surface-base flex h-screen flex-col overflow-hidden border-l shadow-xl",
		widthClass,
	)}
	{role}
	aria-label={ariaLabel}
>
	{@render children?.()}
</div>

<!-- SurfaceBox — a rounded, raised-surface container with optional flex centering.
     Used for chart wrappers, placeholder containers, and any region that needs
     bg-surface-raised + border-radius without the full Card padding/structure.

     Unlike Box (no visual opinions) and Card (header/content/footer structure),
     SurfaceBox is purely a background + border-radius shell.

     widthPx and heightPx accept runtime pixel dimensions for chart containers whose
     size is computed by the parent (e.g. from ResizeObserver). Svelte style directives
     are used internally so app/devtools code never writes raw style= attributes. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	const paddingMap: Record<string, string> = {
		none: "",
		sm: "p-2",
		md: "p-4",
		lg: "p-6",
	};

	let {
		center = false,
		overflow = "hidden",
		padding = "none",
		widthPx,
		heightPx,
		children,
	}: {
		/** When true, centers children using flex. Used for placeholders. */
		center?: boolean;
		/** Overflow behavior — hidden by default, auto for scrollable charts. */
		overflow?: "hidden" | "auto";
		/** Padding token. Default is none (flush edges). */
		padding?: "none" | "sm" | "md" | "lg";
		/** Explicit pixel width for data-driven chart containers. */
		widthPx?: number;
		/** Explicit pixel height for data-driven chart containers. */
		heightPx?: number;
		children?: Snippet;
	} = $props();

	const paddingClass = $derived(paddingMap[padding] ?? "");
</script>

<div
	data-slot="surface-box"
	class={cn(
		"bg-surface-raised rounded-md",
		overflow === "auto" ? "overflow-auto" : "overflow-hidden",
		center && "flex items-center justify-center",
		paddingClass,
	)}
	style:width={widthPx != null ? `${widthPx}px` : undefined}
	style:height={heightPx != null ? `${heightPx}px` : undefined}
>
	{@render children?.()}
</div>

<!-- LogRowShell — absolute-positioned row container for virtualised log tables.
     Provides the outer wrapper that the virtualiser positions via a typed topPx
     prop, plus level-based background tint variants.

     This component exists to keep the raw style directive (required for
     virtualiser-computed pixel offsets) inside the library where raw HTML is
     permitted, while keeping devtools component files free of raw HTML.

     Pass topPx={offset} where offset is the cumulative row height in pixels
     computed by the virtualiser. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	// Level-based visual tint variants for the row background.
	const levelVariantClasses: Record<string, string> = {
		debug: "opacity-50",
		warn: "bg-warning/5",
		error: "bg-destructive/[0.08]",
		perf: "bg-secondary/20",
	};

	let {
		level,
		topPx,
		children,
	}: {
		/** Log level for the row. Controls background tint. */
		level?: "debug" | "info" | "warn" | "error" | "perf";
		/** Virtualiser-computed top offset in pixels. Applied as style:top. */
		topPx?: number;
		children?: Snippet;
	} = $props();

	const levelClass = $derived(level ? (levelVariantClasses[level] ?? "") : "");
</script>

<!-- group class enables Tailwind group-hover on descendant LogRowActions. -->
<div
	data-slot="log-row"
	class={cn("group absolute right-0 left-0", levelClass)}
	style:top="{topPx}px"
	role="row"
>
	{@render children?.()}
</div>

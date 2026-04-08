<!-- CollapsibleSection — CollapsibleTrigger with closed-set styling variants.
     "muted" and "destructive" render a bordered card-style section header.
     "link" renders a plain text trigger (text-xs, muted foreground, hover to foreground).
     "subheading" renders an uppercase label trigger (text-[10px], tracking-wide, muted foreground).
     Default variant is "muted". -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import CollapsibleTrigger from "./collapsible-trigger.svelte";

	// Variant base classes — each variant encodes a complete visual treatment.
	const variantMap: Record<string, string> = {
		// Bordered card-style section header with muted background.
		muted:
			"hover:bg-muted/50 flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors",
		// Bordered card-style section header with destructive background.
		destructive:
			"hover:bg-muted/50 flex w-full items-center gap-2 rounded-lg border border-destructive/50 bg-destructive/5 px-3 py-2 text-left text-sm transition-colors",
		// Plain text trigger — muted foreground that brightens to foreground on hover.
		link: "text-muted-foreground hover:text-foreground flex w-full items-center gap-1 text-xs font-medium transition-colors",
		// Uppercase label trigger — same color as link but with tracking and uppercase treatment.
		subheading:
			"text-muted-foreground hover:text-foreground flex w-full items-center gap-1 text-[10px] font-medium tracking-wide uppercase transition-colors",
	};

	let {
		variant = "muted",
		children,
	}: {
		/**
		 * Visual variant. "muted" and "destructive" render a bordered card header.
		 * "link" renders a plain text trigger. "subheading" renders an uppercase label trigger.
		 */
		variant?: "muted" | "destructive" | "link" | "subheading";
		/** Content rendered inside the trigger. */
		children?: Snippet;
	} = $props();

	const variantClass = $derived(variantMap[variant] ?? variantMap.muted);
</script>

<CollapsibleTrigger class={variantClass}>
	{@render children?.()}
</CollapsibleTrigger>

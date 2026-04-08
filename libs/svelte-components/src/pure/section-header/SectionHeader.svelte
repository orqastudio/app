<!-- SectionHeader — horizontal header bar for a section, a semantic lego block.

Replaces the common pattern of an HStack with horizontal/vertical padding and a
bottom border used as a titled header row above content. Variants lock the
exact padding/height/border treatment — consumers cannot override any visual.

Slots:
  • start   — leading content (title, icon, breadcrumb)
  • end     — trailing content (actions, search, filters)
  • default — used when no start/end slots are provided (convenience)

Variants:
  • section    — primary header of a panel or route view (px-3 py-2 border-b)
  • subsection — nested/secondary header inside a section (px-3 py-2 border-b)
  • compact    — tight headers for dense lists (px-2 py-1 border-b) -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	const variantMap: Record<string, string> = {
		section: "px-3 py-2 border-b border-border",
		subsection: "px-3 py-2 border-b border-border",
		compact: "px-2 py-1 border-b border-border",
	};

	let {
		variant = "section",
		background = "none",
		position,
		start,
		end,
		children,
		role,
		"aria-label": ariaLabel,
	}: {
		variant?: "section" | "subsection" | "compact";
		background?: "none" | "card" | "muted" | "surface" | "primary-subtle";
		/**
		 * When "relative", sets position:relative and removes overflow:hidden so absolutely-positioned
		 * children (e.g. dropdown panels) are anchored to this header and not clipped.
		 */
		position?: "relative";
		start?: Snippet;
		end?: Snippet;
		children?: Snippet;
		role?: string;
		"aria-label"?: string;
	} = $props();

	const variantClass = $derived(variantMap[variant] ?? variantMap.section);

	const backgroundMap: Record<string, string> = {
		none: "",
		card: "bg-card",
		muted: "bg-muted",
		surface: "bg-surface",
		// Primary-tinted: used for historical-session banners in the log table.
		"primary-subtle": "bg-primary/8",
	};
	const backgroundClass = $derived(backgroundMap[background] ?? "");
</script>

<div
	class={cn(
		"flex items-center justify-between gap-2",
		position !== "relative" && "overflow-hidden",
		position === "relative" && "relative",
		variantClass,
		backgroundClass,
	)}
	{role}
	aria-label={ariaLabel}
>
	{#if start || end}
		<div class="flex min-w-0 items-center gap-2">
			{#if start}{@render start()}{/if}
		</div>
		<div class="flex items-center gap-2">
			{#if end}{@render end()}{/if}
		</div>
	{:else if children}
		{@render children()}
	{/if}
</div>

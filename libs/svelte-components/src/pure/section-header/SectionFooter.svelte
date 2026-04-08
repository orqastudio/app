<!-- SectionFooter — horizontal footer bar for a section, a semantic lego block.

Counterpart to SectionHeader. Used for bottom status bars, toolbars pinned to
the foot of a panel, and action rows beneath scrollable content. Variants lock
the exact padding/border treatment — consumers cannot override any visual.

Slots:
  • start   — leading content (status, info)
  • end     — trailing content (actions, buttons)
  • default — used when no start/end slots are provided

Variants:
  • section    — primary footer (px-3 py-2 border-t)
  • subsection — nested footer (px-3 py-1.5 border-t)
  • compact    — tight footers for dense panels (px-2 py-1 border-t) -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { cn } from "../../utils/cn.js";

	const variantMap: Record<string, string> = {
		section: "px-3 py-2 border-t border-border",
		subsection: "px-3 py-1.5 border-t border-border",
		compact: "px-2 py-1 border-t border-border",
		// App-level status bar: fixed 2rem height, small text, muted foreground, right-edge padding offset for status icons
		"status-bar":
			"h-8 px-4 pb-[0.25rem] border-t border-border text-xs text-muted-foreground bg-[color-mix(in_srgb,var(--color-muted)_30%,transparent)]",
	};

	const backgroundMap: Record<string, string> = {
		none: "",
		card: "bg-card",
		muted: "bg-muted",
		surface: "bg-surface",
		// Primary-tinted: used for historical-session banners.
		"primary-subtle": "bg-primary/8",
	};

	let {
		variant = "section",
		background = "none",
		start,
		end,
		children,
		role,
		"aria-label": ariaLabel,
	}: {
		variant?: "section" | "subsection" | "compact" | "status-bar";
		background?: "none" | "card" | "muted" | "surface";
		start?: Snippet;
		end?: Snippet;
		children?: Snippet;
		role?: string;
		"aria-label"?: string;
	} = $props();

	const variantClass = $derived(variantMap[variant] ?? variantMap.section);
	const backgroundClass = $derived(backgroundMap[background] ?? "");
</script>

<div
	class={cn(
		"flex items-center justify-between gap-2 overflow-hidden",
		variantClass,
		backgroundClass,
	)}
	{role}
	aria-label={ariaLabel}
>
	{#if start || end}
		<div class="flex min-w-0 shrink items-center gap-2">
			{#if start}{@render start()}{/if}
		</div>
		<div class="flex shrink-0 items-center gap-2">
			{#if end}{@render end()}{/if}
		</div>
	{:else if children}
		{@render children()}
	{/if}
</div>

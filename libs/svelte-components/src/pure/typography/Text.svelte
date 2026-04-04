<!-- Base typography component. Renders a span (or block element) with a locked variant that controls all typographic properties. No class prop — variants are the only styling API. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	// All supported text variants with their fixed Tailwind class combinations.
	export type TextVariant =
		| "body"
		| "body-muted"
		| "caption"
		| "label"
		| "overline"
		| "mono"
		| "tabular"
		| "heading-xl"
		| "heading-lg"
		| "heading-base"
		| "heading-sm";

	export type TextTone = "warning" | "destructive" | "success" | "muted";

	export interface TextProps {
		variant?: TextVariant;
		truncate?: boolean;
		tone?: TextTone;
		block?: boolean;
		children?: Snippet;
	}

	let { variant = "body", truncate = false, tone, block = false, children }: TextProps = $props();

	// Maps each variant to its fixed Tailwind class combination.
	const variantClasses: Record<TextVariant, string> = {
		"body": "text-sm text-foreground",
		"body-muted": "text-sm text-muted-foreground",
		"caption": "text-xs text-muted-foreground",
		"label": "text-sm font-medium",
		"overline": "text-xs font-semibold uppercase tracking-wide",
		"mono": "text-xs font-mono",
		"tabular": "text-xs font-mono tabular-nums",
		"heading-xl": "text-xl font-semibold tracking-tight",
		"heading-lg": "text-lg font-semibold",
		"heading-base": "text-base font-semibold",
		"heading-sm": "text-sm font-semibold",
	};

	// Maps semantic tone to its Tailwind color class override.
	const toneClasses: Record<TextTone, string> = {
		warning: "text-warning",
		destructive: "text-destructive",
		success: "text-success",
		muted: "text-muted-foreground",
	};

	// Heading variants render as semantic heading elements when used directly.
	const headingVariantElements: Partial<Record<TextVariant, string>> = {
		"heading-xl": "h1",
		"heading-lg": "h2",
		"heading-base": "h3",
		"heading-sm": "h4",
	};

	// Recompute classes and tag whenever props change.
	const classes = $derived(
		[
			variantClasses[variant],
			tone ? toneClasses[tone] : null,
			truncate ? "truncate" : null,
			block ? "block" : null,
		].filter(Boolean).join(" ")
	);

	// Determine the rendered element: heading variants get semantic h tags, block=true gets <p>, default is <span>.
	const tag = $derived(headingVariantElements[variant] ?? (block ? "p" : "span"));
</script>

{#if tag === "h1"}
	<h1 class={classes}>{@render children?.()}</h1>
{:else if tag === "h2"}
	<h2 class={classes}>{@render children?.()}</h2>
{:else if tag === "h3"}
	<h3 class={classes}>{@render children?.()}</h3>
{:else if tag === "h4"}
	<h4 class={classes}>{@render children?.()}</h4>
{:else if tag === "p"}
	<p class={classes}>{@render children?.()}</p>
{:else}
	<span class={classes}>{@render children?.()}</span>
{/if}

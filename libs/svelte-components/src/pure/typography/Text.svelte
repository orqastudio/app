<!-- Base typography component. Renders a span (or block element) with a locked variant that controls all typographic properties. No class prop — variants are the only styling API. -->
<script lang="ts">
	import type { Snippet } from "svelte";

	// All supported text variants with their fixed Tailwind class combinations.
	export type TextVariant =
		| "body"
		| "body-muted"
		| "body-strong"
		| "body-strong-muted"
		| "caption"
		| "caption-strong"
		| "caption-mono"
		| "caption-tabular"
		| "label"
		| "overline"
		| "overline-muted"
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
		lineClamp?: 1 | 2 | 3 | 4;
		children?: Snippet;
	}

	let {
		variant = "body",
		truncate = false,
		tone,
		block = false,
		lineClamp,
		children,
	}: TextProps = $props();

	// Maps each variant to its fixed Tailwind class combination.
	const variantClasses: Record<TextVariant, string> = {
		body: "text-sm text-foreground",
		"body-muted": "text-sm text-muted-foreground",
		"body-strong": "text-sm font-semibold text-foreground",
		"body-strong-muted": "text-sm font-semibold text-muted-foreground",
		caption: "text-xs text-muted-foreground",
		"caption-strong": "text-xs font-semibold text-muted-foreground",
		"caption-mono": "text-xs font-mono text-muted-foreground",
		"caption-tabular": "text-xs font-mono tabular-nums text-muted-foreground",
		label: "text-sm font-medium",
		overline: "text-xs font-semibold uppercase tracking-wide",
		"overline-muted": "text-xs font-semibold uppercase tracking-wide text-muted-foreground",
		mono: "text-xs font-mono",
		tabular: "text-xs font-mono tabular-nums",
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

	// Maps lineClamp value to its Tailwind class. Truncate takes precedence when both are set.
	const lineClampClasses: Record<1 | 2 | 3 | 4, string> = {
		1: "line-clamp-1",
		2: "line-clamp-2",
		3: "line-clamp-3",
		4: "line-clamp-4",
	};

	// Heading variants render as semantic heading elements when used directly.
	const headingVariantElements: Partial<Record<TextVariant, string>> = {
		"heading-xl": "h1",
		"heading-lg": "h2",
		"heading-base": "h3",
		"heading-sm": "h4",
	};

	// Recompute classes and tag whenever props change.
	// truncate wins over lineClamp: single-line hard truncation is stricter than multi-line clamping.
	const classes = $derived(
		[
			variantClasses[variant],
			tone ? toneClasses[tone] : null,
			truncate ? "truncate" : lineClamp ? lineClampClasses[lineClamp] : null,
			block ? "block" : null,
		]
			.filter(Boolean)
			.join(" "),
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

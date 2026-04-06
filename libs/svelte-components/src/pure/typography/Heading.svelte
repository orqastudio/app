<!-- Convenience heading component. Maps level (1–6) to a Text variant and renders the appropriate semantic h element. No size or weight props — level is the only API. -->
<script lang="ts">
	import Text from "./Text.svelte";
	import type { Snippet } from "svelte";

	type HeadingLevel = 1 | 2 | 3 | 4 | 5 | 6;

	export interface HeadingProps {
		level?: HeadingLevel;
		children?: Snippet;
	}

	let { level = 1, children }: HeadingProps = $props();

	// Maps heading level to the corresponding Text variant.
	const levelVariants: Record<
		HeadingLevel,
		"heading-xl" | "heading-lg" | "heading-base" | "heading-sm" | "overline"
	> = {
		1: "heading-xl",
		2: "heading-lg",
		3: "heading-base",
		4: "heading-sm",
		5: "overline",
		6: "overline",
	};

	// Recompute variant whenever level changes.
	const variant = $derived(levelVariants[level]);
</script>

{#if level === 1}
	<h1><Text {variant}>{@render children?.()}</Text></h1>
{:else if level === 2}
	<h2><Text {variant}>{@render children?.()}</Text></h2>
{:else if level === 3}
	<h3><Text {variant}>{@render children?.()}</Text></h3>
{:else if level === 4}
	<h4><Text {variant}>{@render children?.()}</Text></h4>
{:else if level === 5}
	<h5><Text {variant}>{@render children?.()}</Text></h5>
{:else}
	<h6><Text {variant}>{@render children?.()}</Text></h6>
{/if}

<!-- Alias for Text restricted to caption-family variants. Use for metadata, timestamps, and secondary context. -->
<script lang="ts">
	import Text from "./Text.svelte";
	import type { Snippet } from "svelte";

	// Caption exposes only the caption-family variants to enforce typographic discipline.
	export type CaptionVariant = "caption" | "caption-strong" | "caption-mono" | "caption-tabular";

	export interface CaptionProps {
		variant?: CaptionVariant;
		truncate?: boolean;
		tone?: "warning" | "destructive" | "success" | "muted";
		block?: boolean;
		lineClamp?: 1 | 2 | 3 | 4;
		children?: Snippet;
	}

	let { variant = "caption", truncate, tone, block, lineClamp, children }: CaptionProps = $props();
</script>

<Text {variant} {truncate} {tone} {block} {lineClamp}>
	{@render children?.()}
</Text>

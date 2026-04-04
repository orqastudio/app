<!-- Alias for Text variant="caption". Use for metadata, timestamps, and secondary context. -->
<script lang="ts">
	import Text from "./Text.svelte";
	import type { Snippet } from "svelte";

	export interface CaptionProps {
		truncate?: boolean;
		tone?: "warning" | "destructive" | "success" | "muted";
		block?: boolean;
		children?: Snippet;
	}

	let { truncate, tone, block, children }: CaptionProps = $props();
</script>

<Text variant="caption" {truncate} {tone} {block}>
	{@render children?.()}
</Text>

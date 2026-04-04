<!-- Text rendered vertically (writing-mode: vertical-rl) for collapsed column labels such as Kanban columns. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import Text, { type TextVariant, type TextTone } from "../typography/Text.svelte";

	let {
		variant = "caption",
		tone,
		children,
	}: {
		variant?: TextVariant;
		tone?: TextTone;
		children?: Snippet;
	} = $props();
</script>

<!-- writing-mode has no Tailwind utility — inline style is required here. -->
<span class="inline-block select-none" style="writing-mode: vertical-rl; transform: rotate(180deg);">
	<Text {variant} {tone}>{@render children?.()}</Text>
</span>

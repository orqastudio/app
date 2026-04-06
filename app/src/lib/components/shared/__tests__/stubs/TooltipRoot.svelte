<!-- Stub for TooltipRoot in tests. Renders children without requiring Provider context. -->
<script lang="ts">
	let { children }: { children?: import("svelte").Snippet } = $props();
</script>

{#if children}{@render children()}{/if}

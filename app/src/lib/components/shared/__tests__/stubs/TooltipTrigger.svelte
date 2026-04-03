<!-- Stub for TooltipTrigger in tests. Renders child snippet with empty props. -->
<script lang="ts">
	let { child, children }: { child?: import("svelte").Snippet<[{ props: Record<string, any> }]>; children?: import("svelte").Snippet } = $props();
</script>
{#if child}{@render child({ props: {} })}{:else if children}{@render children()}{/if}

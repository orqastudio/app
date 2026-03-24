<script lang="ts">
	import CodeBlock from "$lib/components/content/CodeBlock.svelte";

	let { content }: { content: string } = $props();
</script>

<CodeBlock text={content} lang="bash" />

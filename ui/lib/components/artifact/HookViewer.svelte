<script lang="ts">
	import CodeBlock from "$lib/components/content/CodeBlock.svelte";

	let { content }: { content: string } = $props();
</script>

<CodeBlock code={content} language="bash" />

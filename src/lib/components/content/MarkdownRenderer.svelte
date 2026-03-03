<script lang="ts">
	import SvelteMarkdown from "@humanspeak/svelte-markdown";

	let { content }: { content: string } = $props();
</script>

<div class="prose prose-sm dark:prose-invert max-w-none">
	<SvelteMarkdown source={content} />
</div>

<script lang="ts">
	import CopyIcon from "@lucide/svelte/icons/copy";
	import CheckIcon from "@lucide/svelte/icons/check";

	let {
		code,
		language = "",
	}: {
		code: string;
		language?: string;
	} = $props();

	let copied = $state(false);

	function copyToClipboard() {
		navigator.clipboard.writeText(code).then(() => {
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		});
	}
</script>

<div class="group relative rounded-md border border-border bg-muted/30">
	<div class="flex items-center justify-between border-b border-border px-3 py-1.5">
		{#if language}
			<span class="text-xs text-muted-foreground">{language}</span>
		{:else}
			<span></span>
		{/if}
		<button
			class="flex items-center gap-1 rounded px-1.5 py-0.5 text-xs text-muted-foreground opacity-0 transition-opacity hover:bg-accent group-hover:opacity-100"
			onclick={copyToClipboard}
		>
			{#if copied}
				<CheckIcon class="h-3.5 w-3.5 text-green-500" />
				<span>Copied</span>
			{:else}
				<CopyIcon class="h-3.5 w-3.5" />
				<span>Copy</span>
			{/if}
		</button>
	</div>
	<pre class="overflow-x-auto p-3 text-sm"><code>{code}</code></pre>
</div>

<script lang="ts">
	import type { ContextEntry as ContextEntryType } from "@orqastudio/sdk";
	import { Icon } from "@orqastudio/svelte-components/pure";
	import ContextDetailDialog from "./ContextDetailDialog.svelte";

	let { entry }: { entry: ContextEntryType } = $props();

	let dialogOpen = $state(false);

	const summaryText = $derived.by(() => {
		if (entry.type === "system_prompt_sent") {
			return `System prompt sent (${entry.totalChars.toLocaleString()} chars)`;
		}
		return `Context injected: ${entry.messageCount} ${entry.messageCount === 1 ? "message" : "messages"} (${entry.totalChars.toLocaleString()} chars)`;
	});
</script>

<button
	class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
	onclick={() => {
		dialogOpen = true;
	}}
>
	{#if entry.type === "system_prompt_sent"}
		<Icon name="eye" size="sm" />
	{:else}
		<Icon name="message-square" size="sm" />
	{/if}
	<span class="flex-1 text-xs text-muted-foreground">{summaryText}</span>
</button>

{#if dialogOpen}
	<ContextDetailDialog {entry} bind:open={dialogOpen} />
{/if}

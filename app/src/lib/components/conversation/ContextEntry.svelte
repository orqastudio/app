<script lang="ts">
	import type { ContextEntry as ContextEntryType } from "@orqastudio/sdk";
	import { Icon, Caption } from "@orqastudio/svelte-components/pure";
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
	class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left h-auto justify-start hover:bg-accent"
	onclick={() => {
		dialogOpen = true;
	}}
>
	{#if entry.type === "system_prompt_sent"}
		<Icon name="eye" size="sm" />
	{:else}
		<Icon name="message-square" size="sm" />
	{/if}
	<Caption>{summaryText}</Caption>
</button>

{#if dialogOpen}
	<ContextDetailDialog {entry} bind:open={dialogOpen} />
{/if}

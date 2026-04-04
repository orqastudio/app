<script lang="ts">
	import type { ContextEntry as ContextEntryType } from "@orqastudio/sdk";
	import { Icon, Caption, Button } from "@orqastudio/svelte-components/pure";
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

<Button
	variant="ghost"
	full
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
</Button>

{#if dialogOpen}
	<ContextDetailDialog {entry} bind:open={dialogOpen} />
{/if}

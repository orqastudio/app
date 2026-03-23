<script lang="ts">
	import { Icon,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "@orqastudio/svelte-components/pure";
	import ToolCallCard from "./ToolCallCard.svelte";
	import { getToolDisplay, groupLabel } from "$lib/utils/tool-display";

	interface ToolCallInfo {
		toolCallId: string;
		toolName: string;
		input: string | null;
		output: string | null;
		isError: boolean;
		isComplete: boolean;
	}

	let {
		toolName,
		toolCalls,
	}: {
		toolName: string;
		toolCalls: ToolCallInfo[];
	} = $props();

	let open = $state(false);

	const displayInfo = $derived(getToolDisplay(toolName));
	const label = $derived(groupLabel(toolName, toolCalls.length));
	const errorCount = $derived(toolCalls.filter((c) => c.isError).length);
</script>

<Collapsible bind:open>
	<CollapsibleTrigger
		class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
	>
		<Icon name="chevron-right" size="sm" />
		{@const ToolIcon = displayInfo.icon}
		<ToolIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
		<span class="flex-1 truncate font-mono text-xs">{label}</span>
		{#if errorCount > 0}
			<span class="flex items-center gap-1 text-xs text-destructive">
				<Icon name="x-circle" size="sm" />
				{errorCount}
				{errorCount === 1 ? "error" : "errors"}
			</span>
		{:else}
			<Icon name="check-circle" size="sm" />
		{/if}
	</CollapsibleTrigger>
	<CollapsibleContent>
		<div class="ml-3 mt-1 space-y-1 border-l-2 border-border pl-4">
			{#each toolCalls as toolCall (toolCall.toolCallId)}
				<ToolCallCard
					toolName={toolCall.toolName}
					toolInput={toolCall.input}
					toolOutput={toolCall.output}
					isError={toolCall.isError}
					isComplete={toolCall.isComplete}
				/>
			{/each}
		</div>
	</CollapsibleContent>
</Collapsible>

<script lang="ts">
	import BrainIcon from "@lucide/svelte/icons/brain";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import {
		Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "$lib/components/ui/collapsible";

	let { content, isStreaming = false }: { content: string; isStreaming?: boolean } = $props();

	let open = $state(true);

	// Auto-collapse when streaming ends
	$effect(() => {
		if (!isStreaming && content.length > 0) {
			open = false;
		}
	});
</script>

{#if content.length > 0}
	<Collapsible bind:open>
		<CollapsibleTrigger
			class="flex w-full items-center gap-2 rounded-lg border border-warning/30 bg-warning/5 px-3 py-2 text-left text-sm transition-colors hover:bg-warning/10"
		>
			<ChevronRightIcon
				class="h-3.5 w-3.5 shrink-0 text-warning transition-transform {open ? 'rotate-90' : ''}"
			/>
			<BrainIcon class="h-3.5 w-3.5 shrink-0 text-warning" />
			<span class="flex-1 text-xs text-warning">
				{isStreaming ? "Reasoning..." : "Reasoning"}
			</span>
			{#if isStreaming}
				<span class="h-2 w-2 animate-pulse rounded-full bg-warning"></span>
			{/if}
		</CollapsibleTrigger>
		<CollapsibleContent>
			<div class="mt-1 rounded-lg border border-warning/20 bg-warning/5 px-3 py-2">
				<pre class="whitespace-pre-wrap text-xs text-warning font-mono">{content}</pre>
			</div>
		</CollapsibleContent>
	</Collapsible>
{/if}

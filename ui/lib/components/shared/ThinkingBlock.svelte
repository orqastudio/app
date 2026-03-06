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
			class="flex w-full items-center gap-2 rounded-lg border border-amber-500/30 bg-amber-500/5 px-3 py-2 text-left text-sm transition-colors hover:bg-amber-500/10"
		>
			<ChevronRightIcon
				class="h-3.5 w-3.5 shrink-0 text-amber-600 dark:text-amber-400 transition-transform {open ? 'rotate-90' : ''}"
			/>
			<BrainIcon class="h-3.5 w-3.5 shrink-0 text-amber-600 dark:text-amber-400" />
			<span class="flex-1 text-xs text-amber-700 dark:text-amber-300">
				{isStreaming ? "Reasoning..." : "Reasoning"}
			</span>
			{#if isStreaming}
				<span class="h-2 w-2 animate-pulse rounded-full bg-amber-500"></span>
			{/if}
		</CollapsibleTrigger>
		<CollapsibleContent>
			<div class="mt-1 rounded-lg border border-amber-500/20 bg-amber-500/5 px-3 py-2">
				<pre class="whitespace-pre-wrap text-xs text-amber-800 dark:text-amber-200 font-mono">{content}</pre>
			</div>
		</CollapsibleContent>
	</Collapsible>
{/if}

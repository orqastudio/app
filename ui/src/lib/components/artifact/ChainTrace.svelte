<script lang="ts">
	import {
		Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "$lib/components/ui/collapsible";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import ArrowRightIcon from "@lucide/svelte/icons/arrow-right";
	import { getGraphViz } from "$lib/graph-viz.svelte";
	import ArtifactLink from "./ArtifactLink.svelte";

	const graphViz = getGraphViz();

	let { artifactId }: { artifactId: string } = $props();

	let open = $state(false);

	const upChain = $derived(graphViz.traceChain(artifactId, "up"));
	const downChain = $derived(graphViz.traceChain(artifactId, "down"));

	const hasChain = $derived(upChain.length > 0 || downChain.length > 0);
</script>

{#if hasChain}
	<div class="border-b border-border px-4 py-2">
		<Collapsible bind:open>
			<CollapsibleTrigger
				class="flex items-center gap-1 text-xs font-medium text-muted-foreground hover:text-foreground transition-colors"
			>
				<ChevronRightIcon
					class="h-3 w-3 transition-transform {open ? 'rotate-90' : ''}"
				/>
				Trace
			</CollapsibleTrigger>
			<CollapsibleContent>
				<div class="space-y-3 pt-2 pl-4">
					{#if upChain.length > 0}
						<div class="space-y-1">
							<span class="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
								Why does this exist?
							</span>
							<div class="flex flex-wrap items-center gap-1">
								<!-- Current artifact is the start of the upward chain -->
								<span
									class="inline-flex items-center gap-1 whitespace-nowrap rounded border border-primary/40 bg-primary/10 px-1.5 py-0.5 font-mono text-[11px] font-semibold text-primary"
								>
									{artifactId}
								</span>
								{#each upChain as id (id)}
									<ArrowRightIcon class="h-3 w-3 shrink-0 text-muted-foreground" />
									<ArtifactLink {id} />
								{/each}
							</div>
						</div>
					{/if}

					{#if downChain.length > 0}
						<div class="space-y-1">
							<span class="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
								What does this affect?
							</span>
							<div class="flex flex-wrap items-center gap-1">
								<!-- Current artifact is the start of the downward chain -->
								<span
									class="inline-flex items-center gap-1 whitespace-nowrap rounded border border-primary/40 bg-primary/10 px-1.5 py-0.5 font-mono text-[11px] font-semibold text-primary"
								>
									{artifactId}
								</span>
								{#each downChain as id (id)}
									<ArrowRightIcon class="h-3 w-3 shrink-0 text-muted-foreground" />
									<ArtifactLink {id} />
								{/each}
							</div>
						</div>
					{/if}
				</div>
			</CollapsibleContent>
		</Collapsible>
	</div>
{/if}

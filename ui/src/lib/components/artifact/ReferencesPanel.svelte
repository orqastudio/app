<script lang="ts">
	import {
		Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "$lib/components/ui/collapsible";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import ArtifactLink from "./ArtifactLink.svelte";
	import type { ArtifactRef } from "$lib/types/artifact-graph";

	let { artifactPath }: { artifactPath: string } = $props();

	const artifactId = $derived.by(() => {
		const filename = artifactPath.split("/").pop() ?? "";
		const dotIndex = filename.lastIndexOf(".");
		return dotIndex !== -1 ? filename.slice(0, dotIndex) : filename;
	});

	const incomingRefs = $derived<ArtifactRef[]>(
		artifactId ? artifactGraphSDK.referencesTo(artifactId) : [],
	);

	const outgoingRefs = $derived<ArtifactRef[]>(
		artifactId ? artifactGraphSDK.referencesFrom(artifactId) : [],
	);

	const totalRefs = $derived(incomingRefs.length + outgoingRefs.length);

	let panelOpen = $state(false);
</script>

{#if totalRefs > 0}
	<div class="border-b border-border px-4 py-2">
		<Collapsible bind:open={panelOpen}>
			<CollapsibleTrigger
				class="flex items-center gap-1 text-xs font-medium text-muted-foreground hover:text-foreground transition-colors"
			>
				<ChevronRightIcon
					class="h-3 w-3 transition-transform {panelOpen ? 'rotate-90' : ''}"
				/>
				Relationships
				<span class="text-[10px] tabular-nums">({totalRefs})</span>
			</CollapsibleTrigger>
			<CollapsibleContent>
				<div class="space-y-2 pt-1.5 pl-4">
					{#if incomingRefs.length > 0}
						<div>
							<span class="text-[10px] font-medium uppercase tracking-wider text-muted-foreground">Referenced by</span>
							<div class="flex flex-wrap gap-1 pt-0.5">
								{#each incomingRefs as ref (ref.source_id + ref.field)}
									<ArtifactLink id={ref.source_id} />
								{/each}
							</div>
						</div>
					{/if}

					{#if outgoingRefs.length > 0}
						<div>
							<span class="text-[10px] font-medium uppercase tracking-wider text-muted-foreground">References</span>
							<div class="flex flex-wrap gap-1 pt-0.5">
								{#each outgoingRefs as ref (ref.target_id + ref.field)}
									<ArtifactLink id={ref.target_id} />
								{/each}
							</div>
						</div>
					{/if}
				</div>
			</CollapsibleContent>
		</Collapsible>
	</div>
{/if}

<script lang="ts">
	import {
		Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "$lib/components/ui/collapsible";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import { SvelteMap } from "svelte/reactivity";
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

	/** Humanize a relationship type or field name. */
	function humanizeLabel(value: string): string {
		return value
			.replace(/-/g, " ")
			.replace(/_/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/** Group refs by relationship_type (or field as fallback). */
	function groupRefs(refs: ArtifactRef[]): SvelteMap<string, ArtifactRef[]> {
		const groups = new SvelteMap<string, ArtifactRef[]>();
		for (const ref of refs) {
			const key = ref.relationship_type ?? ref.field;
			const existing = groups.get(key);
			if (existing) {
				existing.push(ref);
			} else {
				groups.set(key, [ref]);
			}
		}
		return groups;
	}

	const incomingGrouped = $derived(groupRefs(incomingRefs));
	const outgoingGrouped = $derived(groupRefs(outgoingRefs));

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
						<div class="space-y-1.5">
							<span class="text-[10px] font-medium uppercase tracking-wider text-muted-foreground">Referenced by</span>
							{#each [...incomingGrouped] as [groupKey, refs] (groupKey)}
								<div class="flex items-baseline gap-2">
									<span class="shrink-0 rounded border border-muted-foreground/20 bg-muted px-1.5 py-0.5 text-[10px] font-medium capitalize text-muted-foreground">
										{humanizeLabel(groupKey)}
									</span>
									<div class="flex flex-wrap gap-1">
										{#each refs as ref ("in:" + ref.source_id + ref.field)}
											<ArtifactLink id={ref.source_id} />
										{/each}
									</div>
								</div>
							{/each}
						</div>
					{/if}

					{#if outgoingRefs.length > 0}
						<div class="space-y-1.5">
							<span class="text-[10px] font-medium uppercase tracking-wider text-muted-foreground">References</span>
							{#each [...outgoingGrouped] as [groupKey, refs] (groupKey)}
								<div class="flex items-baseline gap-2">
									<span class="shrink-0 rounded border border-muted-foreground/20 bg-muted px-1.5 py-0.5 text-[10px] font-medium capitalize text-muted-foreground">
										{humanizeLabel(groupKey)}
									</span>
									<div class="flex flex-wrap gap-1">
										{#each refs as ref ("out:" + ref.target_id + ref.field)}
											<ArtifactLink id={ref.target_id} />
										{/each}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</CollapsibleContent>
		</Collapsible>
	</div>
{/if}

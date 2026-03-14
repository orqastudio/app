<script lang="ts">
	import {
		Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "$lib/components/ui/collapsible";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import ListIcon from "@lucide/svelte/icons/list";
	import NetworkIcon from "@lucide/svelte/icons/network";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import { SvelteMap } from "svelte/reactivity";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { statusColor } from "$lib/components/shared/StatusIndicator.svelte";
	import ArtifactLink from "./ArtifactLink.svelte";
	import RelationshipGraphView from "./RelationshipGraphView.svelte";
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

	/** Toggle between list and graph view. */
	let viewMode = $state<"list" | "graph">("list");

	/** Per-group expanded state for overflow toggle. */
	const expandedGroups = new SvelteMap<string, boolean>();

	function isExpanded(key: string): boolean {
		return expandedGroups.get(key) ?? false;
	}

	function toggleExpanded(key: string): void {
		expandedGroups.set(key, !isExpanded(key));
	}

	/** Resolve status dot color class for an artifact ID. */
	function getStatusDotClass(targetId: string): string | null {
		const node = artifactGraphSDK.resolve(targetId);
		if (!node?.status) return null;
		return statusColor(node.status);
	}

	/** Get visible refs for a group (respecting overflow toggle). */
	function visibleRefs(groupKey: string, direction: string, refs: ArtifactRef[]): ArtifactRef[] {
		const key = `${direction}:${groupKey}`;
		if (refs.length <= 3 || isExpanded(key)) return refs;
		return refs.slice(0, 3);
	}
</script>

{#if totalRefs > 0}
	<div class="border-b border-border px-4 py-2">
		<Collapsible bind:open={panelOpen}>
			<div class="flex items-center justify-between">
				<CollapsibleTrigger
					class="flex items-center gap-1 text-xs font-medium text-muted-foreground hover:text-foreground transition-colors"
				>
					<ChevronRightIcon
						class="h-3 w-3 transition-transform {panelOpen ? 'rotate-90' : ''}"
					/>
					Relationships
					<span class="text-[10px] tabular-nums">({totalRefs})</span>
				</CollapsibleTrigger>
				{#if panelOpen}
					<Tooltip.Root>
						<Tooltip.Trigger>
							{#snippet child({ props })}
								<button
									{...props}
									class="rounded p-0.5 text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
									onclick={() => { viewMode = viewMode === "list" ? "graph" : "list"; }}
								>
									{#if viewMode === "list"}
										<NetworkIcon class="h-3.5 w-3.5" />
									{:else}
										<ListIcon class="h-3.5 w-3.5" />
									{/if}
								</button>
							{/snippet}
						</Tooltip.Trigger>
						<Tooltip.Content side="top">
							<p>{viewMode === "list" ? "Graph view" : "List view"}</p>
						</Tooltip.Content>
					</Tooltip.Root>
				{/if}
			</div>
			<CollapsibleContent>
				{#if viewMode === "graph"}
					<div class="pt-1.5">
						<RelationshipGraphView
							{artifactId}
							{incomingRefs}
							{outgoingRefs}
						/>
					</div>
				{:else}
				<div class="space-y-2 pt-1.5 pl-4">
					{#if incomingRefs.length > 0}
						<div class="space-y-1.5">
							<span class="text-[10px] font-medium uppercase tracking-wider text-muted-foreground">Referenced by</span>
							{#each [...incomingGrouped] as [groupKey, refs] (groupKey)}
								{@const dirKey = `in:${groupKey}`}
								<div class="grid grid-cols-2 items-baseline gap-2">
									<span class="justify-self-start rounded border border-muted-foreground/20 bg-muted px-1.5 py-0.5 text-[10px] font-medium capitalize text-muted-foreground">
										{humanizeLabel(groupKey)}
									</span>
									<div class="flex min-w-0 flex-wrap items-center gap-1">
										{#each visibleRefs(groupKey, "in", refs) as ref ("in:" + ref.source_id + ref.field)}
											{@const dotClass = getStatusDotClass(ref.source_id)}
											<span class="inline-flex items-center gap-1">
												{#if dotClass}
													<span class="inline-block h-1.5 w-1.5 shrink-0 rounded-full {dotClass}"></span>
												{/if}
												<ArtifactLink id={ref.source_id} />
											</span>
										{/each}
										{#if refs.length > 3}
											<button
												class="rounded px-1 py-0.5 text-[10px] font-medium text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
												onclick={() => toggleExpanded(dirKey)}
											>
												{isExpanded(dirKey) ? "hide" : `\u2026 +${refs.length - 3}`}
											</button>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					{/if}

					{#if outgoingRefs.length > 0}
						<div class="space-y-1.5">
							<span class="text-[10px] font-medium uppercase tracking-wider text-muted-foreground">References</span>
							{#each [...outgoingGrouped] as [groupKey, refs] (groupKey)}
								{@const dirKey = `out:${groupKey}`}
								<div class="grid grid-cols-2 items-baseline gap-2">
									<span class="justify-self-start rounded border border-muted-foreground/20 bg-muted px-1.5 py-0.5 text-[10px] font-medium capitalize text-muted-foreground">
										{humanizeLabel(groupKey)}
									</span>
									<div class="flex min-w-0 flex-wrap items-center gap-1">
										{#each visibleRefs(groupKey, "out", refs) as ref ("out:" + ref.target_id + ref.field)}
											{@const dotClass = getStatusDotClass(ref.target_id)}
											<span class="inline-flex items-center gap-1">
												{#if dotClass}
													<span class="inline-block h-1.5 w-1.5 shrink-0 rounded-full {dotClass}"></span>
												{/if}
												<ArtifactLink id={ref.target_id} />
											</span>
										{/each}
										{#if refs.length > 3}
											<button
												class="rounded px-1 py-0.5 text-[10px] font-medium text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
												onclick={() => toggleExpanded(dirKey)}
											>
												{isExpanded(dirKey) ? "hide" : `\u2026 +${refs.length - 3}`}
											</button>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
				{/if}
			</CollapsibleContent>
		</Collapsible>
	</div>
{/if}

<script lang="ts">
	import { Icon,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "@orqastudio/svelte-components/pure";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import { SvelteMap } from "svelte/reactivity";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK } = getStores();
	import ArtifactLink from "./ArtifactLink.svelte";
	import RelationshipGraphView from "./RelationshipGraphView.svelte";
	import type { ArtifactRef } from "@orqastudio/types";

	let { artifactPath }: { artifactPath: string } = $props();

	let panelOpen = $state(false);

	const artifactId = $derived.by(() => {
		const filename = artifactPath.split("/").pop() ?? "";
		const dotIndex = filename.lastIndexOf(".");
		return dotIndex !== -1 ? filename.slice(0, dotIndex) : filename;
	});

	// Ref counts — should be cheap graph lookups
	const incomingCount = $derived.by(() => {
		if (!artifactId) return 0;
		const count = artifactGraphSDK.referencesTo(artifactId).length;
		return count;
	});
	const outgoingCount = $derived.by(() => {
		if (!artifactId) return 0;
		const count = artifactGraphSDK.referencesFrom(artifactId).length;
		return count;
	});
	const totalRefs = $derived(incomingCount + outgoingCount);

	// Full refs only computed when the panel is open — deferred to avoid
	// expensive graph traversal + rendering on every artifact load
	const incomingRefs = $derived<ArtifactRef[]>(
		panelOpen && artifactId ? artifactGraphSDK.referencesTo(artifactId) : [],
	);

	const outgoingRefs = $derived<ArtifactRef[]>(
		panelOpen && artifactId ? artifactGraphSDK.referencesFrom(artifactId) : [],
	);

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
					<Icon name="chevron-right" size="xs" />
					Relationships

				</CollapsibleTrigger>
				{#if panelOpen}
					<TooltipRoot>
						<TooltipTrigger>
							{#snippet child({ props })}
								<button
									{...props}
									class="rounded p-0.5 text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
									onclick={() => { viewMode = viewMode === "list" ? "graph" : "list"; }}
								>
									{#if viewMode === "list"}
										<Icon name="network" size="sm" />
									{:else}
										<Icon name="list" size="sm" />
									{/if}
								</button>
							{/snippet}
						</TooltipTrigger>
						<TooltipContent side="top">
							<p>{viewMode === "list" ? "Graph view" : "List view"}</p>
						</TooltipContent>
					</TooltipRoot>
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

							{#each [...incomingGrouped] as [groupKey, refs] (groupKey)}
								{@const dirKey = `in:${groupKey}`}
								<div class="grid grid-cols-2 items-baseline gap-2">
									<span class="justify-self-start rounded border border-muted-foreground/20 bg-muted px-1.5 py-0.5 text-[10px] font-medium capitalize text-muted-foreground">
										{humanizeLabel(groupKey)}
									</span>

									<div class="flex min-w-0 flex-wrap items-center gap-1">
										{#each visibleRefs(groupKey, "in", refs) as ref, i ("in:" + ref.source_id + ref.relationship_type + i)}
											<ArtifactLink id={ref.source_id} />

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

							{#each [...outgoingGrouped] as [groupKey, refs] (groupKey)}
								{@const dirKey = `out:${groupKey}`}
								<div class="grid grid-cols-2 items-baseline gap-2">
									<span class="justify-self-start rounded border border-muted-foreground/20 bg-muted px-1.5 py-0.5 text-[10px] font-medium capitalize text-muted-foreground">
										{humanizeLabel(groupKey)}
									</span>

									<div class="flex min-w-0 flex-wrap items-center gap-1">
										{#each visibleRefs(groupKey, "out", refs) as ref, i ("out:" + ref.target_id + ref.relationship_type + i)}
											<ArtifactLink id={ref.target_id} />
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

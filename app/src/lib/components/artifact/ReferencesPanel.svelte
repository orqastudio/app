<!-- Collapsible panel showing incoming and outgoing references for an artifact. Supports list and graph views. -->
<script lang="ts">
	import {
		Icon,
		HStack,
		Stack,
		Box,
		Caption,
		Button,
		Badge,
		Panel,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "@orqastudio/svelte-components/pure";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import { SvelteMap } from "svelte/reactivity";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK } = getStores();
	import { ArtifactLink } from "@orqastudio/svelte-components/connected";
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
	const incomingRefs = $derived<readonly ArtifactRef[]>(
		panelOpen && artifactId ? artifactGraphSDK.referencesTo(artifactId) : [],
	);

	const outgoingRefs = $derived<readonly ArtifactRef[]>(
		panelOpen && artifactId ? artifactGraphSDK.referencesFrom(artifactId) : [],
	);

	/**
	 * Humanize a relationship type or field name.
	 * @param value - The raw relationship type or field name string.
	 * @returns A title-cased human-readable label.
	 */
	function humanizeLabel(value: string): string {
		return value
			.replace(/-/g, " ")
			.replace(/_/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/**
	 * Group refs by relationship_type (or field as fallback).
	 * @param refs - The list of artifact references to group.
	 * @returns A SvelteMap keyed by relationship type with arrays of refs.
	 */
	function groupRefs(refs: readonly ArtifactRef[]): SvelteMap<string, ArtifactRef[]> {
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

	/**
	 * Return whether the overflow group identified by key is expanded.
	 * @param key - The composite group key (e.g. "out:delivers") used to look up expansion state.
	 * @returns True if the overflow group is expanded to show all items.
	 */
	function isExpanded(key: string): boolean {
		return expandedGroups.get(key) ?? false;
	}

	/**
	 * Toggle the expanded state for the overflow group identified by key.
	 * @param key - The composite group key (e.g. "out:delivers") whose expansion state to toggle.
	 */
	function toggleExpanded(key: string): void {
		expandedGroups.set(key, !isExpanded(key));
	}

	/**
	 * Get visible refs for a group (respecting overflow toggle).
	 * @param groupKey - The relationship type key for this group.
	 * @param direction - Direction prefix ("in" or "out") used to build the composite expansion key.
	 * @param refs - The full list of refs for this group.
	 * @returns The slice of refs to display (up to 3 unless expanded).
	 */
	function visibleRefs(groupKey: string, direction: string, refs: ArtifactRef[]): ArtifactRef[] {
		const key = `${direction}:${groupKey}`;
		if (refs.length <= 3 || isExpanded(key)) return refs;
		return refs.slice(0, 3);
	}
</script>

{#if totalRefs > 0}
	<Panel padding="tight" border="bottom">
		<Collapsible bind:open={panelOpen}>
			<HStack justify="between">
				<!-- CollapsibleTrigger is a library component (Bits UI primitive wrapper) that
				     forwards class= via ...restProps to the underlying trigger element. Passing
				     class= to a library component is the standard Tailwind composition pattern
				     for headless UI primitives — this is NOT a raw HTML violation. -->
				<CollapsibleTrigger
					class="text-muted-foreground hover:text-foreground text-xs font-medium transition-colors"
				>
					<HStack gap={1}>
						<Icon name="chevron-right" size="sm" />
						Relationships
					</HStack>
				</CollapsibleTrigger>
				{#if panelOpen}
					<TooltipRoot>
						<TooltipTrigger>
							{#snippet child({ props })}
								<Button
									{...props}
									variant="ghost"
									size="icon-sm"
									onclick={() => {
										viewMode = viewMode === "list" ? "graph" : "list";
									}}
								>
									{#if viewMode === "list"}
										<Icon name="network" size="sm" />
									{:else}
										<Icon name="list" size="sm" />
									{/if}
								</Button>
							{/snippet}
						</TooltipTrigger>
						<TooltipContent side="top">
							<Caption>{viewMode === "list" ? "Graph view" : "List view"}</Caption>
						</TooltipContent>
					</TooltipRoot>
				{/if}
			</HStack>
			<CollapsibleContent>
				<Stack gap={1}>
					{#if viewMode === "graph"}
						<RelationshipGraphView {artifactId} {incomingRefs} {outgoingRefs} />
					{:else}
						<Panel padding="normal">
							<Stack gap={2}>
								{#if incomingRefs.length > 0}
									<Stack gap={1}>
										{#each [...incomingGrouped] as [groupKey, refs] (groupKey)}
											{@const dirKey = `in:${groupKey}`}
											<HStack align="baseline" gap={2}>
												<Badge variant="secondary" size="xs" capitalize>
													{humanizeLabel(groupKey)}
												</Badge>

												<Box flex={1} minWidth={0}
													><HStack wrap gap={1}>
														{#each visibleRefs(groupKey, "in", refs) as ref, i ("in:" + ref.source_id + ref.relationship_type + i)}
															<ArtifactLink id={ref.source_id} />
														{/each}
														{#if refs.length > 3}
															<Button
																variant="ghost"
																size="sm"
																onclick={() => toggleExpanded(dirKey)}
															>
																{isExpanded(dirKey) ? "hide" : `\u2026 +${refs.length - 3}`}
															</Button>
														{/if}
													</HStack></Box
												>
											</HStack>
										{/each}
									</Stack>
								{/if}

								{#if outgoingRefs.length > 0}
									<Stack gap={1}>
										{#each [...outgoingGrouped] as [groupKey, refs] (groupKey)}
											{@const dirKey = `out:${groupKey}`}
											<HStack align="baseline" gap={2}>
												<Badge variant="secondary" size="xs" capitalize>
													{humanizeLabel(groupKey)}
												</Badge>

												<Box flex={1} minWidth={0}
													><HStack wrap gap={1}>
														{#each visibleRefs(groupKey, "out", refs) as ref, i ("out:" + ref.target_id + ref.relationship_type + i)}
															<ArtifactLink id={ref.target_id} />
														{/each}
														{#if refs.length > 3}
															<Button
																variant="ghost"
																size="sm"
																onclick={() => toggleExpanded(dirKey)}
															>
																{isExpanded(dirKey) ? "hide" : `\u2026 +${refs.length - 3}`}
															</Button>
														{/if}
													</HStack></Box
												>
											</HStack>
										{/each}
									</Stack>
								{/if}
							</Stack>
						</Panel>
					{/if}
				</Stack>
			</CollapsibleContent>
		</Collapsible>
	</Panel>
{/if}

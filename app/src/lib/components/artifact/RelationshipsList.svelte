<!-- Renders grouped relationships for an artifact: type badge on the left, artifact links on the right. Supports overflow toggling per group. -->
<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
	import {
		Icon,
		Stack,
		HStack,
		Box,
		Text,
		Badge,
		Button,
	} from "@orqastudio/svelte-components/pure";
	import { ArtifactLink } from "@orqastudio/svelte-components/connected";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, projectStore } = getStores();

	interface Relationship {
		type: string;
		target: string | null;
		rationale: string;
		intended?: boolean;
	}

	let { relationships }: { relationships: Relationship[] } = $props();

	/** Per-type expanded state for overflow toggle. */
	const expandedTypes = new SvelteMap<string, boolean>();

	/**
	 * Humanize a relationship type for display (e.g. "grounded-by" → "Grounded By").
	 * @param type - The raw relationship type string.
	 * @returns A title-cased human-readable label.
	 */
	function humanizeType(type: string): string {
		return type.replace(/-/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
	}

	/** Group relationships by type for compact display. */
	const grouped = $derived.by(() => {
		const groups = new SvelteMap<string, Relationship[]>();
		for (const rel of relationships) {
			const existing = groups.get(rel.type);
			if (existing) {
				existing.push(rel);
			} else {
				groups.set(rel.type, [rel]);
			}
		}
		return groups;
	});

	/**
	 * Return whether the overflow list for the given relationship type is expanded.
	 * @param type - The relationship type key (e.g. "delivers") to check expansion state for.
	 * @returns True if the overflow list for the given type is expanded.
	 */
	function isExpanded(type: string): boolean {
		return expandedTypes.get(type) ?? false;
	}

	/**
	 * Toggle the expanded state for the overflow list of the given relationship type.
	 * @param type - The relationship type key whose overflow expansion state to toggle.
	 */
	function toggleExpanded(type: string): void {
		expandedTypes.set(type, !isExpanded(type));
	}

	/**
	 * Get visible items for a type (respecting overflow toggle).
	 * @param type - The relationship type key used to look up expanded state.
	 * @param rels - The full list of relationships for this type.
	 * @returns The slice of relationships to display (up to 3 unless expanded).
	 */
	function visibleRels(type: string, rels: Relationship[]): Relationship[] {
		if (rels.length <= 3 || isExpanded(type)) return rels;
		return rels.slice(0, 3);
	}

	/**
	 * Resolve the display label for a relationship chip based on project config.
	 * Uses relationshipDisplay.defaultField ("title" or "id"), with per-type overrides.
	 * Falls back to ID if title is empty or config says "id".
	 * @param targetId - The artifact ID whose display label to resolve.
	 * @returns The display label (title or ID) for the chip.
	 */
	function chipLabel(targetId: string): string {
		const config = projectStore.projectSettings?.relationshipDisplay;
		const node = artifactGraphSDK.resolve(targetId);
		const defaultField = config?.defaultField ?? "title";
		const artifactType = node?.artifact_type ?? "";
		const displayField = config?.overrides[artifactType] ?? defaultField;

		if (displayField === "title" && node?.title) {
			return node.title;
		}
		return targetId;
	}
</script>

{#if relationships.length > 0}
	<Stack gap={1.5}>
		<Text variant="caption">Relationships</Text>
		<Stack gap={1}>
			{#each [...grouped] as [type, rels] (type)}
				<HStack gap={2} align="baseline">
					<span class="justify-self-start capitalize"
						><Badge variant="outline" size="sm">{humanizeType(type)}</Badge></span
					>
					<Box flex={1} minWidth={0}
						><HStack wrap gap={1}>
							{#each visibleRels(type, rels) as rel, i (i)}
								{#if rel.target}
									{@const label = chipLabel(rel.target ?? "")}
									{#if label !== (rel.target ?? "")}
										<ArtifactLink id={rel.target ?? undefined} displayLabel={label} />
									{:else}
										<ArtifactLink id={rel.target ?? undefined} />
									{/if}
								{:else}
									<TooltipRoot>
										<TooltipTrigger>
											{#snippet child({ props })}
												<span
													{...props}
													class="inline-flex items-center gap-1 rounded border px-1.5 py-0.5 font-mono text-[11px] font-medium {rel.intended
														? 'border-muted-foreground/30 bg-muted text-muted-foreground'
														: 'border-warning/30 bg-warning/10 text-warning'}"
												>
													<Icon name="circle-alert" size="sm" />
													{rel.intended ? "intentional gap" : "unresolved"}
												</span>
											{/snippet}
										</TooltipTrigger>
										<TooltipContent side="top">
											<Text variant="caption">{rel.rationale}</Text>
										</TooltipContent>
									</TooltipRoot>
								{/if}
							{/each}
							{#if rels.length > 3}
								<Button variant="ghost" size="sm" onclick={() => toggleExpanded(type)}>
									{isExpanded(type) ? "hide" : `\u2026 +${rels.length - 3}`}
								</Button>
							{/if}
						</HStack></Box
					>
				</HStack>
			{/each}
		</Stack>
	</Stack>
{/if}

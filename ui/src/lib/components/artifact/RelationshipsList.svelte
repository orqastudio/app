<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
	import ArtifactLink from "./ArtifactLink.svelte";
	import { Badge } from "$lib/components/ui/badge";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { projectStore } from "$lib/stores/project.svelte";

	interface Relationship {
		type: string;
		target: string | null;
		rationale: string;
		intended?: boolean;
	}

	let { relationships }: { relationships: Relationship[] } = $props();

	/** Per-type expanded state for overflow toggle. */
	const expandedTypes = new SvelteMap<string, boolean>();

	/** Humanize a relationship type for display (e.g. "grounded-by" → "Grounded By"). */
	function humanizeType(type: string): string {
		return type
			.replace(/-/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
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

	function isExpanded(type: string): boolean {
		return expandedTypes.get(type) ?? false;
	}

	function toggleExpanded(type: string): void {
		expandedTypes.set(type, !isExpanded(type));
	}

	/** Get visible items for a type (respecting overflow toggle). */
	function visibleRels(type: string, rels: Relationship[]): Relationship[] {
		if (rels.length <= 3 || isExpanded(type)) return rels;
		return rels.slice(0, 3);
	}

	/**
	 * Resolve the display label for a relationship chip based on project config.
	 * Uses relationshipDisplay.defaultField ("title" or "id"), with per-type overrides.
	 * Falls back to ID if title is empty or config says "id".
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
	<div class="space-y-1.5">
		<span class="text-xs font-medium text-muted-foreground">Relationships</span>
		<div class="space-y-1">
			{#each [...grouped] as [type, rels] (type)}
				<div class="grid grid-cols-2 items-baseline gap-2">
					<Badge variant="outline" class="justify-self-start text-[10px] font-medium capitalize">
						{humanizeType(type)}
					</Badge>
					<div class="flex min-w-0 flex-wrap items-center gap-1">
						{#each visibleRels(type, rels) as rel, i (i)}
							{#if rel.target}
								{@const label = chipLabel(rel.target ?? "")}
								{#if label !== (rel.target ?? "")}
									<ArtifactLink id={rel.target ?? undefined} displayLabel={label} />
								{:else}
									<ArtifactLink id={rel.target ?? undefined} />
								{/if}
							{:else}
								<Tooltip.Root>
									<Tooltip.Trigger>
										{#snippet child({ props })}
											<span
												{...props}
												class="inline-flex items-center gap-1 rounded border px-1.5 py-0.5 font-mono text-[11px] font-medium {rel.intended
													? 'border-muted-foreground/30 bg-muted text-muted-foreground'
													: 'border-warning/30 bg-warning/10 text-warning'}"
											>
												<CircleAlertIcon class="h-3 w-3 shrink-0" />
												{rel.intended ? "intentional gap" : "unresolved"}
											</span>
										{/snippet}
									</Tooltip.Trigger>
									<Tooltip.Content side="top" class="max-w-xs">
										<p class="text-xs">{rel.rationale}</p>
									</Tooltip.Content>
								</Tooltip.Root>
							{/if}
						{/each}
						{#if rels.length > 3}
							<button
								class="rounded px-1 py-0.5 text-[10px] font-medium text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
								onclick={() => toggleExpanded(type)}
							>
								{isExpanded(type) ? "hide" : `\u2026 +${rels.length - 3}`}
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>
{/if}

<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
	import ArtifactLink from "./ArtifactLink.svelte";
	import { Badge } from "$lib/components/ui/badge";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";

	interface Relationship {
		type: string;
		target: string | null;
		rationale: string;
		intended?: boolean;
	}

	let { relationships }: { relationships: Relationship[] } = $props();

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
</script>

{#if relationships.length > 0}
	<div class="space-y-1.5">
		<span class="text-xs font-medium text-muted-foreground">Relationships</span>
		<div class="space-y-1">
			{#each [...grouped] as [type, rels] (type)}
				<div class="flex items-baseline gap-2">
					<Badge variant="outline" class="shrink-0 text-[10px] font-medium capitalize">
						{humanizeType(type)}
					</Badge>
					<div class="flex min-w-0 flex-1 flex-wrap items-center gap-1">
						{#each rels as rel, i (i)}
							{#if rel.target}
								<Tooltip.Root>
									<Tooltip.Trigger>
										{#snippet child({ props })}
											<span {...props}>
												<ArtifactLink id={rel.target ?? undefined} />
											</span>
										{/snippet}
									</Tooltip.Trigger>
									<Tooltip.Content side="top" class="max-w-xs">
										<p class="text-xs">{rel.rationale}</p>
									</Tooltip.Content>
								</Tooltip.Root>
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
					</div>
				</div>
			{/each}
		</div>
	</div>
{/if}

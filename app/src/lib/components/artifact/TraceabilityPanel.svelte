<script lang="ts">
	import { SvelteSet } from "svelte/reactivity";
	import {
		Icon,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
		Badge,
	} from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import ArtifactLink from "./ArtifactLink.svelte";
	import type { TraceabilityResult, AncestryChain } from "@orqastudio/types";
	import { iconForArtifactType } from "$lib/config/relationship-icons";

	interface Props {
		result: TraceabilityResult | null;
		loading: boolean;
		error: string | null;
	}

	let { result, loading, error }: Props = $props();

	let open = $state(false);
	let descendantsOpen = $state(false);
	let siblingsOpen = $state(false);

	/**
	 * Whether the panel has any content to show (after a successful load).
	 */
	const hasContent = $derived(
		result !== null && (
			result.ancestry_chains.length > 0 ||
			result.descendants.length > 0 ||
			result.siblings.length > 0
		)
	);

	/**
	 * Deduplicate ancestry chains: if two chains share the same sequence of IDs,
	 * only show the first.
	 */
	const uniqueChains = $derived.by((): AncestryChain[] => {
		if (!result) return [];
		const seen = new SvelteSet<string>();
		return result.ancestry_chains.filter((chain) => {
			const key = chain.path.map((n) => n.id).join(",");
			if (seen.has(key)) return false;
			seen.add(key);
			return true;
		});
	});

	/** Descendants capped at 20 for the initial render. */
	const visibleDescendants = $derived(
		result ? result.descendants.slice(0, 20) : []
	);
</script>

{#if loading}
	<div class="border-b border-border px-4 py-2">
		<div class="flex items-center gap-2 text-xs text-muted-foreground">
			<LoadingSpinner size="sm" />
			<span>Loading traceability…</span>
		</div>
	</div>
{:else if error}
	<div class="border-b border-border px-4 py-2">
		<p class="text-xs text-destructive">{error}</p>
	</div>
{:else if result}
	<div class="border-b border-border px-4 py-2">
		<Collapsible bind:open>
			<CollapsibleTrigger
				class="flex w-full items-center gap-1 text-xs font-medium text-muted-foreground hover:text-foreground transition-colors"
			>
				<span class={open ? "rotate-90 transition-transform" : "transition-transform"}>
					<Icon name="chevron-right" size="xs" />
				</span>
				<span>Traceability</span>
				{#if result.disconnected}
					<Badge variant="destructive" class="ml-1 px-1 py-0 text-[9px]">disconnected</Badge>
				{:else if result.impact_radius > 0}
					<span class="ml-auto text-[10px] text-muted-foreground">
						{result.impact_radius} affected
					</span>
				{/if}
			</CollapsibleTrigger>

			<CollapsibleContent>
				<div class="space-y-3 pt-2 pl-4">

					<!-- Disconnected warning -->
					{#if result.disconnected}
						<div class="flex items-start gap-2 rounded border border-warning/30 bg-warning/10 px-2 py-1.5">
							<span class="mt-0.5 shrink-0 text-warning">
								<Icon name="triangle-alert" size="xs" />
							</span>
							<p class="text-[11px] text-warning leading-snug">
								This artifact has no path to any pillar. It is disconnected from the vision hierarchy.
							</p>
						</div>
					{/if}

					<!-- Ancestry chains -->
					{#if uniqueChains.length > 0}
						<div class="space-y-2">
							<span class="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
								Why does this exist?
							</span>
							{#each uniqueChains as chain, chainIdx (chainIdx)}
								<div class="space-y-0.5">
									{#each chain.path as node, nodeIdx (node.id + nodeIdx)}
										<div class="flex items-center gap-1.5">
											<!-- Indent guide line -->
											{#if nodeIdx > 0}
												<div
													class="ml-1 h-full w-px shrink-0 self-stretch bg-border"
													style="margin-left: {(nodeIdx - 1) * 12}px"
												></div>
												<span
													class="shrink-0 text-muted-foreground/50"
													style="margin-left: {(nodeIdx - 1) * 12}px"
												>
													<Icon name="corner-down-right" size="xs" />
												</span>
											{/if}

											<!-- Artifact chip -->
											<div
												class="flex items-center gap-1"
												style={nodeIdx === 0 ? "" : `margin-left: ${nodeIdx * 8}px`}
											>
												<span class="shrink-0 text-muted-foreground">
													<Icon name={iconForArtifactType(node.artifact_type)} size="xs" />
												</span>
												<ArtifactLink id={node.id} />
												{#if node.artifact_type === "pillar" || node.artifact_type === "vision"}
													<Badge variant="secondary" class="px-1 py-0 text-[9px] capitalize">
														{node.artifact_type}
													</Badge>
												{/if}
											</div>

											<!-- Relationship label between nodes -->
											{#if node.relationship && nodeIdx < chain.path.length - 1}
												<span class="text-[10px] text-muted-foreground/60 italic ml-1">
													via {node.relationship}
												</span>
											{/if}
										</div>
									{/each}
								</div>
							{/each}
						</div>
					{/if}

					<!-- Descendants -->
					{#if result.descendants.length > 0}
						<div class="space-y-1">
							<Collapsible bind:open={descendantsOpen}>
								<CollapsibleTrigger
									class="flex items-center gap-1 text-[10px] font-medium uppercase tracking-wide text-muted-foreground hover:text-foreground transition-colors"
								>
									<span class={descendantsOpen ? "rotate-90 transition-transform" : "transition-transform"}>
										<Icon name="chevron-right" size="xs" />
									</span>
									What does this affect?
									<span class="ml-1 text-[10px] text-muted-foreground">
										({result.descendants.length})
									</span>
								</CollapsibleTrigger>
								<CollapsibleContent>
									<div class="flex flex-wrap gap-1 pt-1 pl-3">
										{#each visibleDescendants as desc (desc.id)}
											<div class="flex items-center gap-1">
												<ArtifactLink id={desc.id} />
												{#if desc.depth > 1}
													<span class="text-[10px] text-muted-foreground/50">+{desc.depth}</span>
												{/if}
											</div>
										{/each}
										{#if result.descendants.length > 20}
											<span class="text-[10px] text-muted-foreground">
												… and {result.descendants.length - 20} more
											</span>
										{/if}
									</div>
								</CollapsibleContent>
							</Collapsible>
						</div>
					{/if}

					<!-- Siblings -->
					{#if result.siblings.length > 0}
						<div class="space-y-1">
							<Collapsible bind:open={siblingsOpen}>
								<CollapsibleTrigger
									class="flex items-center gap-1 text-[10px] font-medium uppercase tracking-wide text-muted-foreground hover:text-foreground transition-colors"
								>
									<span class={siblingsOpen ? "rotate-90 transition-transform" : "transition-transform"}>
										<Icon name="chevron-right" size="xs" />
									</span>
									Related work
									<span class="ml-1 text-[10px] text-muted-foreground">
										({result.siblings.length})
									</span>
								</CollapsibleTrigger>
								<CollapsibleContent>
									<div class="flex flex-wrap gap-1 pt-1 pl-3">
										{#each result.siblings as siblingId (siblingId)}
											<ArtifactLink id={siblingId} />
										{/each}
									</div>
								</CollapsibleContent>
							</Collapsible>
						</div>
					{/if}

					{#if !hasContent && !result.disconnected}
						<p class="text-[11px] text-muted-foreground">No traceability data for this artifact.</p>
					{/if}
				</div>
			</CollapsibleContent>
		</Collapsible>
	</div>
{/if}

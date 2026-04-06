<!-- Collapsible panel showing artifact traceability: ancestry chains (why it exists), descendants (what it affects), and siblings (related work). -->
<script lang="ts">
	import { SvelteSet } from "svelte/reactivity";
	import {
		Icon,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
		Badge,
		HStack,
		Stack,
		Text,
		Caption,
		Panel,
	} from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ArtifactLink } from "@orqastudio/svelte-components/connected";
	import type { TraceabilityResult, AncestryChain } from "@orqastudio/types";
	import { getStores } from "@orqastudio/sdk";

	const { pluginRegistry } = getStores();

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
	<Panel padding="tight" border="bottom">
		<HStack gap={2}>
			<LoadingSpinner size="sm" />
			<Caption>Loading traceability…</Caption>
		</HStack>
	</Panel>
{:else if error}
	<Panel padding="tight" border="bottom">
		<Text variant="caption" tone="destructive" block>{error}</Text>
	</Panel>
{:else if result}
	<Panel padding="tight" border="bottom">
		<Collapsible bind:open>
			<CollapsibleTrigger
				class="flex w-full items-center gap-1 text-xs font-medium text-muted-foreground hover:text-foreground transition-colors"
			>
				<span class={open ? "rotate-90 transition-transform" : "transition-transform"}>
					<Icon name="chevron-right" size="xs" />
				</span>
				<span>Traceability</span>
				{#if result.disconnected}
					<Badge variant="destructive" size="xs">disconnected</Badge>
				{:else if result.impact_radius > 0}
					<span class="ml-auto text-[10px] text-muted-foreground">
						{result.impact_radius} affected
					</span>
				{/if}
			</CollapsibleTrigger>

			<CollapsibleContent>
				<Panel padding="normal">
				<Stack gap={3}>

					<!-- Disconnected warning — bg-warning/10 is not in Box background map, kept as div. -->
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
						<Stack gap={2}>
							<Caption variant="caption-strong" block>Why does this exist?</Caption>
							{#each uniqueChains as chain, chainIdx (chainIdx)}
								<Stack gap={0.5}>
									{#each chain.path as node, nodeIdx (node.id + nodeIdx)}
										<HStack gap={1}>
											<!-- Indent guide line — style= indentation is dynamic, must use inline style. -->
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

											<!-- Artifact chip — indent by node depth in the chain (capped at 8 levels). -->
											<HStack gap={1} indent={Math.min(nodeIdx, 8) as 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8}>
												<span class="shrink-0 text-muted-foreground">
													<Icon name={pluginRegistry.getIconForType(node.artifact_type)} size="xs" />
												</span>
												<ArtifactLink id={node.id} />
												{#if node.artifact_type === "pillar" || node.artifact_type === "vision"}
													<Badge variant="secondary" size="xs">{node.artifact_type}</Badge>
												{/if}
											</HStack>

											<!-- Relationship label between nodes -->
											{#if node.relationship && nodeIdx < chain.path.length - 1}
												<span class="text-[10px] text-muted-foreground/60 italic ml-1">
													via {node.relationship}
												</span>
											{/if}
										</HStack>
									{/each}
								</Stack>
							{/each}
						</Stack>
					{/if}

					<!-- Descendants -->
					{#if result.descendants.length > 0}
						<Stack gap={1}>
							<Collapsible bind:open={descendantsOpen}>
								<CollapsibleTrigger
									class="text-[10px] font-medium uppercase tracking-wide text-muted-foreground hover:text-foreground transition-colors"
								>
									<HStack gap={1}>
										<span class={descendantsOpen ? "rotate-90 transition-transform" : "transition-transform"}>
											<Icon name="chevron-right" size="xs" />
										</span>
										What does this affect?
										<span class="ml-1 text-[10px] text-muted-foreground">
											({result.descendants.length})
										</span>
									</HStack>
								</CollapsibleTrigger>
								<CollapsibleContent>
									<Panel padding="tight"><HStack wrap gap={1}>
										{#each visibleDescendants as desc (desc.id)}
											<HStack gap={1}>
												<ArtifactLink id={desc.id} />
												{#if desc.depth > 1}
													<span class="text-[10px] text-muted-foreground/50">+{desc.depth}</span>
												{/if}
											</HStack>
										{/each}
										{#if result.descendants.length > 20}
											<Caption>… and {result.descendants.length - 20} more</Caption>
										{/if}
									</HStack></Panel>
								</CollapsibleContent>
							</Collapsible>
						</Stack>
					{/if}

					<!-- Siblings -->
					{#if result.siblings.length > 0}
						<Stack gap={1}>
							<Collapsible bind:open={siblingsOpen}>
								<CollapsibleTrigger
									class="text-[10px] font-medium uppercase tracking-wide text-muted-foreground hover:text-foreground transition-colors"
								>
									<HStack gap={1}>
										<span class={siblingsOpen ? "rotate-90 transition-transform" : "transition-transform"}>
											<Icon name="chevron-right" size="xs" />
										</span>
										Related work
										<span class="ml-1 text-[10px] text-muted-foreground">
											({result.siblings.length})
										</span>
									</HStack>
								</CollapsibleTrigger>
								<CollapsibleContent>
									<Panel padding="tight"><HStack wrap gap={1}>
										{#each result.siblings as siblingId (siblingId)}
											<ArtifactLink id={siblingId} />
										{/each}
									</HStack></Panel>
								</CollapsibleContent>
							</Collapsible>
						</Stack>
					{/if}

					{#if !hasContent && !result.disconnected}
						<Caption block>No traceability data for this artifact.</Caption>
					{/if}
				</Stack>
				</Panel>
			</CollapsibleContent>
		</Collapsible>
	</Panel>
{/if}

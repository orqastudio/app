<!-- Collapsible panel showing artifact traceability: ancestry chains (why it exists), descendants (what it affects), and siblings (related work). -->
<script lang="ts">
	import { SvelteSet } from "svelte/reactivity";
	import {
		Icon,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleSection,
		Badge,
		HStack,
		Stack,
		Text,
		Caption,
		Panel,
		Callout,
		LoadingSpinner,
		TreeGuideLine,
		TreeIndentIcon,
	} from "@orqastudio/svelte-components/pure";
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
		result !== null &&
			(result.ancestry_chains.length > 0 ||
				result.descendants.length > 0 ||
				result.siblings.length > 0),
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
	const visibleDescendants = $derived(result ? result.descendants.slice(0, 20) : []);
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
			<CollapsibleSection variant="link">
				<Icon name="chevron-right" size="xs" rotate90={open} />
				Traceability
				{#if result.disconnected}
					<Badge variant="destructive" size="xs">disconnected</Badge>
				{:else if result.impact_radius > 0}
					<Caption tone="muted">{result.impact_radius} affected</Caption>
				{/if}
			</CollapsibleSection>

			<CollapsibleContent>
				<Panel padding="normal">
					<Stack gap={3}>
						<!-- Disconnected warning -->
						{#if result.disconnected}
							<Callout tone="warning" density="compact" align="start" iconName="triangle-alert">
								<Caption tone="warning">
									This artifact has no path to any pillar. It is disconnected from the vision
									hierarchy.
								</Caption>
							</Callout>
						{/if}

						<!-- Ancestry chains -->
						{#if uniqueChains.length > 0}
							<Stack gap={2}>
								<Caption variant="caption-strong" block>Why does this exist?</Caption>
								{#each uniqueChains as chain, chainIdx (chainIdx)}
									<Stack gap={0.5}>
										{#each chain.path as node, nodeIdx (node.id + nodeIdx)}
											<HStack gap={1}>
												<!-- Indent guide line at depth-computed offset -->
												{#if nodeIdx > 0}
													<TreeGuideLine depth={nodeIdx - 1} />
													<TreeIndentIcon name="corner-down-right" depth={nodeIdx - 1} />
												{/if}

												<!-- Artifact chip — indent by node depth in the chain (capped at 8 levels). -->
												<HStack
													gap={1}
													indent={Math.min(nodeIdx, 8) as 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8}
												>
													<Icon
														tone="muted"
														name={pluginRegistry.getIconForType(node.artifact_type)}
														size="xs"
													/>
													<ArtifactLink id={node.id} />
													{#if node.artifact_type === "pillar" || node.artifact_type === "vision"}
														<Badge variant="secondary" size="xs">{node.artifact_type}</Badge>
													{/if}
												</HStack>

												<!-- Relationship label between nodes -->
												{#if node.relationship && nodeIdx < chain.path.length - 1}
													<Caption tone="muted" italic>via {node.relationship}</Caption>
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
									<CollapsibleSection variant="subheading">
										<HStack gap={1}>
											<Icon name="chevron-right" size="xs" rotate90={descendantsOpen} />
											What does this affect?
											<Caption tone="muted">({result.descendants.length})</Caption>
										</HStack>
									</CollapsibleSection>
									<CollapsibleContent>
										<Panel padding="tight"
											><HStack wrap gap={1}>
												{#each visibleDescendants as desc (desc.id)}
													<HStack gap={1}>
														<ArtifactLink id={desc.id} />
														{#if desc.depth > 1}
															<Caption tone="muted">+{desc.depth}</Caption>
														{/if}
													</HStack>
												{/each}
												{#if result.descendants.length > 20}
													<Caption>… and {result.descendants.length - 20} more</Caption>
												{/if}
											</HStack></Panel
										>
									</CollapsibleContent>
								</Collapsible>
							</Stack>
						{/if}

						<!-- Siblings -->
						{#if result.siblings.length > 0}
							<Stack gap={1}>
								<Collapsible bind:open={siblingsOpen}>
									<CollapsibleSection variant="subheading">
										<HStack gap={1}>
											<Icon name="chevron-right" size="xs" rotate90={siblingsOpen} />
											Related work
											<Caption tone="muted">({result.siblings.length})</Caption>
										</HStack>
									</CollapsibleSection>
									<CollapsibleContent>
										<Panel padding="tight"
											><HStack wrap gap={1}>
												{#each result.siblings as siblingId (siblingId)}
													<ArtifactLink id={siblingId} />
												{/each}
											</HStack></Panel
										>
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

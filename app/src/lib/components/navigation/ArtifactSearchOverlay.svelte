<script lang="ts">
	import {
		Caption,
		Box,
		HStack,
		Text,
		Button,
		Panel,
		SectionHeader,
		SectionFooter,
		ScrollArea,
		Icon,
		Backdrop,
		SearchCard,
		SearchBarInput,
		SearchResultItem,
	} from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { navigationStore, artifactGraphSDK } = getStores();
	import { statusIconName } from "@orqastudio/svelte-components/pure";
	import type { ArtifactNode } from "@orqastudio/types";

	let query = $state("");
	let inputEl = $state<HTMLInputElement | null>(null);
	let selectedIndex = $state(0);

	const open = $derived(navigationStore.searchOverlayOpen);

	// Auto-focus when overlay opens
	$effect(() => {
		if (open) {
			query = "";
			selectedIndex = 0;
			setTimeout(() => inputEl?.focus(), 0);
		}
	});

	// Search results derived from query
	const results = $derived.by(() => {
		if (!query.trim()) return [] as ArtifactNode[];
		const q = query.trim().toLowerCase();
		const matches: ArtifactNode[] = [];

		for (const node of artifactGraphSDK.graph.values()) {
			const idMatch = node.id.toLowerCase().includes(q);
			const titleMatch = node.title.toLowerCase().includes(q);
			const descMatch = node.description?.toLowerCase().includes(q) ?? false;

			if (idMatch || titleMatch || descMatch) {
				matches.push(node);
			}

			if (matches.length >= 50) break;
		}

		// Sort: exact ID matches first, then partial ID, then title/description
		return matches.sort((a, b) => {
			const aId = a.id.toLowerCase() === q ? 0 : a.id.toLowerCase().includes(q) ? 1 : 2;
			const bId = b.id.toLowerCase() === q ? 0 : b.id.toLowerCase().includes(q) ? 1 : 2;
			if (aId !== bId) return aId - bId;
			return a.title.localeCompare(b.title);
		});
	});

	// Clamp selected index when results change
	$effect(() => {
		if (selectedIndex >= results.length) {
			selectedIndex = Math.max(0, results.length - 1);
		}
	});

	/** Close the search overlay by setting the navigation store flag to false. */
	function close() {
		navigationStore.searchOverlayOpen = false;
	}

	/**
	 * Navigate to the selected artifact and close the search overlay.
	 * @param node - The artifact node the user selected from the search results.
	 */
	function selectResult(node: ArtifactNode) {
		navigationStore.navigateToArtifact(node.id);
		close();
	}

	/**
	 * Handle keyboard navigation within the search overlay (Escape, ArrowUp, ArrowDown, Enter).
	 * @param e - The keyboard event forwarded from the Backdrop onkeydown handler.
	 */
	function handleKeydown(e: KeyboardEvent) {
		if (e.key === "Escape") {
			e.preventDefault();
			close();
		} else if (e.key === "ArrowDown") {
			e.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
		} else if (e.key === "ArrowUp") {
			e.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (e.key === "Enter" && results.length > 0) {
			e.preventDefault();
			selectResult(results[selectedIndex]);
		}
	}
</script>

{#if open}
	<Backdrop label="Search artifacts" onclick={close} onkeydown={handleKeydown}>
		<SearchCard>
			<!-- Search input row -->
			<SectionHeader>
				<Icon name="search" size="md" />
				<SearchBarInput bind:value={query} bind:ref={inputEl} placeholder="Search artifacts..." />
				{#if query}
					<Button
						variant="ghost"
						size="icon-sm"
						onclick={() => {
							query = "";
							inputEl?.focus();
						}}
						aria-label="Clear search"
					>
						<Icon name="x" size="sm" />
					</Button>
				{/if}
			</SectionHeader>

			<!-- Results list -->
			{#if query.trim() && results.length > 0}
				<Box>
					<ScrollArea>
						<Panel padding="tight">
							{#each results as node, i (node.id)}
								<SearchResultItem
									iconName={node.status ? statusIconName(node.status) : "file-text"}
									project={node.project ?? undefined}
									id={node.id}
									title={node.title}
									artifactType={node.artifact_type ?? undefined}
									active={i === selectedIndex}
									onclick={() => selectResult(node)}
									onmouseenter={() => {
										selectedIndex = i;
									}}
								/>
							{/each}
						</Panel>
					</ScrollArea>
				</Box>
			{:else if query.trim()}
				<Panel padding="loose">
					<Caption>No matching artifacts</Caption>
				</Panel>
			{:else}
				<Panel padding="loose">
					<Caption>Type to search across all artifacts</Caption>
				</Panel>
			{/if}

			<!-- Footer hint -->
			<SectionFooter variant="compact">
				{#snippet start()}
					<Text variant="caption">↑↓ Navigate</Text>
				{/snippet}
				{#snippet end()}
					<HStack gap={2}>
						{#if query.trim() && results.length > 0}
							<Text variant="caption"
								>{results.length}{results.length >= 50 ? "+" : ""} results</Text
							>
						{/if}
						<Text variant="caption">↵ Open</Text>
						<Text variant="caption">Esc Close</Text>
					</HStack>
				{/snippet}
			</SectionFooter>
		</SearchCard>
	</Backdrop>
{/if}

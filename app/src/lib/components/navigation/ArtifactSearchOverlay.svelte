<script lang="ts">
	import { Icon, Caption, Box, Stack, HStack, Text, Button, SearchInput } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { navigationStore, artifactGraphSDK } = getStores();
	import { statusIconName, resolveIcon } from "@orqastudio/svelte-components/pure";
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

	function close() {
		navigationStore.searchOverlayOpen = false;
	}

	function selectResult(node: ArtifactNode) {
		navigationStore.navigateToArtifact(node.id);
		close();
	}

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

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			close();
		}
	}
</script>

{#if open}
	<!-- Backdrop — fixed overlay that intercepts clicks outside the search card -->
	<div
		class="fixed inset-0 z-50 bg-background/60 backdrop-blur-sm"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		aria-label="Search artifacts"
		tabindex="-1"
	>
		<!-- Centred card in upper third -->
		<div class="mx-auto mt-[15vh] w-full max-w-xl px-4">
			<div class="rounded-lg border border-border bg-popover shadow-2xl">
				<!-- Search input row -->
				<HStack gap={2} paddingX={3} borderBottom>
					<Icon name="search" size="md" />
					<input
						bind:this={inputEl}
						bind:value={query}
						placeholder="Search artifacts..."
						class="h-12 flex-1 bg-transparent text-sm text-foreground outline-none placeholder:text-muted-foreground"
					/>
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
				</HStack>

				<!-- Results list -->
				{#if query.trim() && results.length > 0}
					<Box overflow="auto">
						<Box padding={1}>
							{#each results as node, i (node.id)}
								<button
									class="flex w-full items-center justify-start gap-2 rounded-md px-2 py-1.5 text-sm {i === selectedIndex ? 'bg-accent text-accent-foreground' : 'hover:bg-accent/50'}"
									onclick={() => selectResult(node)}
									onmouseenter={() => {
										selectedIndex = i;
									}}
								>
									<!-- Status icon -->
									{#if node.status}
										{@const StatusIcon = resolveIcon(statusIconName(node.status))}
										<StatusIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
									{:else}
										<Icon name="file-text" size="sm" />
									{/if}

									<!-- Project badge (org mode) -->
									{#if node.project}
										<span class="shrink-0 rounded bg-primary/10 px-1 py-0.5 text-[9px] font-medium text-primary">
											{node.project}
										</span>
									{/if}

									<!-- ID badge -->
									<span class="shrink-0 rounded bg-muted px-1 py-0.5 font-mono text-[11px] text-muted-foreground">
										{node.id}
									</span>

									<!-- Title -->
									<span class="min-w-0 flex-1 truncate">{node.title}</span>

									<!-- Type -->
									<Text variant="caption" truncate>{node.artifact_type}</Text>
								</button>
							{/each}
						</Box>
					</Box>
				{:else if query.trim()}
					<Box paddingX={3} paddingY={6}>
						<Caption>No matching artifacts</Caption>
					</Box>
				{:else}
					<Box paddingX={3} paddingY={6}>
						<Caption>Type to search across all artifacts</Caption>
					</Box>
				{/if}

				<!-- Footer hint -->
				<HStack justify="between" paddingX={3} paddingY={1} borderTop>
					<Text variant="caption">↑↓ Navigate</Text>
					{#if query.trim() && results.length > 0}
						<Text variant="caption">{results.length}{results.length >= 50 ? "+" : ""} results</Text>
					{/if}
					<Text variant="caption">↵ Open</Text>
					<Text variant="caption">Esc Close</Text>
				</HStack>
			</div>
		</div>
	</div>
{/if}

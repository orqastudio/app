<script lang="ts">
	import { Icon, Caption } from "@orqastudio/svelte-components/pure";
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
	<!-- Backdrop -->
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
				<!-- Search input -->
				<div class="flex items-center gap-2 border-b border-border px-3">
					<Icon name="search" size="md" />
					<input
						bind:this={inputEl}
						bind:value={query}
						placeholder="Search artifacts..."
						class="h-12 flex-1 bg-transparent text-sm text-foreground outline-none placeholder:text-muted-foreground"
					/>
					{#if query}
						<button
							class="flex h-6 w-6 items-center justify-center rounded hover:bg-accent"
							onclick={() => {
								query = "";
								inputEl?.focus();
							}}
						>
							<Icon name="x" size="sm" />
						</button>
					{/if}
				</div>

				<!-- Results -->
				{#if query.trim() && results.length > 0}
					<div class="max-h-[40vh] overflow-y-auto">
						<div class="p-1">
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
									<span
										class="shrink-0 rounded bg-muted px-1 py-0.5 font-mono text-[11px] text-muted-foreground"
									>
										{node.id}
									</span>

									<!-- Title -->
									<span class="min-w-0 flex-1 truncate">{node.title}</span>

									<!-- Type -->
									<span class="shrink-0 text-xs uppercase tracking-wider text-muted-foreground">
										{node.artifact_type}
									</span>
								</button>
							{/each}
						</div>
					</div>
				{:else if query.trim()}
					<div class="flex items-center justify-center px-3 py-6">
						<Caption>No matching artifacts</Caption>
					</div>
				{:else}
					<div class="flex items-center justify-center px-3 py-6">
						<Caption>Type to search across all artifacts</Caption>
					</div>
				{/if}

				<!-- Footer hint -->
				<div
					class="flex items-center justify-between border-t border-border px-3 py-1.5 text-[10px] text-muted-foreground"
				>
					<span>↑↓ Navigate</span>
					{#if query.trim() && results.length > 0}
						<span>{results.length}{results.length >= 50 ? "+" : ""} results</span>
					{/if}
					<span>↵ Open</span>
					<span>Esc Close</span>
				</div>
			</div>
		</div>
	</div>
{/if}

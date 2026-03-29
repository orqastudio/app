<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardContent } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import {
		getStores,
		buildConflictResolutionPrompt,
		parseConflictResolutionResponse,
	} from "@orqastudio/sdk";
	import type { RegistrationConflict } from "@orqastudio/sdk";
	import type { PluginManifest, ConflictResolutionSuggestion } from "@orqastudio/types";

	const { conversationStore } = getStores();

	// -----------------------------------------------------------------------
	// Props
	// -----------------------------------------------------------------------

	interface Props {
		conflicts: RegistrationConflict[];
		existingManifest: PluginManifest;
		newManifest: PluginManifest;
		onResolve: (resolutions: Record<string, { plugin: string; alias: string; label?: string }>) => void;
		onCancel: () => void;
	}

	const { conflicts, existingManifest, newManifest, onResolve, onCancel }: Props = $props();

	// -----------------------------------------------------------------------
	// State
	// -----------------------------------------------------------------------

	let suggestions = $state<ConflictResolutionSuggestion[]>([]);
	let loadingSuggestions = $state(false);
	let suggestionsError = $state<string | null>(null);

	/** Per-conflict resolution choices. Key = conflict key, value = chosen alias config. */
	let resolutions = $state<Record<string, { plugin: string; alias: string; label?: string }>>({});

	/** Custom input mode per conflict. */
	let customMode = $state<Record<string, boolean>>({});
	let customInputs = $state<Record<string, string>>({});

	const allResolved = $derived(
		conflicts.every((c) => resolutions[c.key] !== undefined),
	);

	// -----------------------------------------------------------------------
	// AI Suggestions
	// -----------------------------------------------------------------------

	$effect(() => {
		void fetchSuggestions();
	});

	/** Fetches AI-generated conflict resolution suggestions from the conversation store. */
	async function fetchSuggestions() {
		loadingSuggestions = true;
		suggestionsError = null;

		try {
			const prompt = buildConflictResolutionPrompt(
				conflicts,
				existingManifest,
				newManifest,
			);

			// Use the store's one-shot method — no invoke() in components (RULE-006).
			const response = await conversationStore.oneShotMessage(prompt);

			suggestions = parseConflictResolutionResponse(response);
		} catch {
			// AI suggestions are optional — dialog still works with manual input
			suggestionsError = "AI suggestions unavailable — enter aliases manually below.";
			suggestions = [];
		} finally {
			loadingSuggestions = false;
		}
	}

	// -----------------------------------------------------------------------
	// Resolution Actions
	// -----------------------------------------------------------------------

	/**
	 * Applies an AI-suggested resolution to the resolutions map for its conflict key.
	 * @param suggestion - The resolution suggestion to apply.
	 */
	function applySuggestion(suggestion: ConflictResolutionSuggestion) {
		switch (suggestion.strategy) {
			case "rename-new":
				if (suggestion.newAlias) {
					resolutions[suggestion.key] = {
						plugin: newManifest.name,
						alias: suggestion.newAlias,
					};
				}
				break;
			case "rename-existing":
				if (suggestion.existingAlias) {
					resolutions[suggestion.key] = {
						plugin: existingManifest.name,
						alias: suggestion.existingAlias,
					};
				}
				break;
			case "rename-both":
				// For rename-both, we rename the new plugin's key (less disruption)
				if (suggestion.newAlias) {
					resolutions[suggestion.key] = {
						plugin: newManifest.name,
						alias: suggestion.newAlias,
					};
				}
				break;
		}
	}

	/**
	 * Applies the manually entered custom alias for a conflict key.
	 * @param conflictKey - The conflict key to resolve with the custom alias.
	 */
	function applyCustom(conflictKey: string) {
		const alias = customInputs[conflictKey]?.trim();
		if (!alias) return;
		resolutions[conflictKey] = {
			plugin: newManifest.name,
			alias,
		};
		customMode[conflictKey] = false;
	}

	/**
	 * Clears the current resolution for a conflict key, allowing re-selection.
	 * @param conflictKey - The conflict key whose resolution should be cleared.
	 */
	function clearResolution(conflictKey: string) {
		delete resolutions[conflictKey];
		resolutions = { ...resolutions };
	}
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm">
	<div class="w-full max-w-lg space-y-4 rounded-lg border border-border bg-background p-6 shadow-lg">
		<!-- Header -->
		<div class="flex items-center gap-3">
			<div class="flex h-10 w-10 items-center justify-center rounded-full bg-amber-500/10">
				<Icon name="triangle-alert" size="md" />
			</div>
			<div>
				<h2 class="text-sm font-semibold">Plugin Conflicts Detected</h2>
				<p class="text-xs text-muted-foreground">
					{newManifest.displayName ?? newManifest.name} conflicts with {existingManifest.displayName ?? existingManifest.name}
				</p>
			</div>
		</div>

		<!-- Conflicts -->
		<div class="max-h-80 space-y-3 overflow-y-auto">
			{#each conflicts as conflict (conflict.key)}
				<CardRoot class="gap-1">
					<CardHeader class="pb-1">
						<CardTitle class="text-xs font-semibold">
							<div class="flex items-center gap-2">
								<Badge variant="outline" class="text-[9px] px-1 py-0">{conflict.type}</Badge>
								<code class="text-xs">{conflict.key}</code>
							</div>
						</CardTitle>
					</CardHeader>
					<CardContent class="pt-0 space-y-2">
						<p class="text-[10px] text-muted-foreground">{conflict.detail}</p>

						<!-- Resolution status -->
						{#if resolutions[conflict.key]}
							<div class="flex items-center justify-between rounded bg-accent/30 px-2 py-1.5">
								<div class="text-xs">
									<span class="text-muted-foreground">Rename</span>
									<span class="font-medium">{resolutions[conflict.key].plugin}</span>
									<span class="text-muted-foreground">→</span>
									<code class="font-semibold">{resolutions[conflict.key].alias}</code>
								</div>
								<button
									class="text-xs text-muted-foreground hover:text-foreground"
									onclick={() => clearResolution(conflict.key)}
								>
									<Icon name="x" size="sm" />
								</button>
							</div>
						{:else}
							<!-- AI suggestions for this conflict -->
							{#if loadingSuggestions}
								<div class="flex items-center gap-2 py-2">
									<LoadingSpinner size="sm" />
									<span class="text-[10px] text-muted-foreground">Getting AI suggestions...</span>
								</div>
							{:else}
								{@const conflictSuggestions = suggestions.filter((s) => s.key === conflict.key)}
								{#if conflictSuggestions.length > 0}
									<div class="space-y-1">
										{#each conflictSuggestions as suggestion (suggestion.key)}
											<button
												class="w-full rounded border border-border px-2 py-1.5 text-left transition-colors hover:bg-accent/30"
												onclick={() => applySuggestion(suggestion)}
											>
												<div class="flex items-center justify-between">
													<div class="text-xs">
														{#if suggestion.strategy === "rename-new" && suggestion.newAlias}
															Rename new → <code class="font-semibold">{suggestion.newAlias}</code>
														{:else if suggestion.strategy === "rename-existing" && suggestion.existingAlias}
															Rename existing → <code class="font-semibold">{suggestion.existingAlias}</code>
														{:else if suggestion.strategy === "rename-both"}
															Rename both → <code class="font-semibold">{suggestion.newAlias}</code> / <code class="font-semibold">{suggestion.existingAlias}</code>
														{/if}
													</div>
													<Icon name="chevron-right" size="sm" />
												</div>
												<p class="mt-0.5 text-[10px] text-muted-foreground">{suggestion.rationale}</p>
											</button>
										{/each}
									</div>
								{/if}

								<!-- Custom input -->
								{#if customMode[conflict.key]}
									<div class="flex gap-2">
										<input
											type="text"
											class="flex-1 rounded border border-border bg-background px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-ring"
											placeholder="Enter custom alias..."
											bind:value={customInputs[conflict.key]}
											onkeydown={(e: KeyboardEvent) => { if (e.key === "Enter") applyCustom(conflict.key); }}
										/>
										<Button
											variant="default"
											size="sm"
											class="h-7 px-2 text-xs"
											disabled={!customInputs[conflict.key]?.trim()}
											onclick={() => applyCustom(conflict.key)}
										>
											Apply
										</Button>
										<Button
											variant="ghost"
											size="sm"
											class="h-7 px-2 text-xs"
											onclick={() => { customMode[conflict.key] = false; }}
										>
											Cancel
										</Button>
									</div>
								{:else}
									<button
										class="text-xs text-muted-foreground hover:text-foreground"
										onclick={() => { customMode[conflict.key] = true; }}
									>
										Enter custom alias...
									</button>
								{/if}
							{/if}
						{/if}
					</CardContent>
				</CardRoot>
			{/each}
		</div>

		{#if suggestionsError}
			<p class="text-[10px] text-muted-foreground">{suggestionsError}</p>
		{/if}

		<!-- Actions -->
		<div class="flex justify-end gap-2 pt-2 border-t border-border">
			<Button variant="ghost" size="sm" class="text-xs" onclick={onCancel}>
				Cancel Install
			</Button>
			<Button
				variant="default"
				size="sm"
				class="text-xs"
				disabled={!allResolved}
				onclick={() => onResolve(resolutions)}
			>
				{#if allResolved}
					Apply & Install
				{:else}
					Resolve all conflicts to continue
				{/if}
			</Button>
		</div>
	</div>
</div>

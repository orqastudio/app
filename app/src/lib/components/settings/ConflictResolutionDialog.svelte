<script lang="ts">
	import {
		Icon,
		HStack,
		Stack,
		Text,
		Heading,
		ScrollArea,
		Panel,
		Caption,
		Code,
	} from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardContent } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Input } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import {
		getStores,
		buildConflictResolutionPrompt,
		parseConflictResolutionResponse,
	} from "@orqastudio/sdk";
	import type { RegistrationConflict } from "@orqastudio/sdk";
	import type { PluginManifest, ConflictResolutionSuggestion } from "@orqastudio/types";
	import { assertNever } from "@orqastudio/types";

	const { conversationStore } = getStores();

	// -----------------------------------------------------------------------
	// Props
	// -----------------------------------------------------------------------

	interface Props {
		conflicts: RegistrationConflict[];
		existingManifest: PluginManifest;
		newManifest: PluginManifest;
		onResolve: (
			resolutions: Record<string, { plugin: string; alias: string; label?: string }>,
		) => void;
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

	const allResolved = $derived(conflicts.every((c) => resolutions[c.key] !== undefined));

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
			const prompt = buildConflictResolutionPrompt(conflicts, existingManifest, newManifest);

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
			default:
				assertNever(suggestion.strategy);
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

<!-- Exception: fixed overlay with backdrop-blur-sm — requires CSS positioning/filter not expressible via ORQA layout primitives -->
<div class="bg-background/80 fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm">
	<Panel padding="loose" rounded="lg" border="all">
		<Stack gap={4}>
			<!-- Header -->
			<HStack gap={3}>
				<!-- Exception: amber warning circle — bg-amber-500/10 is a semantic warning color not in the Panel background token set -->
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-amber-500/10">
					<Icon name="triangle-alert" size="md" />
				</div>
				<Stack gap={0}>
					<Heading level={4}>Plugin Conflicts Detected</Heading>
					<Text variant="caption" tone="muted">
						{newManifest.displayName ?? newManifest.name} conflicts with {existingManifest.displayName ??
							existingManifest.name}
					</Text>
				</Stack>
			</HStack>

			<!-- Conflicts -->
			<ScrollArea maxHeight="lg">
				<Stack gap={3}>
					{#each conflicts as conflict (conflict.key)}
						<CardRoot gap={1}>
							<CardHeader>
								<CardTitle>
									<HStack gap={2}>
										<Badge variant="outline" size="xs">{conflict.type}</Badge>
										<Code>{conflict.key}</Code>
									</HStack>
								</CardTitle>
							</CardHeader>
							<CardContent>
								<Caption tone="muted">{conflict.detail}</Caption>

								<!-- Resolution status -->
								{#if resolutions[conflict.key]}
									<Panel padding="tight">
										<HStack gap={2} justify="between">
											<Text variant="caption">
												<Caption tone="muted">Rename</Caption>
												<Caption>{resolutions[conflict.key].plugin}</Caption>
												<Caption tone="muted">→</Caption>
												<Code>{resolutions[conflict.key].alias}</Code>
											</Text>
											<Button
												variant="ghost"
												size="icon-sm"
												onclick={() => clearResolution(conflict.key)}
											>
												<Icon name="x" size="sm" />
											</Button>
										</HStack>
									</Panel>
								{:else}
									<!-- AI suggestions for this conflict -->
									{#if loadingSuggestions}
										<Panel padding="tight">
											<HStack gap={2}>
												<LoadingSpinner size="sm" />
												<Caption tone="muted">Getting AI suggestions...</Caption>
											</HStack>
										</Panel>
									{:else}
										{@const conflictSuggestions = suggestions.filter((s) => s.key === conflict.key)}
										{#if conflictSuggestions.length > 0}
											<Stack gap={1}>
												{#each conflictSuggestions as suggestion (suggestion.key)}
													<Button
														variant="outline"
														size="sm"
														onclick={() => applySuggestion(suggestion)}
													>
														<HStack gap={2} justify="between">
															<Caption>
																{#if suggestion.strategy === "rename-new" && suggestion.newAlias}
																	Rename new → <Code>{suggestion.newAlias}</Code>
																{:else if suggestion.strategy === "rename-existing" && suggestion.existingAlias}
																	Rename existing → <Code>{suggestion.existingAlias}</Code>
																{:else if suggestion.strategy === "rename-both"}
																	Rename both → <Code>{suggestion.newAlias}</Code> / <Code
																		>{suggestion.existingAlias}</Code
																	>
																{/if}
															</Caption>
															<Icon name="chevron-right" size="sm" />
														</HStack>
														<Caption tone="muted">{suggestion.rationale}</Caption>
													</Button>
												{/each}
											</Stack>
										{/if}

										<!-- Custom input -->
										{#if customMode[conflict.key]}
											<HStack gap={2}>
												<Input
													placeholder="Enter custom alias..."
													bind:value={customInputs[conflict.key]}
													onkeydown={(e: KeyboardEvent) => {
														if (e.key === "Enter") applyCustom(conflict.key);
													}}
												/>
												<Button
													variant="default"
													size="sm"
													disabled={!customInputs[conflict.key]?.trim()}
													onclick={() => applyCustom(conflict.key)}
												>
													Apply
												</Button>
												<Button
													variant="ghost"
													size="sm"
													onclick={() => {
														customMode[conflict.key] = false;
													}}
												>
													Cancel
												</Button>
											</HStack>
										{:else}
											<Button
												variant="ghost"
												size="sm"
												onclick={() => {
													customMode[conflict.key] = true;
												}}
											>
												Enter custom alias...
											</Button>
										{/if}
									{/if}
								{/if}
							</CardContent>
						</CardRoot>
					{/each}
				</Stack>
			</ScrollArea>

			{#if suggestionsError}
				<Caption tone="muted">{suggestionsError}</Caption>
			{/if}

			<!-- Actions -->
			<Panel border="top" padding="tight">
				<HStack gap={2} justify="end">
					<Button variant="ghost" size="sm" onclick={onCancel}>Cancel Install</Button>
					<Button
						variant="default"
						size="sm"
						disabled={!allResolved}
						onclick={() => onResolve(resolutions)}
					>
						{#if allResolved}
							Apply & Install
						{:else}
							Resolve all conflicts to continue
						{/if}
					</Button>
				</HStack>
			</Panel>
		</Stack>
	</Panel>
</div>

<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import BotIcon from "@lucide/svelte/icons/bot";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import SearchInput from "$lib/components/shared/SearchInput.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";
	import type { ArtifactType } from "$lib/types";
	import type { Component } from "svelte";

	let { category }: { category: ActivityView } = $props();

	let localFilter = $state(artifactStore.filterText);

	$effect(() => {
		artifactStore.setFilter(localFilter);
	});

	const categoryConfig: Record<
		string,
		{
			icon: Component;
			label: string;
			artifactType: ArtifactType;
			emptyTitle: string;
			emptyDescription: string;
		}
	> = {
		agents: {
			icon: BotIcon,
			label: "Agents",
			artifactType: "agent",
			emptyTitle: "No agents yet",
			emptyDescription:
				"Agents define AI personas with specialized knowledge and behavior. Create your first agent to customize how Claude works on your project.",
		},
		rules: {
			icon: ShieldIcon,
			label: "Rules",
			artifactType: "rule",
			emptyTitle: "No rules yet",
			emptyDescription:
				"Rules enforce coding standards and project conventions. They are automatically applied based on file path globs.",
		},
		skills: {
			icon: ZapIcon,
			label: "Skills",
			artifactType: "skill",
			emptyTitle: "No skills yet",
			emptyDescription:
				"Skills define reusable capabilities that agents can invoke during sessions. Create your first skill to get started.",
		},
		hooks: {
			icon: GitBranchIcon,
			label: "Hooks",
			artifactType: "hook",
			emptyTitle: "No hooks yet",
			emptyDescription:
				"Hooks include lifecycle hooks that run automated actions before or after AI operations, and hookify enforcement rules.",
		},
	};

	const config = $derived(categoryConfig[category]);
	const items = $derived(config ? artifactStore.artifactsByType(config.artifactType) : []);

	function handleItemClick(name: string, path: string) {
		if (!config) return;
		navigationStore.openArtifact(path, [name]);
	}
</script>

{#if config}
	<div class="flex h-full flex-col">
		<div class="border-b border-border p-2">
			<SearchInput
				bind:value={localFilter}
				placeholder="Filter {config.label.toLowerCase()}..."
				size="xs"
			/>
		</div>

		<ScrollArea.Root class="min-h-0 flex-1">
			<div class="p-1">
				{#if items.length === 0}
					<div class="px-2 py-8">
						<EmptyState
							icon={config.icon}
							title={config.emptyTitle}
							description={config.emptyDescription}
						/>
					</div>
				{:else}
					{#each items as item}
						<button
							class="flex w-full flex-col gap-0.5 rounded px-2 py-1.5 text-left hover:bg-accent/50"
							class:bg-accent={navigationStore.selectedArtifactPath === item.rel_path}
							onclick={() => handleItemClick(item.name, item.rel_path)}
						>
							<span class="truncate text-sm font-medium">{item.name}</span>
							{#if item.description}
								<p class="line-clamp-2 text-xs text-muted-foreground">
									{item.description}
								</p>
							{/if}
						</button>
					{/each}
				{/if}
			</div>
		</ScrollArea.Root>
	</div>
{/if}

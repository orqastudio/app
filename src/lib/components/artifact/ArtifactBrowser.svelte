<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import { Input } from "$lib/components/ui/input";
	import SearchIcon from "@lucide/svelte/icons/search";
	import UsersIcon from "@lucide/svelte/icons/users";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import ArtifactListItem from "./ArtifactListItem.svelte";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";
	import type { ArtifactType } from "$lib/types";
	import type { Component } from "svelte";

	let { category }: { category: ActivityView } = $props();

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
		docs: {
			icon: FileTextIcon,
			label: "Docs",
			artifactType: "doc",
			emptyTitle: "No docs yet",
			emptyDescription:
				"Docs are reference documents that provide context to agents during sessions. Add documentation to improve AI responses.",
		},
		agents: {
			icon: UsersIcon,
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
		navigationStore.openArtifact(path, [config.label, name]);
	}
</script>

{#if config}
	<div class="flex h-full flex-col">
		<!-- Search bar -->
		<div class="border-b border-border p-3">
			<div class="relative">
				<SearchIcon class="absolute left-2.5 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
				<Input
					class="pl-8"
					placeholder="Filter {config.label.toLowerCase()}..."
					value={artifactStore.filterText}
					oninput={(e: Event) => {
						const target = e.target as HTMLInputElement;
						artifactStore.setFilter(target.value);
					}}
				/>
			</div>
		</div>

		<!-- List -->
		{#if artifactStore.loading}
			<div class="flex flex-1 items-center justify-center">
				<LoadingSpinner />
			</div>
		{:else if artifactStore.error}
			<div class="flex flex-1 items-center justify-center px-4">
				<ErrorDisplay
					message={artifactStore.error}
					onRetry={() => artifactStore.setError(null)}
				/>
			</div>
		{:else if items.length === 0}
			<div class="flex flex-1 items-center justify-center px-4">
				<EmptyState
					icon={config.icon}
					title={config.emptyTitle}
					description={config.emptyDescription}
				/>
			</div>
		{:else}
			<ScrollArea.Root class="flex-1">
				<div class="space-y-1 p-2">
					{#each items as item}
						<ArtifactListItem
							artifact={item}
							selected={navigationStore.selectedArtifactPath === item.rel_path}
							onclick={() => handleItemClick(item.name, item.rel_path)}
						/>
					{/each}
				</div>
			</ScrollArea.Root>
		{/if}
	</div>
{/if}

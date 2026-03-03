<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import { Input } from "$lib/components/ui/input";
	import { Badge } from "$lib/components/ui/badge";
	import UsersIcon from "@lucide/svelte/icons/users";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import SearchIcon from "@lucide/svelte/icons/search";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";
	import type { ArtifactType, ComplianceStatus } from "$lib/types";
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

	const complianceVariant: Record<ComplianceStatus, "default" | "secondary" | "destructive" | "outline"> = {
		compliant: "default",
		non_compliant: "destructive",
		unknown: "secondary",
		error: "destructive",
	};

	function handleItemClick(name: string, path: string) {
		if (!config) return;
		navigationStore.openArtifact(path, [config.label, name]);
	}
</script>

{#if config}
	<div class="flex h-full flex-col">
		<div class="border-b border-border p-2">
			<div class="relative">
				<SearchIcon class="absolute left-2 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
				<Input
					class="h-7 pl-7 text-xs"
					placeholder="Filter {config.label.toLowerCase()}..."
					value={artifactStore.filterText}
					oninput={(e: Event) => {
						const target = e.target as HTMLInputElement;
						artifactStore.setFilter(target.value);
					}}
				/>
			</div>
		</div>

		<ScrollArea.Root class="flex-1">
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
							<div class="flex items-center justify-between gap-2">
								<span class="truncate text-sm font-medium">{item.name}</span>
								<Badge variant={complianceVariant[item.compliance_status]} class="text-[10px] px-1 py-0">
									{item.compliance_status}
								</Badge>
							</div>
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

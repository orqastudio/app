<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import BotIcon from "@lucide/svelte/icons/bot";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
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
			singular: string;
			artifactType: ArtifactType;
			description: string;
			location: string;
		}
	> = {
		agents: {
			icon: BotIcon,
			label: "Agents",
			singular: "agent",
			artifactType: "agent",
			description:
				"Agent definitions give AI personas specialized knowledge and behavior for your project.",
			location: ".claude/agents/",
		},
		rules: {
			icon: ShieldIcon,
			label: "Rules",
			singular: "rule",
			artifactType: "rule",
			description:
				"Rules enforce coding standards and project conventions. They are loaded automatically by Claude Code.",
			location: ".claude/rules/",
		},
		skills: {
			icon: ZapIcon,
			label: "Skills",
			singular: "skill",
			artifactType: "skill",
			description:
				"Skills define reusable capabilities that agents can invoke during sessions.",
			location: ".claude/skills/",
		},
		hooks: {
			icon: GitBranchIcon,
			label: "Hooks",
			singular: "hook",
			artifactType: "hook",
			description:
				"Hooks run automated actions at lifecycle events — before/after prompts, on stop, etc.",
			location: ".claude/hooks/",
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
		{:else}
			<div class="space-y-6 p-6">
				<!-- Header -->
				<div class="space-y-1">
					<h1 class="text-2xl font-semibold tracking-tight">{config.label}</h1>
					<p class="text-sm text-muted-foreground">{config.description}</p>
					<p class="text-xs text-muted-foreground">
						Source: <code class="rounded bg-muted px-1 py-0.5">{config.location}</code>
					</p>
				</div>

				{#if items.length === 0}
					<Card.Root>
						<Card.Content class="py-8 text-center">
							<p class="text-sm text-muted-foreground">
								No {config.label.toLowerCase()} found. Add files to <code class="rounded bg-muted px-1 py-0.5 text-xs">{config.location}</code> and re-scan.
							</p>
						</Card.Content>
					</Card.Root>
				{:else}
					<!-- Summary -->
					<p class="text-sm text-muted-foreground">
						{items.length} {items.length === 1 ? config.singular : config.label.toLowerCase()} detected. Select one from the sidebar to view its contents.
					</p>

					<!-- Card grid -->
					<div class="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
						{#each items as item}
							{@const Icon = config.icon}
							<button
								class="text-left"
								onclick={() => handleItemClick(item.name, item.rel_path)}
							>
								<Card.Root class="transition-colors hover:bg-accent/50">
									<Card.Content class="p-4">
										<div class="flex items-start gap-3">
											<Icon class="mt-0.5 h-4 w-4 shrink-0 text-muted-foreground" />
											<div class="min-w-0">
												<p class="truncate text-sm font-medium">{item.name}</p>
												{#if item.description}
													<p class="mt-0.5 line-clamp-2 text-xs text-muted-foreground">
														{item.description}
													</p>
												{/if}
											</div>
										</div>
									</Card.Content>
								</Card.Root>
							</button>
						{/each}
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}

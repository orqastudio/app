<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import BotIcon from "@lucide/svelte/icons/bot";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import ViolationsPanel from "$lib/components/governance/ViolationsPanel.svelte";
	import { Badge } from "$lib/components/ui/badge";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { enforcementStore } from "$lib/stores/enforcement.svelte";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";
	import type { Component } from "svelte";

	let { category }: { category: ActivityView } = $props();

	const categoryConfig: Record<
		string,
		{
			icon: Component;
			label: string;
			singular: string;
			description: string;
			location: string;
		}
	> = {
		agents: {
			icon: BotIcon,
			label: "Agents",
			singular: "agent",
			description:
				"Agent definitions give AI personas specialized knowledge and behavior for your project.",
			location: ".claude/agents/",
		},
		rules: {
			icon: ShieldIcon,
			label: "Rules",
			singular: "rule",
			description:
				"Rules enforce coding standards and project conventions. They are loaded automatically by Claude Code.",
			location: ".claude/rules/",
		},
		skills: {
			icon: ZapIcon,
			label: "Skills",
			singular: "skill",
			description:
				"Skills define reusable capabilities that agents can invoke during sessions.",
			location: ".claude/skills/",
		},
		hooks: {
			icon: GitBranchIcon,
			label: "Hooks",
			singular: "hook",
			description:
				"Hooks run automated actions at lifecycle events — before/after prompts, on stop, etc.",
			location: ".claude/hooks/",
		},
	};

	const config = $derived(categoryConfig[category]);

	/** Derive items from the navTree nodes for this category. */
	const items = $derived.by(() => {
		const navType = navigationStore.getNavType(category);
		if (!navType) return [];
		return navType.nodes.filter((n) => {
			// Filter out README nodes
			if (!n.path) return false;
			const name = n.path.replace(/\\/g, "/").split("/").pop() ?? "";
			return name !== "README" && name !== "README.md";
		});
	});

	// Violation counts per rule name (only relevant when category === "rules")
	const violationsByRule = $derived(
		enforcementStore.violations.reduce<Record<string, { blocks: number; warns: number }>>(
			(acc, v) => {
				if (!acc[v.rule_name]) acc[v.rule_name] = { blocks: 0, warns: 0 };
				if (v.action === "Block") acc[v.rule_name].blocks++;
				else acc[v.rule_name].warns++;
				return acc;
			},
			{},
		),
	);

	function handleItemClick(name: string, path: string) {
		if (!config) return;
		navigationStore.openArtifact(path, [name]);
	}
</script>

{#if config}
	<div class="flex h-full flex-col">
		{#if artifactStore.navTreeLoading}
			<div class="flex flex-1 items-center justify-center">
				<LoadingSpinner />
			</div>
		{:else if artifactStore.navTreeError}
			<div class="flex flex-1 items-center justify-center px-4">
				<ErrorDisplay
					message={artifactStore.navTreeError}
					onRetry={() => artifactStore.loadNavTree()}
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

				{#if category === "rules" && (enforcementStore.blockCount > 0 || enforcementStore.warnCount > 0)}
					<div class="flex items-center gap-2">
						{#if enforcementStore.blockCount > 0}
							<Badge variant="destructive" class="text-xs px-1.5 py-0.5">
								{enforcementStore.blockCount} blocked
							</Badge>
						{/if}
						{#if enforcementStore.warnCount > 0}
							<Badge variant="warning" class="text-xs px-1.5 py-0.5">
								{enforcementStore.warnCount} warned
							</Badge>
						{/if}
					</div>
				{/if}

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
						{#each items as item (item.path)}
							{@const Icon = config.icon}
							<button
								class="text-left"
								onclick={() => item.path && handleItemClick(item.label, item.path)}
							>
								<Card.Root class="transition-colors hover:bg-accent/50">
									<Card.Content class="p-4">
										<div class="flex items-start gap-3">
											<Icon class="mt-0.5 h-4 w-4 shrink-0 text-muted-foreground" />
											<div class="min-w-0 flex-1">
												<p class="truncate text-sm font-medium">{item.label}</p>
												{#if item.description}
													<p class="mt-0.5 line-clamp-2 text-xs text-muted-foreground">
														{item.description}
													</p>
												{/if}
											</div>
											{#if category === "rules" && violationsByRule[item.label]}
												{@const counts = violationsByRule[item.label]}
												<div class="flex shrink-0 items-center gap-1">
													{#if counts.blocks > 0}
														<Badge variant="destructive" class="h-4 px-1 py-0 text-xs">
															{counts.blocks}
														</Badge>
													{/if}
													{#if counts.warns > 0}
														<Badge variant="warning" class="h-4 px-1 py-0 text-xs">
															{counts.warns}
														</Badge>
													{/if}
												</div>
											{/if}
										</div>
									</Card.Content>
								</Card.Root>
							</button>
						{/each}
					</div>
				{/if}

				<!-- Violation history panel (rules category only) -->
				{#if category === "rules"}
					<div class="mt-2 h-72 overflow-hidden rounded-md border border-border">
						<ViolationsPanel
							violations={enforcementStore.violationHistory}
							loading={enforcementStore.historyLoading}
							error={enforcementStore.historyError}
							onRetry={() => enforcementStore.loadViolationHistory()}
						/>
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}

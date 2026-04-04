<script lang="ts">
	import { Icon, CardRoot, CardContent, Heading } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import ViolationsPanel from "$lib/components/governance/ViolationsPanel.svelte";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import type { ActivityView } from "@orqastudio/sdk";
	import { CATEGORY_CONFIG } from "$lib/config/category-config";

	const { artifactStore, enforcementStore, navigationStore } = getStores();

	let { category }: { category: ActivityView } = $props();

	const config = $derived(CATEGORY_CONFIG[category]);

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
					<Heading level={1}>{config.label}</Heading>
					<p class="text-sm text-muted-foreground">{config.description}</p>
					<p class="text-xs text-muted-foreground">
						Source: <code class="rounded bg-muted px-1 py-0.5">{config.location}</code>
					</p>
				</div>

				{#if category === "rules" && (enforcementStore.blockCount > 0 || enforcementStore.warnCount > 0)}
					<div class="flex items-center gap-2">
						{#if enforcementStore.blockCount > 0}
							<Badge variant="destructive" size="sm">
								{enforcementStore.blockCount} blocked
							</Badge>
						{/if}
						{#if enforcementStore.warnCount > 0}
							<Badge variant="warning" size="sm">
								{enforcementStore.warnCount} warned
							</Badge>
						{/if}
					</div>
				{/if}

				{#if items.length === 0}
					<CardRoot>
						<CardContent>
							<div class="py-8 text-center">
								<p class="text-sm text-muted-foreground">
									No {config.label.toLowerCase()} found. Add files to <code class="rounded bg-muted px-1 py-0.5 text-xs">{config.location}</code> and re-scan.
								</p>
							</div>
						</CardContent>
					</CardRoot>
				{:else}
					<!-- Summary -->
					<p class="text-sm text-muted-foreground">
						{items.length} {items.length === 1 ? config.singular : config.label.toLowerCase()} detected. Select one from the sidebar to view its contents.
					</p>

					<!-- Card grid -->
					<div class="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-3">
						{#each items as item (item.path)}
							<button
								class="h-auto w-full p-0 text-left rounded-md hover:bg-accent/50 transition-colors"
								onclick={() => item.path && handleItemClick(item.label, item.path)}
							>
								<CardRoot>
									<CardContent>
										<div class="flex items-start gap-3 p-4">
											<Icon name={config.icon} size="md" />
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
													<Badge variant="destructive" size="xs">
														{counts.blocks}
													</Badge>
												{/if}
												{#if counts.warns > 0}
													<Badge variant="warning" size="xs">
														{counts.warns}
													</Badge>
												{/if}
											</div>
											{/if}
										</div>
									</CardContent>
								</CardRoot>
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

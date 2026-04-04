<!-- Landing page for an artifact category: shows all items in a card grid with violation counts for rules. -->
<script lang="ts">
	import { Icon, CardRoot, CardContent, Heading, Badge, Stack, HStack, Grid, Text, Caption, Box } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import ViolationsPanel from "$lib/components/governance/ViolationsPanel.svelte";
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
	<Stack height="full">
		{#if artifactStore.navTreeLoading}
			<HStack justify="center" align="center" flex={1} height="full">
				<LoadingSpinner />
			</HStack>
		{:else if artifactStore.navTreeError}
			<HStack justify="center" align="center" flex={1} height="full" paddingX={4}>
				<ErrorDisplay
					message={artifactStore.navTreeError}
					onRetry={() => artifactStore.loadNavTree()}
				/>
			</HStack>
		{:else}
			<Stack gap={6} padding={6}>
				<!-- Header -->
				<Stack gap={1}>
					<Heading level={1}>{config.label}</Heading>
					<Text variant="body-muted" block>{config.description}</Text>
					<Text variant="caption" block>
						Source: <code class="rounded bg-muted px-1 py-0.5">{config.location}</code>
					</Text>
				</Stack>

				{#if category === "rules" && (enforcementStore.blockCount > 0 || enforcementStore.warnCount > 0)}
					<HStack gap={2}>
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
					</HStack>
				{/if}

				{#if items.length === 0}
					<CardRoot>
						<CardContent>
							<Stack align="center" paddingY={8}>
								<Text variant="body-muted" block>
									No {config.label.toLowerCase()} found. Add files to <code class="rounded bg-muted px-1 py-0.5 text-xs">{config.location}</code> and re-scan.
								</Text>
							</Stack>
						</CardContent>
					</CardRoot>
				{:else}
					<!-- Summary -->
					<Text variant="body-muted" block>
						{items.length} {items.length === 1 ? config.singular : config.label.toLowerCase()} detected. Select one from the sidebar to view its contents.
					</Text>

					<!-- Card grid -->
					<Grid cols={1} sm={2} lg={3} gap={2}>
						{#each items as item (item.path)}
							<button
								class="h-auto w-full p-0 text-left rounded-md hover:bg-accent/50 transition-colors"
								onclick={() => item.path && handleItemClick(item.label, item.path)}
							>
								<CardRoot>
									<CardContent>
										<HStack gap={3} padding={4} align="start">
											<Icon name={config.icon} size="md" />
											<Box flex={1} minWidth={0}>
												<Text variant="label" truncate block>{item.label}</Text>
												{#if item.description}
													<Caption lineClamp={2} block>{item.description}</Caption>
												{/if}
											</Box>
											{#if category === "rules" && violationsByRule[item.label]}
												{@const counts = violationsByRule[item.label]}
												<HStack gap={1} flex={0}>
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
												</HStack>
											{/if}
										</HStack>
									</CardContent>
								</CardRoot>
							</button>
						{/each}
					</Grid>
				{/if}

				<!-- Violation history panel (rules category only) -->
				{#if category === "rules"}
					<Box marginTop={2} height="full" overflow="hidden" rounded="md" border>
						<ViolationsPanel
							violations={enforcementStore.violationHistory}
							loading={enforcementStore.historyLoading}
							error={enforcementStore.historyError}
							onRetry={() => enforcementStore.loadViolationHistory()}
						/>
					</Box>
				{/if}
			</Stack>
		{/if}
	</Stack>
{/if}

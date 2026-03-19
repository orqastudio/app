<script lang="ts">
	import { CollapsibleRoot as Collapsible, CollapsibleTrigger, CollapsibleContent } from "@orqastudio/svelte-components/pure";
	import { ScrollArea } from "@orqastudio/svelte-components/pure";
	import { EmptyState } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import { ArtifactListItem } from "@orqastudio/svelte-components/connected";
	import ArtifactToolbar from "$lib/components/navigation/ArtifactToolbar.svelte";
	import { getStores } from "@orqastudio/sdk";
	import type { ActivityView } from "@orqastudio/sdk";

	const { artifactStore, navigationStore } = getStores();
	import type { DocNode, ArtifactViewState, SortConfig } from "@orqastudio/types";
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { applyFilters, applySort, applyGrouping } from "$lib/utils/artifact-view";
	import { SvelteMap } from "svelte/reactivity";

	function resolveDirectoryIcon(iconName: string | null | undefined): string {
		return iconName ?? "folder";
	}

	let { category }: { category: ActivityView } = $props();

	/** View states keyed by category path (one per artifact type). */
	const viewStates = new SvelteMap<string, ArtifactViewState>();

	function getViewState(cat: string): ArtifactViewState {
		if (!viewStates.has(cat)) {
			// Initialize from navigation config defaults
			const navConfig = navigationStore.getNavType(cat as ActivityView)?.navigation_config;
			const defaults = navConfig?.defaults;
			viewStates.set(cat, {
				sort: defaults?.sort ?? { field: "title", direction: "asc" },
				filters: defaults?.filters ?? {},
				group: defaults?.group ?? null,
			});
		}
		return viewStates.get(cat)!;
	}

	/** Find the NavType for this category in the navTree. */
	const currentNavType = $derived(navigationStore.getNavType(category));

	/** Nodes for this category — either from navTree or empty. */
	const allNodes = $derived(currentNavType ? currentNavType.nodes : []);

	/** Label for this category. */
	const categoryLabel = $derived(
		currentNavType?.label ?? navigationStore.getLabelForKey(category),
	);

	/** Whether data is still loading. */
	const loading = $derived(artifactStore.navTreeLoading);

	/** Any error loading the tree. */
	const treeError = $derived(artifactStore.navTreeError);

	function isReadmePath(path: string | null): boolean {
		if (!path) return false;
		const p = path.replace(/\\/g, "/");
		const name = p.split("/").pop() ?? "";
		return name === "README" || name === "README.md";
	}

	/** All nodes from the navTree, with README filtered out. */
	const rawNodes = $derived(
		allNodes.filter((n) => !isReadmePath(n.path)),
	);

	/** Whether nodes form a tree (have children) or a flat list. */
	const isTree = $derived(rawNodes.some((n) => n.children !== null));

	// ---- View state (reactive, per category) ----

	let currentSort = $state<SortConfig>({ field: "title", direction: "asc" });
	let currentFilters = $state<Record<string, string[]>>({});
	let currentGroup = $state<string | null>(null);

	// When category changes, load the view state for it
	$effect(() => {
		const state = getViewState(category);
		currentSort = state.sort;
		currentFilters = state.filters;
		currentGroup = state.group;
	});

	function handleSortChange(sort: SortConfig) {
		currentSort = sort;
		const state = getViewState(category);
		state.sort = sort;
	}

	function handleFilterChange(filters: Record<string, string[]>) {
		currentFilters = filters;
		const state = getViewState(category);
		state.filters = filters;
	}

	function handleGroupChange(group: string | null) {
		currentGroup = group;
		const state = getViewState(category);
		state.group = group;
	}

	// ---- Processed nodes (filter → sort) ----

	const processedNodes = $derived.by(() => {
		if (isTree) return rawNodes;
		const filtered = applyFilters(rawNodes, currentFilters);
		return applySort(filtered, currentSort);
	});

	/** Grouped sections, only used when currentGroup is set. */
	const groupedNodes = $derived.by(() => {
		if (isTree || !currentGroup) return null;
		return applyGrouping(
			processedNodes,
			currentGroup,
			currentNavType?.navigation_config?.defaults?.group_order?.[currentGroup],
			currentNavType?.filterable_fields ?? [],
		);
	});

	// ---- Breadcrumb helpers ----

	function humanizeSegment(segment: string): string {
		return segment
			.split("-")
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(" ");
	}

	function buildBreadcrumbs(node: DocNode): string[] {
		const crumbs: string[] = [];

		// Add group label if in a group
		const group = navigationStore.activeGroup;
		if (group) {
			crumbs.push(navigationStore.getLabelForKey(group));
			// Only add type label if the group has multiple sub-categories.
			const groupChildren = navigationStore.getGroupChildren(group);
			if (groupChildren.length > 1) {
				crumbs.push(categoryLabel);
			}
		} else {
			crumbs.push(categoryLabel);
		}

		// Add folder hierarchy for tree items.
		if (isTree && node.path && currentNavType) {
			const typeRoot = currentNavType.path.replace(/\\/g, "/").replace(/\/$/, "");
			const normalizedPath = node.path.replace(/\\/g, "/");
			const relativePath = normalizedPath.startsWith(typeRoot + "/")
				? normalizedPath.slice(typeRoot.length + 1)
				: normalizedPath;
			const segments = relativePath.split("/");
			// All segments except the last are intermediate folders
			for (let i = 0; i < segments.length - 1; i++) {
				crumbs.push(humanizeSegment(segments[i]));
			}
		}

		// Add the item name
		crumbs.push(node.label);

		return crumbs;
	}

	function handleLeafClick(node: DocNode) {
		if (!node.path) return;
		navigationStore.openArtifact(node.path, buildBreadcrumbs(node));
	}

</script>

<div class="flex h-full flex-col">
	{#if !isTree}
		<ArtifactToolbar
			sortableFields={currentNavType?.sortable_fields ?? []}
			filterableFields={currentNavType?.filterable_fields ?? []}
			navigationConfig={currentNavType?.navigation_config}
			nodes={rawNodes}
			{currentSort}
			{currentFilters}
			{currentGroup}
			onSortChange={handleSortChange}
			onFilterChange={handleFilterChange}
			onGroupChange={handleGroupChange}
		/>
	{/if}

	<ScrollArea class="min-h-0 flex-1">
		<div class="p-1">
			{#if loading}
				<div class="flex items-center justify-center py-8">
					<LoadingSpinner />
				</div>
			{:else if treeError}
				<div class="px-2 py-4">
					<ErrorDisplay message={treeError} onRetry={() => artifactStore.loadNavTree()} />
				</div>
			{:else if rawNodes.length === 0}
				<div class="px-2 py-8">
					<EmptyState
						icon="file-text"
						title="No {categoryLabel.toLowerCase()} yet"
						description="No {categoryLabel.toLowerCase()} files found in this project."
					/>
				</div>
			{:else if processedNodes.length === 0}
				<div class="px-2 py-4 text-center text-xs text-muted-foreground">
					No matching items.
				</div>
			{:else if isTree}
				<div class="space-y-0.5 p-1">
					{#each processedNodes as node (node.path ?? node.label)}
						{@render treeSection(node, 0)}
					{/each}
				</div>
			{:else if groupedNodes !== null}
				{@const collapsedDefaults = currentNavType?.navigation_config?.defaults?.collapsed_groups ?? []}
				<div class="space-y-0.5">
					{#each groupedNodes as group (group.label)}
						<Collapsible open={!collapsedDefaults.includes(group.label.toLowerCase())}>
							<CollapsibleTrigger
								class="flex w-full items-center gap-1.5 rounded px-2 py-1.5 text-xs font-semibold uppercase tracking-wide text-muted-foreground hover:bg-accent/50"
							>
								<Icon name="chevron-right" size="xs" />
								{group.label}
								<span class="ml-auto text-[10px] font-normal tabular-nums">{group.nodes.length}</span>
							</CollapsibleTrigger>
							<CollapsibleContent>
								{#each group.nodes as node (node.path)}
									<ArtifactListItem
										label={node.label}
										description={node.description ?? undefined}
										status={node.status ?? undefined}
														path={node.path}
										onclick={() => handleLeafClick(node)}
									/>
								{/each}
							</CollapsibleContent>
						</Collapsible>
					{/each}
				</div>
			{:else}
				{#each processedNodes as node (node.path)}
					<ArtifactListItem
						label={node.label}
						description={node.description ?? undefined}
						status={node.status ?? undefined}
						path={node.path}
						onclick={() => handleLeafClick(node)}
					/>
				{/each}
			{/if}
		</div>
	</ScrollArea>
</div>

{#snippet treeSection(node: DocNode, depth: number)}
	{#if node.children}
		{@const dirIconName = resolveDirectoryIcon(node.icon)}
		<Collapsible open={true}>
			<CollapsibleTrigger
				class="flex w-full items-center gap-1 rounded px-1 py-1 text-xs font-semibold uppercase tracking-wide text-muted-foreground hover:bg-accent/50"
				style="padding-left: {depth * 12 + 4}px"
			>
				<Icon name="chevron-right" size="xs" />
				<Icon name={dirIconName} size="xs" />
				{node.label}
			</CollapsibleTrigger>
			<CollapsibleContent>
				{#each node.children as child (child.path ?? child.label)}
					{@render treeSection(child, depth + 1)}
				{/each}
			</CollapsibleContent>
		</Collapsible>
	{:else if node.path}
		<div style="padding-left: {depth * 12}px">
			<ArtifactListItem
				label={node.label}
				description={node.description ?? undefined}
				status={node.status ?? undefined}
				path={node.path}
				onclick={() => handleLeafClick(node)}
			/>
		</div>
	{/if}
{/snippet}

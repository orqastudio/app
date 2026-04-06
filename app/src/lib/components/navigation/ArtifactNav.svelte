<script lang="ts">
	import {
		CollapsibleRoot as Collapsible,
		CollapsibleTrigger,
		CollapsibleContent,
	} from "@orqastudio/svelte-components/pure";
	import { EmptyState } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import {
		Caption,
		Stack,
		Box,
		Center,
		Panel,
		ScrollArea,
	} from "@orqastudio/svelte-components/pure";
	import { ArtifactListItem } from "@orqastudio/svelte-components/connected";
	import ArtifactToolbar from "$lib/components/navigation/ArtifactToolbar.svelte";
	import { getStores } from "@orqastudio/sdk";
	import type { ActivityView } from "@orqastudio/sdk";

	const { artifactStore, navigationStore } = getStores();
	import type { DocNode, ArtifactViewState, SortConfig } from "@orqastudio/types";
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { applyFilters, applySort, applyGrouping } from "$lib/utils/artifact-view";
	import { SvelteMap } from "svelte/reactivity";

	/**
	 * Return the icon name for a directory node, falling back to "folder".
	 * @param iconName
	 */
	function resolveDirectoryIcon(iconName: string | null | undefined): string {
		return iconName ?? "folder";
	}

	let { category }: { category: ActivityView } = $props();

	/** View states keyed by category path (one per artifact type). */
	const viewStates = new SvelteMap<string, ArtifactViewState>();

	/**
	 * Return the persisted view state for a category, initializing from nav config defaults if absent.
	 * @param cat
	 */
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
	const categoryLabel = $derived(currentNavType?.label ?? navigationStore.getLabelForKey(category));

	/** Whether data is still loading. */
	const loading = $derived(artifactStore.navTreeLoading);

	/** Any error loading the tree. */
	const treeError = $derived(artifactStore.navTreeError);

	/**
	 * Return true when a path refers to a README file (excluded from artifact lists).
	 * @param path
	 */
	function isReadmePath(path: string | null): boolean {
		if (!path) return false;
		const p = path.replace(/\\/g, "/");
		const name = p.split("/").pop() ?? "";
		return name === "README" || name === "README.md";
	}

	/** All nodes from the navTree, with README filtered out. */
	const rawNodes = $derived(allNodes.filter((n) => !isReadmePath(n.path)));

	/** Whether nodes form a tree (have children) or a flat list. */
	const isTree = $derived(rawNodes.some((n) => n.children !== null));

	// ---- View state (reactive, per category) ----

	let currentSort = $state<SortConfig>({ field: "title", direction: "asc" });
	let currentFilters = $state<Readonly<Record<string, readonly string[]>>>({});
	let currentGroup = $state<string | null>(null);

	// When category changes, load the view state for it
	$effect(() => {
		const state = getViewState(category);
		currentSort = state.sort;
		currentFilters = state.filters;
		currentGroup = state.group;
	});

	/**
	 * Persist the new sort config and update reactive sort state.
	 * @param sort
	 */
	function handleSortChange(sort: SortConfig) {
		currentSort = sort;
		const state = getViewState(category);
		viewStates.set(category, { ...state, sort });
	}

	/**
	 *
	 * @param filters
	 */
	function handleFilterChange(filters: Record<string, readonly string[]>) {
		currentFilters = filters;
		const state = getViewState(category);
		viewStates.set(category, { ...state, filters });
	}

	/**
	 *
	 * @param group
	 */
	function handleGroupChange(group: string | null) {
		currentGroup = group;
		const state = getViewState(category);
		viewStates.set(category, { ...state, group });
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

	/**
	 *
	 * @param segment
	 */
	function humanizeSegment(segment: string): string {
		return segment
			.split("-")
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(" ");
	}

	/**
	 *
	 * @param node
	 */
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

	/**
	 *
	 * @param node
	 */
	function handleLeafClick(node: DocNode) {
		if (!node.path) return;
		navigationStore.openArtifact(node.path, buildBreadcrumbs(node));
	}
</script>

<Stack gap={0} height="full">
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

	<Box minHeight={0} flex={1}>
		<ScrollArea>
			<Panel padding="tight">
				{#if loading}
					<Panel padding="normal"
						><Center full>
							<LoadingSpinner />
						</Center></Panel
					>
				{:else if treeError}
					<Panel padding="normal">
						<ErrorDisplay message={treeError} onRetry={() => artifactStore.loadNavTree()} />
					</Panel>
				{:else if rawNodes.length === 0}
					<Panel padding="normal">
						<EmptyState
							icon="file-text"
							title="No {categoryLabel.toLowerCase()} yet"
							description="No {categoryLabel.toLowerCase()} files found in this project."
						/>
					</Panel>
				{:else if processedNodes.length === 0}
					<Panel padding="tight"
						><Center full>
							<Caption>No matching items.</Caption>
						</Center></Panel
					>
				{:else if isTree}
					<Stack gap={0}>
						{#each processedNodes as node (node.path ?? node.label)}
							{@render treeSection(node, 0)}
						{/each}
					</Stack>
				{:else if groupedNodes !== null}
					{@const collapsedDefaults =
						currentNavType?.navigation_config?.defaults?.collapsed_groups ?? []}
					<Stack gap={0}>
						{#each groupedNodes as group (group.label)}
							<Collapsible open={!collapsedDefaults.includes(group.label.toLowerCase())}>
								<CollapsibleTrigger
									class="text-muted-foreground hover:bg-accent/50 flex w-full items-center gap-1.5 rounded px-2 py-1.5 text-xs font-semibold tracking-wide uppercase"
								>
									<Icon name="chevron-right" size="xs" />
									{group.label}
									<span class="ml-auto text-[10px] font-normal tabular-nums"
										>{group.nodes.length}</span
									>
								</CollapsibleTrigger>
								<CollapsibleContent>
									{#each group.nodes as node (node.path)}
										<ArtifactListItem
											label={node.label}
											description={node.description ?? undefined}
											status={node.status ?? undefined}
											path={node.path ?? undefined}
											onclick={() => handleLeafClick(node)}
										/>
									{/each}
								</CollapsibleContent>
							</Collapsible>
						{/each}
					</Stack>
				{:else}
					{#each processedNodes as node (node.path)}
						<ArtifactListItem
							label={node.label}
							description={node.description ?? undefined}
							status={node.status ?? undefined}
							path={node.path ?? undefined}
							onclick={() => handleLeafClick(node)}
						/>
					{/each}
				{/if}
			</Panel>
		</ScrollArea>
	</Box>
</Stack>

{#snippet treeSection(node: DocNode, depth: number)}
	{#if node.children}
		{@const dirIconName = resolveDirectoryIcon(node.icon)}
		<Collapsible open={true}>
			<CollapsibleTrigger
				class="text-muted-foreground hover:bg-accent/50 flex w-full items-center gap-1 rounded px-1 py-1 text-xs font-semibold tracking-wide uppercase"
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
		<!-- Dynamic pixel indentation cannot be expressed via Box props; style attribute retained -->
		<div style="padding-left: {depth * 12}px">
			<ArtifactListItem
				label={node.label}
				description={node.description ?? undefined}
				status={node.status ?? undefined}
				path={node.path ?? undefined}
				onclick={() => handleLeafClick(node)}
			/>
		</div>
	{/if}
{/snippet}

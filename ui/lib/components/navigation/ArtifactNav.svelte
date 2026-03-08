<script lang="ts">
	import * as Collapsible from "$lib/components/ui/collapsible";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import TargetIcon from "@lucide/svelte/icons/target";
	import LayersIcon from "@lucide/svelte/icons/layers";
	import CheckSquareIcon from "@lucide/svelte/icons/check-square";
	import LightbulbIcon from "@lucide/svelte/icons/lightbulb";
	import ScrollTextIcon from "@lucide/svelte/icons/scroll-text";
	import BookOpenIcon from "@lucide/svelte/icons/book-open";
	import BotIcon from "@lucide/svelte/icons/bot";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import FlaskConicalIcon from "@lucide/svelte/icons/flask-conical";
	import ClipboardListIcon from "@lucide/svelte/icons/clipboard-list";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import SearchInput from "$lib/components/shared/SearchInput.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { enforcementStore } from "$lib/stores/enforcement.svelte";
	import {
		navigationStore,
		SUB_CATEGORY_LABELS,
		GROUP_SUB_CATEGORIES,
		type ActivityView,
		type ActivityGroup,
	} from "$lib/stores/navigation.svelte";
	import type { ArtifactSummary, ArtifactType, DocNode } from "$lib/types";
	import type { Component } from "svelte";

	const GROUP_DISPLAY_LABELS: Record<ActivityGroup, string> = {
		documentation: "Docs",
		planning: "Planning",
		team: "Team",
		governance: "Governance",
	};

	let { category }: { category: ActivityView } = $props();

	let filterText = $state("");

	interface NavConfig {
		label: string;
		emptyTitle: string;
		emptyDescription: string;
		emptyIcon: Component;
		getNodes: () => DocNode[];
		isLoading: () => boolean;
		onRetry?: () => void;
		treeError?: () => string | null;
	}

	function flatToNodes(items: ArtifactSummary[]): DocNode[] {
		return items.map((item) => ({
			label: item.name,
			path: item.rel_path,
			children: null,
			description: item.description ?? null,
		}));
	}

	const categoryConfig: Record<string, NavConfig> = {
		milestones: {
			label: "Milestones",
			emptyTitle: "No milestones yet",
			emptyDescription: "Milestones define strategic goals and gate questions for the project.",
			emptyIcon: TargetIcon,
			getNodes: () => flatToNodes(artifactStore.milestones),
			isLoading: () => artifactStore.milestonesLoading,
		},
		epics: {
			label: "Epics",
			emptyTitle: "No epics yet",
			emptyDescription: "Epics are trackable work units that group related tasks together.",
			emptyIcon: LayersIcon,
			getNodes: () => flatToNodes(artifactStore.epics),
			isLoading: () => artifactStore.epicsLoading,
		},
		tasks: {
			label: "Tasks",
			emptyTitle: "No tasks yet",
			emptyDescription: "Tasks are scoped work items within an epic.",
			emptyIcon: CheckSquareIcon,
			getNodes: () => flatToNodes(artifactStore.tasks),
			isLoading: () => artifactStore.tasksLoading,
		},
		ideas: {
			label: "Ideas",
			emptyTitle: "No ideas captured yet",
			emptyDescription: "Ideas are candidate features that need research and validation before promotion.",
			emptyIcon: LightbulbIcon,
			getNodes: () => flatToNodes(artifactStore.ideas),
			isLoading: () => artifactStore.ideasLoading,
		},
		decisions: {
			label: "Decisions",
			emptyTitle: "No decisions recorded yet",
			emptyDescription: "Architecture decisions capture why key choices were made.",
			emptyIcon: ScrollTextIcon,
			getNodes: () => flatToNodes(artifactStore.decisions),
			isLoading: () => artifactStore.decisionsLoading,
		},
		lessons: {
			label: "Lessons",
			emptyTitle: "No lessons captured yet",
			emptyDescription: "Lessons record implementation discoveries and prevent recurring mistakes.",
			emptyIcon: BookOpenIcon,
			getNodes: () => flatToNodes(artifactStore.lessons),
			isLoading: () => artifactStore.lessonsLoading,
		},
		agents: {
			label: "Agents",
			emptyTitle: "No agents yet",
			emptyDescription:
				"Agents define AI personas with specialized knowledge and behavior. Create your first agent to customize how Claude works on your project.",
			emptyIcon: BotIcon,
			getNodes: () => flatToNodes(artifactStore.artifactsByType("agent" as ArtifactType)),
			isLoading: () => artifactStore.loading,
			onRetry: () => artifactStore.loadGovernanceList("agent"),
		},
		rules: {
			label: "Rules",
			emptyTitle: "No rules yet",
			emptyDescription:
				"Rules enforce coding standards and project conventions. They are automatically applied based on file path globs.",
			emptyIcon: ShieldIcon,
			getNodes: () => flatToNodes(artifactStore.artifactsByType("rule" as ArtifactType)),
			isLoading: () => artifactStore.loading,
			onRetry: () => artifactStore.loadGovernanceList("rule"),
		},
		skills: {
			label: "Skills",
			emptyTitle: "No skills yet",
			emptyDescription:
				"Skills define reusable capabilities that agents can invoke during sessions. Create your first skill to get started.",
			emptyIcon: ZapIcon,
			getNodes: () => flatToNodes(artifactStore.artifactsByType("skill" as ArtifactType)),
			isLoading: () => artifactStore.loading,
			onRetry: () => artifactStore.loadGovernanceList("skill"),
		},
		hooks: {
			label: "Hooks",
			emptyTitle: "No hooks yet",
			emptyDescription:
				"Hooks include lifecycle hooks that run automated actions before or after AI operations, and hookify enforcement rules.",
			emptyIcon: GitBranchIcon,
			getNodes: () => flatToNodes(artifactStore.artifactsByType("hook" as ArtifactType)),
			isLoading: () => artifactStore.loading,
			onRetry: () => artifactStore.loadGovernanceList("hook"),
		},
		docs: {
			label: "Docs",
			emptyTitle: "No documentation found.",
			emptyDescription: "",
			emptyIcon: FileTextIcon,
			getNodes: () => artifactStore.docTree,
			isLoading: () => artifactStore.docTreeLoading,
			treeError: () => artifactStore.docTreeError,
			onRetry: () => artifactStore.loadDocTree(),
		},
		research: {
			label: "Research",
			emptyTitle: "No research docs found.",
			emptyDescription: "",
			emptyIcon: FlaskConicalIcon,
			getNodes: () => artifactStore.researchTree,
			isLoading: () => artifactStore.researchTreeLoading,
			treeError: () => artifactStore.researchTreeError,
			onRetry: () => artifactStore.loadResearchTree(),
		},
		plans: {
			label: "Plans",
			emptyTitle: "No plans found.",
			emptyDescription: "",
			emptyIcon: ClipboardListIcon,
			getNodes: () => artifactStore.planTree,
			isLoading: () => artifactStore.planTreeLoading,
			treeError: () => artifactStore.planTreeError,
			onRetry: () => artifactStore.loadPlanTree(),
		},
	};

	/** Tree types use hierarchical rendering and have their own error fields. */
	const TREE_CATEGORIES = new Set(["docs", "research", "plans"]);

	const config = $derived(categoryConfig[category]);
	const isTree = $derived(TREE_CATEGORIES.has(category));

	/** All nodes from the config, with README filtered out for all categories. */
	const rawNodes = $derived(
		config
			? config.getNodes().filter((n) => !isReadmePath(n.path))
			: [],
	);

	const loading = $derived(config ? config.isLoading() : false);
	const treeError = $derived(config?.treeError ? config.treeError() : null);

	function isReadmePath(path: string | null): boolean {
		if (!path) return false;
		const p = path.replace(/\\/g, "/");
		const name = p.split("/").pop() ?? "";
		return name === "README" || name === "README.md";
	}

	// ---- Filtering ----

	function filterTree(nodes: DocNode[], query: string): DocNode[] {
		if (!query) return nodes;
		const q = query.toLowerCase();
		const result: DocNode[] = [];
		for (const node of nodes) {
			if (node.children) {
				const filteredChildren = filterTree(node.children, query);
				if (filteredChildren.length > 0) {
					result.push({ ...node, children: filteredChildren });
				} else if (node.label.toLowerCase().includes(q)) {
					result.push(node);
				}
			} else if (
				node.label.toLowerCase().includes(q) ||
				(node.description?.toLowerCase().includes(q) ?? false)
			) {
				result.push(node);
			}
		}
		return result;
	}

	const filteredNodes = $derived(filterTree(rawNodes, filterText));

	// ---- Breadcrumb helpers ----

	function humanizeSegment(segment: string): string {
		return segment
			.split("-")
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(" ");
	}

	function breadcrumbsForTreePath(path: string): string[] {
		return path.split("/").map(humanizeSegment);
	}

	function buildBreadcrumbs(node: DocNode): string[] {
		const crumbs: string[] = [];

		// Add group label if in a group
		const group = navigationStore.activeGroup;
		if (group) {
			crumbs.push(GROUP_DISPLAY_LABELS[group]);
			// Only add type label if the group has multiple sub-categories.
			// When a group has exactly one sub-category, the type label IS the
			// group label — adding both produces duplicates like "Docs > Docs".
			const subCategories = GROUP_SUB_CATEGORIES[group];
			if (subCategories.length > 1) {
				crumbs.push(SUB_CATEGORY_LABELS[category]);
			}
		} else {
			crumbs.push(SUB_CATEGORY_LABELS[category]);
		}

		// Add folder hierarchy for tree items
		if (isTree && node.path) {
			const segments = node.path.split("/");
			// All segments except the last are folders
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

	// ---- Rules violation dots ----

	const rulesWithViolations = $derived(
		new Set(enforcementStore.violations.map((v) => v.rule_name)),
	);

	// ---- Cross-link auto-select ----

	$effect(() => {
		const pendingId = navigationStore.pendingArtifactId;
		if (!pendingId || rawNodes.length === 0 || isTree) return;
		const match = rawNodes.find((n) => n.label.startsWith(pendingId));
		if (match?.path) {
			navigationStore.pendingArtifactId = null;
			navigationStore.openArtifact(match.path, [match.label]);
		}
	});
</script>

{#if config}
	<div class="flex h-full flex-col">
		<div class="border-b border-border p-2">
			<SearchInput
				bind:value={filterText}
				placeholder="Filter {config.label.toLowerCase()}..."
				size="xs"
			/>
		</div>

		<ScrollArea.Root class="min-h-0 flex-1">
			<div class="p-1">
				{#if loading}
					<div class="flex items-center justify-center py-8">
						<LoadingSpinner />
					</div>
				{:else if treeError}
					<div class="px-2 py-4">
						<ErrorDisplay message={treeError} onRetry={config.onRetry} />
					</div>
				{:else if artifactStore.error && !isTree}
					<div class="px-2 py-4">
						<ErrorDisplay message={artifactStore.error} onRetry={config.onRetry} />
					</div>
				{:else if rawNodes.length === 0}
					<div class="px-2 py-8">
						<EmptyState
							icon={config.emptyIcon}
							title={config.emptyTitle}
							description={config.emptyDescription}
						/>
					</div>
				{:else if filteredNodes.length === 0}
					<div class="px-2 py-4 text-center text-xs text-muted-foreground">
						No matching items.
					</div>
				{:else if isTree}
					<div class="space-y-0.5 p-1">
						{#each filteredNodes as node (node.path ?? node.label)}
							{@render treeSection(node, 0)}
						{/each}
					</div>
				{:else}
					{#each filteredNodes as node (node.path)}
						<button
							class="flex w-full flex-col gap-0.5 rounded px-2 py-1.5 text-left hover:bg-accent/50"
							class:bg-accent={navigationStore.selectedArtifactPath === node.path}
							onclick={() => handleLeafClick(node)}
						>
							<span class="flex items-center gap-1.5 truncate text-sm font-medium">
								{node.label}
								{#if category === "rules" && rulesWithViolations.has(node.label)}
									<span
										class="inline-block h-2 w-2 shrink-0 rounded-full bg-destructive"
										title="Has violations"
									></span>
								{/if}
							</span>
							{#if node.description}
								<p class="line-clamp-2 text-xs text-muted-foreground">{node.description}</p>
							{/if}
						</button>
					{/each}
				{/if}
			</div>
		</ScrollArea.Root>
	</div>
{/if}

{#snippet treeSection(node: DocNode, depth: number)}
	{#if node.children}
		<Collapsible.Root open={true}>
			<Collapsible.Trigger
				class="flex w-full items-center gap-1 rounded px-1 py-1 text-xs font-semibold uppercase tracking-wide text-muted-foreground hover:bg-accent/50"
				style="padding-left: {depth * 12 + 4}px"
			>
				<ChevronRightIcon class="h-3 w-3 transition-transform [[data-state=open]_&]:rotate-90" />
				{node.label}
			</Collapsible.Trigger>
			<Collapsible.Content>
				{#each node.children as child (child.path ?? child.label)}
					{@render treeSection(child, depth + 1)}
				{/each}
			</Collapsible.Content>
		</Collapsible.Root>
	{:else if node.path}
		<button
			class="flex w-full flex-col gap-0.5 rounded px-2 py-1.5 text-left hover:bg-accent/50"
			class:bg-accent={navigationStore.selectedArtifactPath === node.path}
			style="padding-left: {depth * 12 + 8}px"
			onclick={() => handleLeafClick(node)}
		>
			<span class="truncate text-sm font-medium">{node.label}</span>
			{#if node.description}
				<p class="line-clamp-2 text-xs text-muted-foreground">{node.description}</p>
			{/if}
		</button>
	{/if}
{/snippet}

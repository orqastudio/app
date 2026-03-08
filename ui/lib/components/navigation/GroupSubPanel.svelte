<script lang="ts">
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import FlaskConicalIcon from "@lucide/svelte/icons/flask-conical";
	import ClipboardListIcon from "@lucide/svelte/icons/clipboard-list";
	import BotIcon from "@lucide/svelte/icons/bot";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import UsersIcon from "@lucide/svelte/icons/users";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import BookOpenIcon from "@lucide/svelte/icons/book-open";
	import TargetIcon from "@lucide/svelte/icons/target";
	import LayersIcon from "@lucide/svelte/icons/layers";
	import CheckSquareIcon from "@lucide/svelte/icons/check-square";
	import LightbulbIcon from "@lucide/svelte/icons/lightbulb";
	import ScrollTextIcon from "@lucide/svelte/icons/scroll-text";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import DocTreeNav from "./DocTreeNav.svelte";
	import ArtifactListNav from "./ArtifactListNav.svelte";
	import {
		navigationStore,
		GROUP_SUB_CATEGORIES,
		SUB_CATEGORY_LABELS,
		COMING_SOON_ACTIVITIES,
		type ActivityGroup,
		type ActivityView,
	} from "$lib/stores/navigation.svelte";
	import type { Component } from "svelte";

	let { group }: { group: ActivityGroup } = $props();

	const subCategoryIcons: Record<ActivityView, Component> = {
		chat: UsersIcon,
		project: LayersIcon,
		docs: FileTextIcon,
		research: FlaskConicalIcon,
		plans: ClipboardListIcon,
		milestones: TargetIcon,
		epics: LayersIcon,
		tasks: CheckSquareIcon,
		ideas: LightbulbIcon,
		agents: BotIcon,
		skills: ZapIcon,
		orchestrator: UsersIcon,
		rules: ShieldIcon,
		hooks: GitBranchIcon,
		lessons: BookOpenIcon,
		decisions: ScrollTextIcon,
		settings: ShieldIcon,
		configure: ShieldIcon,
	};

	const subCategories = $derived(GROUP_SUB_CATEGORIES[group]);
	const activeSubCategory = $derived(navigationStore.activeSubCategory);

	/** Sub-categories that support DocTreeNav */
	const DOC_TREE_MODES: Partial<Record<ActivityView, "docs" | "research" | "plans">> = {
		docs: "docs",
		research: "research",
		plans: "plans",
	};

	/** Sub-categories that support ArtifactListNav */
	const ARTIFACT_LIST_CATEGORIES: ActivityView[] = ["agents", "rules", "skills", "hooks", "lessons"];
</script>

<div class="flex h-full flex-col">
	<!-- Sub-category buttons -->
	<div class="border-b border-border">
		{#each subCategories as subKey (subKey)}
			{@const SubIcon = subCategoryIcons[subKey]}
			{@const isActive = activeSubCategory === subKey}
			<Tooltip.Root>
				<Tooltip.Trigger class="w-full">
					{#snippet child({ props })}
						<button
							{...props}
							class="flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors
								{isActive
								? 'bg-accent text-accent-foreground font-medium'
								: 'text-muted-foreground hover:bg-accent/40 hover:text-foreground'}"
							onclick={() => navigationStore.setSubCategory(subKey)}
						>
							<SubIcon class="h-4 w-4 shrink-0" />
							<span class="truncate">{SUB_CATEGORY_LABELS[subKey]}</span>
						</button>
					{/snippet}
				</Tooltip.Trigger>
			</Tooltip.Root>
		{/each}
	</div>

	<!-- Content area for the selected sub-category -->
	<div class="min-h-0 flex-1 overflow-hidden">
		{#if activeSubCategory === null}
			<div class="flex h-full items-center justify-center p-4 text-center text-xs text-muted-foreground">
				Select a category above.
			</div>
		{:else if COMING_SOON_ACTIVITIES.includes(activeSubCategory)}
			<div class="px-3 py-6">
				<EmptyState
					title="Coming soon"
					description="Backend readers for this category are not yet available."
				/>
			</div>
		{:else if activeSubCategory === "orchestrator"}
			<div class="p-2">
				<button
					class="flex w-full items-center gap-2 rounded px-2 py-2 text-left text-sm transition-colors
						{navigationStore.selectedArtifactPath === 'orchestrator'
						? 'bg-accent text-accent-foreground'
						: 'text-foreground/80 hover:bg-accent/50'}"
					onclick={() => navigationStore.openArtifact("orchestrator", ["Orchestrator"])}
				>
					<UsersIcon class="h-4 w-4 shrink-0 text-muted-foreground" />
					<span class="truncate">orchestrator.md</span>
				</button>
			</div>
		{:else if DOC_TREE_MODES[activeSubCategory] !== undefined}
			<DocTreeNav mode={DOC_TREE_MODES[activeSubCategory]!} />
		{:else if ARTIFACT_LIST_CATEGORIES.includes(activeSubCategory)}
			<ArtifactListNav category={activeSubCategory} />
		{:else}
			<div class="flex h-full items-center justify-center p-4 text-center text-xs text-muted-foreground">
				No content available.
			</div>
		{/if}
	</div>
</div>

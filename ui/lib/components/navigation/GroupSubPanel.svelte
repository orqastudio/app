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
	import {
		navigationStore,
		GROUP_SUB_CATEGORIES,
		SUB_CATEGORY_LABELS,
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
</script>

<div class="flex flex-col">
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

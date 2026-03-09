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
	import FolderIcon from "@lucide/svelte/icons/folder";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import type { Component } from "svelte";

	let { group }: { group: string } = $props();

	/** Map from icon name strings to Lucide icon components. */
	const ICON_MAP: Record<string, Component> = {
		"file-text": FileTextIcon,
		"flask-conical": FlaskConicalIcon,
		"clipboard-list": ClipboardListIcon,
		bot: BotIcon,
		zap: ZapIcon,
		users: UsersIcon,
		shield: ShieldIcon,
		"git-branch": GitBranchIcon,
		"book-open": BookOpenIcon,
		target: TargetIcon,
		layers: LayersIcon,
		"check-square": CheckSquareIcon,
		lightbulb: LightbulbIcon,
		"scroll-text": ScrollTextIcon,
		folder: FolderIcon,
	};

	/** Fallback icon map keyed by artifact type key (for projects without icon in config). */
	const FALLBACK_ICONS: Record<string, Component> = {
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
	};

	function resolveIcon(key: string, iconName: string | undefined): Component {
		if (iconName && iconName in ICON_MAP) {
			return ICON_MAP[iconName];
		}
		if (key in FALLBACK_ICONS) {
			return FALLBACK_ICONS[key];
		}
		return FolderIcon;
	}

	// Use the store getter which derives from artifact config
	const subCategories = $derived(navigationStore.getGroupChildren(group));
	const activeSubCategory = $derived(navigationStore.activeSubCategory);
</script>

<div class="flex flex-col">
	{#each subCategories as sub (sub.key)}
		{@const SubIcon = resolveIcon(sub.key, undefined)}
		{@const isActive = activeSubCategory === sub.key}
		<Tooltip.Root>
			<Tooltip.Trigger class="w-full">
				{#snippet child({ props })}
					<button
						{...props}
						class="flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors
							{isActive
							? 'bg-accent text-accent-foreground font-medium'
							: 'text-muted-foreground hover:bg-accent/40 hover:text-foreground'}"
						onclick={() => navigationStore.setSubCategory(sub.key)}
					>
						<SubIcon class="h-4 w-4 shrink-0" />
						<span class="truncate">{sub.label}</span>
					</button>
				{/snippet}
			</Tooltip.Trigger>
		</Tooltip.Root>
	{/each}
</div>

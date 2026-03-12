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
	import CompassIcon from "@lucide/svelte/icons/compass";
	import CodeIcon from "@lucide/svelte/icons/code";
	import LayoutIcon from "@lucide/svelte/icons/layout";
	import PaletteIcon from "@lucide/svelte/icons/palette";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { isArtifactGroup } from "$lib/types/project";
	import type { Component } from "svelte";

	let { group }: { group: string } = $props();

	/** Map from icon name strings (as stored in config / navTree) to Lucide icon components. */
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
		compass: CompassIcon,
		code: CodeIcon,
		layout: LayoutIcon,
		palette: PaletteIcon,
	};

	function resolveIcon(iconName: string | undefined): Component {
		if (iconName && iconName in ICON_MAP) {
			return ICON_MAP[iconName];
		}
		return FolderIcon;
	}

	/**
	 * Look up the icon for a sub-category by matching its config path against navTree types.
	 * Priority: config icon → navTree icon → undefined (caller falls back to FolderIcon).
	 */
	function getSubCategoryIcon(subKey: string): string | undefined {
		const config = projectStore.artifactConfig;
		for (const entry of config) {
			if (isArtifactGroup(entry)) {
				for (const child of entry.children) {
					if (child.key === subKey) {
						if (child.icon) return child.icon;
						const tree = artifactStore.navTree;
						if (!tree) return undefined;
						for (const group of tree.groups) {
							for (const type of group.types) {
								if (type.path === child.path) return type.icon;
							}
						}
					}
				}
			}
		}
		return undefined;
	}

	// Use the store getter which derives from artifact config
	const subCategories = $derived(navigationStore.getGroupChildren(group));
	const activeSubCategory = $derived(navigationStore.activeSubCategory);
</script>

<div class="flex flex-col">
	{#each subCategories as sub (sub.key)}
		{@const SubIcon = resolveIcon(getSubCategoryIcon(sub.key))}
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

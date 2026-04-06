<script lang="ts">
	import { TooltipRoot, TooltipTrigger, Stack, NavItem } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { navigationStore, projectStore, artifactStore } = getStores();
	import { isArtifactGroup } from "@orqastudio/types";
	import { resolveIcon } from "@orqastudio/svelte-components/pure";

	let { group }: { group: string } = $props();

	/**
	 * Return the provided icon name, or "folder" if none is defined.
	 * @param iconName - The icon name from a sub-category config entry, or undefined if absent.
	 * @returns The resolved icon name to pass to the Icon component.
	 */
	function resolveIconName(iconName: string | undefined): string {
		return iconName ?? "folder";
	}

	/**
	 * Look up the icon for a sub-category by matching its config path against navTree types.
	 * Priority: config icon → navTree icon → undefined (caller falls back to FolderIcon).
	 * @param subKey - The sub-category key to resolve an icon for (e.g. "task", "epic").
	 * @returns The icon name string, or undefined if no icon is configured or found in the navTree.
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

	// Use the store getter which derives from artifact config or navigation tree
	const subCategories = $derived(navigationStore.getGroupChildren(group));
	const activeSubCategory = $derived(navigationStore.activeSubCategory);
</script>

<Stack gap={0}>
	{#each subCategories as sub (sub.key)}
		{@const subIconName = resolveIconName(sub.icon ?? getSubCategoryIcon(sub.key))}
		{@const isActive = activeSubCategory === sub.key}
		<TooltipRoot>
			<TooltipTrigger full>
				{#snippet child({ props })}
					<NavItem
						{...props}
						icon={resolveIcon(subIconName)}
						label={sub.label}
						active={isActive}
						onclick={() => navigationStore.setSubCategory(sub.key)}
					/>
				{/snippet}
			</TooltipTrigger>
		</TooltipRoot>
	{/each}
</Stack>

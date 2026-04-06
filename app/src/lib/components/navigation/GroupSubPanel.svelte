<script lang="ts">
	import { TooltipRoot, TooltipTrigger, Stack } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { navigationStore, projectStore, artifactStore } = getStores();
	import { isArtifactGroup } from "@orqastudio/types";
	import { Icon } from "@orqastudio/svelte-components/pure";

	let { group }: { group: string } = $props();

	/**
	 *
	 * @param iconName
	 */
	function resolveIconName(iconName: string | undefined): string {
		return iconName ?? "folder";
	}

	/**
	 * Look up the icon for a sub-category by matching its config path against navTree types.
	 * Priority: config icon → navTree icon → undefined (caller falls back to FolderIcon).
	 * @param subKey
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
			<TooltipTrigger class="w-full">
				{#snippet child({ props })}
					<button
						{...props}
						class="flex w-full items-center justify-start gap-2 px-3 py-2 text-sm {isActive
							? 'bg-accent text-accent-foreground font-medium'
							: 'text-muted-foreground hover:bg-accent/40 hover:text-foreground'}"
						onclick={() => navigationStore.setSubCategory(sub.key)}
					>
						<Icon name={subIconName} size="md" />
						<span class="truncate">{sub.label}</span>
					</button>
				{/snippet}
			</TooltipTrigger>
		</TooltipRoot>
	{/each}
</Stack>

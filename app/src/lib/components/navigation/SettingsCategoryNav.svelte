<!-- Settings category navigation with architecture-aligned section groups. -->
<script lang="ts">
	import { Icon, ScrollArea } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { settingsStore, pluginRegistry } = getStores();

	interface SettingsItem {
		id: string;
		label: string;
		icon: string;
		description: string;
	}

	interface SettingsGroup {
		label: string;
		items: SettingsItem[];
	}

	interface Props {
		mode: "app" | "project";
		activeSection?: string;
		onSectionChange?: (section: string) => void;
	}

	const { mode, activeSection, onSectionChange }: Props = $props();

	const currentSection = $derived(activeSection ?? settingsStore.activeSection);

	/**
	 * Handles section selection, delegating to the onSectionChange prop or the settings store.
	 * @param id - The section identifier to activate.
	 */
	function handleSectionChange(id: string) {
		if (onSectionChange) {
			onSectionChange(id);
		} else {
			settingsStore.setActiveSection(id);
		}
	}

	// App-level settings (not grouped — these are global, not project-scoped)
	const appCategories: SettingsItem[] = [
		{
			id: "provider",
			label: "Provider",
			icon: "monitor",
			description: "Sidecar status, CLI path",
		},
		{
			id: "model",
			label: "Model",
			icon: "brain",
			description: "Default Claude model",
		},
		{
			id: "appearance",
			label: "Appearance",
			icon: "palette",
			description: "Theme, font size",
		},
		{
			id: "shortcuts",
			label: "Keyboard Shortcuts",
			icon: "keyboard",
			description: "Shortcut reference",
		},
	];

	// Project settings grouped by architecture section (Methodology / Sidecar / Connector / Plugins).
	// Plugin-contributed settings pages are appended to the matching section group, or
	// to a new group if the section doesn't match any built-in group label.
	const projectGroups: SettingsGroup[] = $derived.by(() => {
		const baseGroups: SettingsGroup[] = [
			{
				label: "Methodology",
				items: [
					{
						id: "project-general",
						label: "General",
						icon: "settings",
						description: "Name, icon, description",
					},
					{
						id: "project-status",
						label: "Status Machine",
						icon: "workflow",
						description: "Statuses, transitions, auto rules",
					},
					{
						id: "project-relationships",
						label: "Relationships",
						icon: "git-branch",
						description: "Canonical and plugin relationships",
					},
					{
						id: "project-artifact-links",
						label: "Artifact Links",
						icon: "link",
						description: "Display mode, chip colours",
					},
				],
			},
			{
				label: "Sidecar",
				items: [
					{
						id: "project-scanning",
						label: "Model & Scanning",
						icon: "scan-search",
						description: "Model, paths, stack detection",
					},
				],
			},
			{
				label: "Connector",
				items: [
					{
						id: "project-delivery",
						label: "Delivery Pipeline",
						icon: "rocket",
						description: "Delivery types and hierarchy",
					},
				],
			},
			{
				label: "Plugins",
				items: [
					{
						id: "project-plugins",
						label: "Plugins",
						icon: "puzzle",
						description: "Browse, install, manage plugins",
					},
				],
			},
		];

		// Merge plugin-contributed settings pages into the appropriate group.
		// Pages whose `section` matches a group label (case-insensitive) are appended
		// to that group. Pages with no matching group create a new group keyed by section.
		const pluginPages = pluginRegistry.getSettingsPages();
		if (pluginPages.length === 0) return baseGroups;

		// Clone groups so the base const is not mutated.
		const groups: SettingsGroup[] = baseGroups.map((g) => ({ ...g, items: [...g.items] }));

		for (const page of pluginPages) {
			const targetSection = page.section.toLowerCase();
			const existing = groups.find((g) => g.label.toLowerCase() === targetSection);
			const item: SettingsItem = {
				id: `plugin:${page.pluginName}:${page.id}`,
				label: page.label,
				icon: "puzzle",
				description: page.pluginName,
			};
			if (existing) {
				existing.items.push(item);
			} else {
				// Create a new group for this section
				groups.push({ label: page.section, items: [item] });
			}
		}

		return groups;
	});
</script>

<ScrollArea class="h-full">
	<div class="p-2">
		{#if mode === "app"}
			<div class="space-y-0.5">
				{#each appCategories as item (item.id)}
					<button
						class="flex w-full items-center gap-2 rounded px-2 py-2 text-left transition-colors hover:bg-accent/50"
						class:bg-accent={currentSection === item.id}
						class:text-accent-foreground={currentSection === item.id}
						onclick={() => handleSectionChange(item.id)}
					>
						<Icon name={item.icon} size="md" />
						<div class="min-w-0">
							<div class="truncate text-sm font-medium">{item.label}</div>
							<div class="truncate text-xs text-muted-foreground">{item.description}</div>
						</div>
					</button>
				{/each}
			</div>
		{:else}
			<div class="space-y-4">
				{#each projectGroups as group (group.label)}
					<div>
						<div class="mb-1 px-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
							{group.label}
						</div>
						<div class="space-y-0.5">
							{#each group.items as item (item.id)}
								<button
									class="flex w-full items-center gap-2 rounded px-2 py-2 text-left transition-colors hover:bg-accent/50"
									class:bg-accent={currentSection === item.id}
									class:text-accent-foreground={currentSection === item.id}
									onclick={() => handleSectionChange(item.id)}
								>
									<Icon name={item.icon} size="md" />
									<div class="min-w-0">
										<div class="truncate text-sm font-medium">{item.label}</div>
										<div class="truncate text-xs text-muted-foreground">{item.description}</div>
									</div>
								</button>
							{/each}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</ScrollArea>

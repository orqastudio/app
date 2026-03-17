<script lang="ts">
	import { Icon, ScrollArea } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { settingsStore } = getStores();

	interface SettingsCategory {
		id: string;
		label: string;
		icon: string;
		description: string;
	}

	interface Props {
		mode: "app" | "project";
		activeSection?: string;
		onSectionChange?: (section: string) => void;
	}

	const { mode, activeSection, onSectionChange }: Props = $props();

	const currentSection = $derived(activeSection ?? settingsStore.activeSection);
	function handleSectionChange(id: string) {
		if (onSectionChange) {
			onSectionChange(id);
		} else {
			settingsStore.setActiveSection(id);
		}
	}

	const appCategories: SettingsCategory[] = [
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

	const projectCategories: SettingsCategory[] = [
		{
			id: "project-general",
			label: "General",
			icon: "settings",
			description: "Name, icon, description",
		},
		{
			id: "project-scanning",
			label: "Model & Scanning",
			icon: "scan-search",
			description: "Model, paths, stack detection",
		},
		{
			id: "project-navigation",
			label: "Navigation",
			icon: "panel-left",
			description: "Nav tree, item order, visibility",
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
		{
			id: "project-delivery",
			label: "Delivery Pipeline",
			icon: "rocket",
			description: "Delivery types and hierarchy",
		},
		{
			id: "project-status",
			label: "Status Machine",
			icon: "workflow",
			description: "Statuses, transitions, auto rules",
		},
		{
			id: "project-plugins",
			label: "Plugins",
			icon: "puzzle",
			description: "Browse, install, manage plugins",
		},
	];

	const categories = $derived(mode === "app" ? appCategories : projectCategories);
</script>

<ScrollArea class="h-full">
	<div class="space-y-0.5 p-2">
		{#each categories as cat (cat.id)}
			<button
				class="flex w-full items-center gap-2 rounded px-2 py-2 text-left transition-colors hover:bg-accent/50"
				class:bg-accent={currentSection === cat.id}
				class:text-accent-foreground={currentSection === cat.id}
				onclick={() => handleSectionChange(cat.id)}
			>
				<Icon name={cat.icon} size="md" />
				<div class="min-w-0">
					<div class="truncate text-sm font-medium">{cat.label}</div>
					<div class="truncate text-xs text-muted-foreground">{cat.description}</div>
				</div>
			</button>
		{/each}
	</div>
</ScrollArea>

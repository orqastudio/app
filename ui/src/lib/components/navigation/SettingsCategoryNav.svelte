<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import MonitorIcon from "@lucide/svelte/icons/monitor";
	import BrainIcon from "@lucide/svelte/icons/brain";
	import SettingsIcon from "@lucide/svelte/icons/settings";
	import ScanSearchIcon from "@lucide/svelte/icons/scan-search";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import PaletteIcon from "@lucide/svelte/icons/palette";
	import KeyboardIcon from "@lucide/svelte/icons/keyboard";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import type { Component } from "svelte";

	interface SettingsCategory {
		id: string;
		label: string;
		icon: Component;
		description: string;
	}

	interface Props {
		mode: "app" | "project";
	}

	const { mode }: Props = $props();

	const appCategories: SettingsCategory[] = [
		{
			id: "provider",
			label: "Provider",
			icon: MonitorIcon,
			description: "Sidecar status, CLI path",
		},
		{
			id: "model",
			label: "Model",
			icon: BrainIcon,
			description: "Default Claude model",
		},
		{
			id: "appearance",
			label: "Appearance",
			icon: PaletteIcon,
			description: "Theme, font size",
		},
		{
			id: "shortcuts",
			label: "Keyboard Shortcuts",
			icon: KeyboardIcon,
			description: "Shortcut reference",
		},
	];

	const projectCategories: SettingsCategory[] = [
		{
			id: "project-general",
			label: "General",
			icon: SettingsIcon,
			description: "Name, icon, description",
		},
		{
			id: "project-scanning",
			label: "Model & Scanning",
			icon: ScanSearchIcon,
			description: "Model, paths, stack detection",
		},
		{
			id: "project-governance",
			label: "Governance",
			icon: ShieldIcon,
			description: "Agents, rules, skills, hooks",
		},
	];

	const categories = $derived(mode === "app" ? appCategories : projectCategories);
</script>

<ScrollArea.Root class="h-full">
	<div class="space-y-0.5 p-2">
		{#each categories as cat (cat.id)}
			{@const Icon = cat.icon}
			<button
				class="flex w-full items-center gap-2 rounded px-2 py-2 text-left transition-colors hover:bg-accent/50"
				class:bg-accent={settingsStore.activeSection === cat.id}
				class:text-accent-foreground={settingsStore.activeSection === cat.id}
				onclick={() => settingsStore.setActiveSection(cat.id)}
			>
				<Icon class="h-4 w-4 shrink-0 text-muted-foreground" />
				<div class="min-w-0">
					<div class="truncate text-sm font-medium">{cat.label}</div>
					<div class="truncate text-xs text-muted-foreground">{cat.description}</div>
				</div>
			</button>
		{/each}
	</div>
</ScrollArea.Root>

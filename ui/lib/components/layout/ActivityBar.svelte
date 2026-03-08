<script lang="ts">
	import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import ClipboardListIcon from "@lucide/svelte/icons/clipboard-list";
	import UsersIcon from "@lucide/svelte/icons/users";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import SettingsIcon from "@lucide/svelte/icons/settings";
	import { Separator } from "$lib/components/ui/separator";
	import { navigationStore, type ActivityGroup } from "$lib/stores/navigation.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import ActivityBarItem from "./ActivityBarItem.svelte";
	import type { Component } from "svelte";

	interface GroupItem {
		group: ActivityGroup;
		icon: Component;
		label: string;
	}

	const groupItems: GroupItem[] = [
		{ group: "documentation", icon: FileTextIcon, label: "Documentation" },
		{ group: "planning", icon: ClipboardListIcon, label: "Planning" },
		{ group: "team", icon: UsersIcon, label: "Team" },
		{ group: "governance", icon: ShieldIcon, label: "Governance" },
	];
</script>

<div class="flex w-12 flex-col items-center border-r border-border bg-muted/30 py-2">
	<!-- Project Dashboard -->
	<ActivityBarItem
		icon={LayoutDashboardIcon}
		label="Project Dashboard"
		active={navigationStore.activeActivity === "project"}
		onclick={() => navigationStore.setActivity("project")}
	/>

	<div class="my-1 w-6">
		<Separator />
	</div>

	<!-- Group categories -->
	{#each groupItems as item (item.group)}
		<ActivityBarItem
			icon={item.icon}
			label={item.label}
			active={navigationStore.activeGroup === item.group}
			onclick={() => navigationStore.setGroup(item.group)}
		/>
	{/each}

	<div class="flex-1"></div>

	<!-- Project Settings -->
	<ActivityBarItem
		icon={SettingsIcon}
		label="Project Settings"
		active={navigationStore.activeActivity === "settings"}
		onclick={() => { settingsStore.setActiveSection("project-general"); navigationStore.setActivity("settings"); }}
	/>
</div>

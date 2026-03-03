<script lang="ts">
	import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import BotIcon from "@lucide/svelte/icons/bot";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import SettingsIcon from "@lucide/svelte/icons/settings";
	import { Separator } from "$lib/components/ui/separator";
	import { navigationStore, type ActivityView } from "$lib/stores/navigation.svelte";
	import ActivityBarItem from "./ActivityBarItem.svelte";
	import type { Component } from "svelte";

	interface ActivityItem {
		view: ActivityView;
		icon: Component;
		label: string;
	}

	const dashboardItem: ActivityItem = {
		view: "project",
		icon: LayoutDashboardIcon,
		label: "Project Dashboard",
	};

	const artifactItems: ActivityItem[] = [
		{ view: "docs", icon: FileTextIcon, label: "Docs" },
		{ view: "agents", icon: BotIcon, label: "Agents" },
		{ view: "rules", icon: ShieldIcon, label: "Rules" },
		{ view: "skills", icon: ZapIcon, label: "Skills" },
		{ view: "hooks", icon: GitBranchIcon, label: "Hooks" },
	];
</script>

<div class="flex w-12 flex-col items-center border-r border-border bg-muted/30 py-2">
	<!-- Project Dashboard -->
	<ActivityBarItem
		icon={dashboardItem.icon}
		label={dashboardItem.label}
		active={navigationStore.activeActivity === dashboardItem.view}
		onclick={() => navigationStore.setActivity(dashboardItem.view)}
	/>

	<div class="my-1 w-6">
		<Separator />
	</div>

	<!-- Artifact categories -->
	{#each artifactItems as item}
		<ActivityBarItem
			icon={item.icon}
			label={item.label}
			active={navigationStore.activeActivity === item.view}
			onclick={() => navigationStore.setActivity(item.view)}
		/>
	{/each}

	<div class="flex-1"></div>

	<!-- Settings at bottom -->
	<ActivityBarItem
		icon={SettingsIcon}
		label="Settings"
		active={navigationStore.activeActivity === "settings"}
		onclick={() => navigationStore.setActivity("settings")}
	/>
</div>

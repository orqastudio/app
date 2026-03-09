<script lang="ts">
	import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import ClipboardListIcon from "@lucide/svelte/icons/clipboard-list";
	import UsersIcon from "@lucide/svelte/icons/users";
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import SettingsIcon from "@lucide/svelte/icons/settings";
	import FolderIcon from "@lucide/svelte/icons/folder";
	import BookOpenIcon from "@lucide/svelte/icons/book-open";
	import ZapIcon from "@lucide/svelte/icons/zap";
	import TargetIcon from "@lucide/svelte/icons/target";
	import LayersIcon from "@lucide/svelte/icons/layers";
	import LightbulbIcon from "@lucide/svelte/icons/lightbulb";
	import FlaskConicalIcon from "@lucide/svelte/icons/flask-conical";
	import ScrollTextIcon from "@lucide/svelte/icons/scroll-text";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import BotIcon from "@lucide/svelte/icons/bot";
	import { Separator } from "$lib/components/ui/separator";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { isArtifactGroup } from "$lib/types/project";
	import ActivityBarItem from "./ActivityBarItem.svelte";
	import type { Component } from "svelte";

	/** Map from icon name strings (as stored in config) to Lucide icon components. */
	const ICON_MAP: Record<string, Component> = {
		"file-text": FileTextIcon,
		"clipboard-list": ClipboardListIcon,
		users: UsersIcon,
		shield: ShieldIcon,
		folder: FolderIcon,
		"book-open": BookOpenIcon,
		zap: ZapIcon,
		target: TargetIcon,
		layers: LayersIcon,
		lightbulb: LightbulbIcon,
		"flask-conical": FlaskConicalIcon,
		"scroll-text": ScrollTextIcon,
		"git-branch": GitBranchIcon,
		bot: BotIcon,
	};

	function resolveIcon(iconName: string | undefined): Component {
		if (iconName && iconName in ICON_MAP) {
			return ICON_MAP[iconName];
		}
		return FolderIcon;
	}

	/** Convert a config key to a human-readable label (mirrors Rust humanize_name). */
	function humanizeKey(key: string): string {
		return key
			.replace(/[-_]/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	const artifactConfig = $derived(projectStore.artifactConfig);
</script>

<div class="flex w-12 flex-col items-center border-r border-border bg-muted/30 py-2">
	<!-- Project Dashboard -->
	<ActivityBarItem
		icon={LayoutDashboardIcon}
		label="Project Dashboard"
		active={navigationStore.activeActivity === "project"}
		onclick={() => navigationStore.setActivity("project")}
	/>

	{#if artifactConfig.length > 0}
		<div class="my-1 w-6">
			<Separator />
		</div>

		<!-- Config-driven artifact entries -->
		{#each artifactConfig as entry (entry.key)}
			{@const Icon = resolveIcon(entry.icon)}
			{@const entryLabel = entry.label ?? humanizeKey(entry.key)}
			{#if isArtifactGroup(entry)}
				<!-- Group entry — clicking activates the group -->
				<ActivityBarItem
					icon={Icon}
					label={entryLabel}
					active={navigationStore.activeGroup === entry.key}
					onclick={() => navigationStore.setGroup(entry.key)}
				/>
			{:else}
				<!-- Direct type entry — clicking activates the type directly -->
				<ActivityBarItem
					icon={Icon}
					label={entryLabel}
					active={navigationStore.activeActivity === entry.key && navigationStore.activeGroup === null}
					onclick={() => { navigationStore.activeGroup = null; navigationStore.setActivity(entry.key); }}
				/>
			{/if}
		{/each}
	{/if}

	<div class="flex-1"></div>

	<!-- Project Settings -->
	<ActivityBarItem
		icon={SettingsIcon}
		label="Project Settings"
		active={navigationStore.activeActivity === "settings"}
		onclick={() => { settingsStore.setActiveSection("project-general"); navigationStore.setActivity("settings"); }}
	/>
</div>

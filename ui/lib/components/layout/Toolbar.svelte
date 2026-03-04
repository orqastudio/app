<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import ArrowLeftRightIcon from "@lucide/svelte/icons/arrow-left-right";
	import XIcon from "@lucide/svelte/icons/x";
	import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
	import fMark from "$lib/assets/f-mark.svg";
	import { projectStore } from "$lib/stores/project.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";

	const projectName = $derived(
		projectStore.projectSettings?.name ?? projectStore.activeProject?.name ?? "",
	);
	const hasProject = $derived(projectStore.hasProject);
	const isConfiguring = $derived(navigationStore.activeActivity === "configure");

	async function handleOpenProject() {
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Open Project Folder",
		});
		if (selected && typeof selected === "string") {
			await projectStore.openProject(selected);
		}
	}

	function handleConfigure() {
		if (isConfiguring) {
			// Toggle off — go back to project dashboard or welcome
			if (hasProject) {
				navigationStore.setActivity("project");
			} else {
				navigationStore.setActivity("chat");
			}
		} else {
			settingsStore.setActiveSection("provider");
			navigationStore.setActivity("configure");
		}
	}
</script>

<div class="flex h-10 items-center gap-2 border-b border-border bg-background px-4">
	{#if projectStore.iconDataUrl}
		<img src={projectStore.iconDataUrl} alt={projectName || "Orqa Studio"} class="h-5 w-5 rounded object-contain" />
	{:else}
		<img src={fMark} alt="Orqa Studio" class="h-5 w-5" />
	{/if}
	<span class="text-sm font-semibold">{projectName || "Orqa Studio"}</span>
	<div class="flex-1"></div>
	{#if hasProject}
		<button
			class="flex items-center gap-1 rounded px-2 py-1 text-xs text-muted-foreground hover:bg-accent/50 hover:text-foreground"
			onclick={handleOpenProject}
			title="Change Project"
		>
			<ArrowLeftRightIcon class="h-3.5 w-3.5" />
			<span>Change Project</span>
		</button>
		<button
			class="flex items-center gap-1 rounded px-2 py-1 text-xs text-muted-foreground hover:bg-accent/50 hover:text-foreground"
			onclick={() => projectStore.closeProject()}
			title="Close Project"
		>
			<XIcon class="h-3.5 w-3.5" />
			<span>Close Project</span>
		</button>
	{:else}
		<button
			class="flex items-center gap-1 rounded px-2 py-1 text-xs text-muted-foreground hover:bg-accent/50 hover:text-foreground"
			onclick={handleOpenProject}
			title="Open Project"
		>
			<FolderOpenIcon class="h-3.5 w-3.5" />
			<span>Open Project</span>
		</button>
	{/if}
	<button
		class="flex items-center gap-1 rounded px-2 py-1 text-xs hover:bg-accent/50"
		class:text-foreground={isConfiguring}
		class:bg-accent={isConfiguring}
		class:text-muted-foreground={!isConfiguring}
		onclick={handleConfigure}
		title="Orqa Studio Configuration"
	>
		<SlidersHorizontalIcon class="h-3.5 w-3.5" />
		<span>Configure</span>
	</button>
</div>

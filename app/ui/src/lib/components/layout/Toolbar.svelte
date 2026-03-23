<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { getVersion, getName } from "@tauri-apps/api/app";
	import logoStatic from "$lib/assets/logo-static.svg";
	import { getStores } from "@orqastudio/sdk";

	const { projectStore } = getStores();
	import WindowControls from "./WindowControls.svelte";
	import AboutDialog from "./AboutDialog.svelte";
	import NewProjectDialog from "./NewProjectDialog.svelte";
	import SettingsDialog from "./SettingsDialog.svelte";
	import InitConfirmDialog from "./InitConfirmDialog.svelte";
	import MenuBar from "./MenuBar.svelte";
	import ProjectSwitcher from "../navigation/ProjectSwitcher.svelte";

	const hasProject = $derived(projectStore.hasProject);

	let aboutOpen = $state(false);
	let settingsOpen = $state(false);
	let newProjectOpen = $state(false);
	let initConfirmOpen = $state(false);
	let pendingInitPath = $state<string | null>(null);
	let appName = $state("OrqaStudio");
	let appVersion = $state("0.1.0");

	$effect(() => {
		getName().then((n) => { appName = n; });
		getVersion().then((v) => { appVersion = v; });
	});

	function handleNewProject(): void {
		newProjectOpen = true;
	}

	async function handleOpenProject(): Promise<void> {
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Open Orqa Project",
		});
		if (selected && typeof selected === "string") {
			const isOrqa = await projectStore.checkIsOrqaProject(selected);
			if (isOrqa) {
				await projectStore.openProject(selected);
			} else {
				pendingInitPath = selected;
				initConfirmOpen = true;
			}
		}
	}

	async function confirmInitialize(): Promise<void> {
		initConfirmOpen = false;
		if (pendingInitPath) {
			await projectStore.openProject(pendingInitPath);
			pendingInitPath = null;
		}
	}

	function cancelInitialize(): void {
		initConfirmOpen = false;
		pendingInitPath = null;
	}

	function handleSettings(): void {
		settingsOpen = true;
	}

	function handleDragStart(e: MouseEvent): void {
		if (e.button !== 0) return;
		const target = e.target as HTMLElement;
		if (target.closest("button, [data-menu-bar]")) return;
		getCurrentWindow().startDragging();
	}

	function handleDoubleClick(e: MouseEvent): void {
		const target = e.target as HTMLElement;
		if (target.closest("button, [data-menu-bar]")) return;
		const win = getCurrentWindow();
		win.isMaximized().then((maximized) => {
			if (maximized) { win.unmaximize(); } else { win.maximize(); }
		});
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="toolbar-drag relative z-50 flex h-10 items-center border-b border-border bg-background"
	onmousedown={handleDragStart}
	ondblclick={handleDoubleClick}
>
	<div class="flex h-10 w-12 shrink-0 items-center justify-center border-r border-border">
		{#if projectStore.iconDataUrl}
			<img src={projectStore.iconDataUrl} alt="OrqaStudio" class="h-5 w-5 rounded object-contain pointer-events-none" />
		{:else}
			<img src={logoStatic} alt="OrqaStudio" class="h-5 w-5 pointer-events-none" />
		{/if}
	</div>

	<MenuBar
		{hasProject}
		onNewProject={handleNewProject}
		onOpenProject={handleOpenProject}
		onCloseProject={() => projectStore.closeProject()}
		onSettings={handleSettings}
		onAbout={() => { aboutOpen = true; }}
		onExit={() => getCurrentWindow().close()}
	/>

	<div class="ml-2">
		<ProjectSwitcher />
	</div>
	<div class="flex-1"></div>
	<WindowControls />
</div>

<AboutDialog
	open={aboutOpen}
	onClose={() => { aboutOpen = false; }}
	{appName}
	{appVersion}
/>

<SettingsDialog
	open={settingsOpen}
	onClose={() => { settingsOpen = false; }}
/>

<NewProjectDialog
	open={newProjectOpen}
	onClose={() => { newProjectOpen = false; }}
/>

<InitConfirmDialog
	open={initConfirmOpen}
	pendingPath={pendingInitPath}
	onConfirm={confirmInitialize}
	onCancel={cancelInitialize}
/>

<style>
	.toolbar-drag {
		-webkit-app-region: drag;
	}
	.toolbar-drag :global(button) {
		cursor: default;
		-webkit-app-region: no-drag;
	}
</style>

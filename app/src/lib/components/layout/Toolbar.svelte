<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { getVersion, getName } from "@tauri-apps/api/app";
	import { invoke } from "@tauri-apps/api/core";
	import logoStatic from "$lib/assets/logo-static.svg";
	import { Spacer, Button, Text } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { projectStore } = getStores();
	import WindowControls from "./WindowControls.svelte";
	import AboutDialog from "./AboutDialog.svelte";
	import NewProjectDialog from "./NewProjectDialog.svelte";
	import SettingsDialog from "./SettingsDialog.svelte";
	import InitConfirmDialog from "./InitConfirmDialog.svelte";
	import MenuBar from "./MenuBar.svelte";

	const hasProject = $derived(projectStore.hasProject);

	let aboutOpen = $state(false);
	let settingsOpen = $state(false);
	let newProjectOpen = $state(false);
	let initConfirmOpen = $state(false);
	let pendingInitPath = $state<string | null>(null);
	let appName = $state("OrqaStudio");
	let appVersion = $state("0.1.0");

	$effect(() => {
		getName().then((n) => {
			appName = n;
		});
		getVersion().then((v) => {
			appVersion = v;
		});
	});

	/** Open the new project dialog. */
	function handleNewProject(): void {
		newProjectOpen = true;
	}

	/** Show the OS folder picker and open the selected directory as a project. */
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

	/** Confirm initializing the pending path as an Orqa project. */
	async function confirmInitialize(): Promise<void> {
		initConfirmOpen = false;
		if (pendingInitPath) {
			await projectStore.openProject(pendingInitPath);
			pendingInitPath = null;
		}
	}

	/** Cancel the pending project initialization and clear the pending path. */
	function cancelInitialize(): void {
		initConfirmOpen = false;
		pendingInitPath = null;
	}

	/** Open the settings dialog. */
	function handleSettings(): void {
		settingsOpen = true;
	}

	/** Invoke the Tauri command to launch the devtools window. */
	async function handleLaunchDevtools(): Promise<void> {
		await invoke("launch_devtools");
	}

	/**
	 * Start window drag on primary mouse button press outside interactive elements.
	 * @param e
	 */
	function handleDragStart(e: MouseEvent): void {
		if (e.button !== 0) return;
		const target = e.target as HTMLElement;
		if (target.closest("button, [data-menu-bar]")) return;
		getCurrentWindow().startDragging();
	}

	/**
	 * Toggle window maximize state on double-click outside interactive elements.
	 * @param e
	 */
	function handleDoubleClick(e: MouseEvent): void {
		const target = e.target as HTMLElement;
		if (target.closest("button, [data-menu-bar]")) return;
		const win = getCurrentWindow();
		win.isMaximized().then((maximized) => {
			if (maximized) {
				win.unmaximize();
			} else {
				win.maximize();
			}
		});
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="toolbar-drag border-border bg-background relative z-50 flex h-10 items-center border-b"
	onmousedown={handleDragStart}
	ondblclick={handleDoubleClick}
>
	<div class="border-border flex h-10 w-12 shrink-0 items-center justify-center border-r">
		{#if projectStore.iconDataUrl}
			<img
				src={projectStore.iconDataUrl}
				alt="OrqaStudio"
				class="pointer-events-none h-5 w-5 rounded object-contain"
			/>
		{:else}
			<img src={logoStatic} alt="OrqaStudio" class="pointer-events-none h-5 w-5" />
		{/if}
	</div>

	<MenuBar
		{hasProject}
		onNewProject={handleNewProject}
		onOpenProject={handleOpenProject}
		onCloseProject={() => projectStore.closeProject()}
		onSettings={handleSettings}
		onAbout={() => {
			aboutOpen = true;
		}}
		onExit={() => getCurrentWindow().close()}
	/>

	<Spacer />
	<Button variant="ghost" onclick={handleLaunchDevtools} title="Open OrqaDev">
		<Text variant="caption" tone="muted">DevTools</Text>
	</Button>
	<WindowControls />
</div>

<AboutDialog
	open={aboutOpen}
	onClose={() => {
		aboutOpen = false;
	}}
	{appName}
	{appVersion}
/>

<SettingsDialog
	open={settingsOpen}
	onClose={() => {
		settingsOpen = false;
	}}
/>

<NewProjectDialog
	open={newProjectOpen}
	onClose={() => {
		newProjectOpen = false;
	}}
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

<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import { getVersion, getName } from "@tauri-apps/api/app";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import logoStatic from "$lib/assets/logo-static.svg";
	import { launchDevtools } from "$lib/services/plugin-service";
	import {
		Spacer,
		Button,
		Text,
		Panel,
		WindowTitleBar,
		AppIcon,
	} from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { projectStore } = getStores();
	import { WindowControls } from "@orqastudio/svelte-components/pure";
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

	/** Launch the devtools window via the plugin service. */
	async function handleLaunchDevtools(): Promise<void> {
		await launchDevtools();
	}
</script>

<WindowTitleBar>
	<!-- Logo area matching the activity bar width -->
	<Panel fixedWidth="icon-bar" border="right" direction="column" align="center" full padding="none">
		{#if projectStore.iconDataUrl}
			<AppIcon
				src={projectStore.iconDataUrl}
				alt="OrqaStudio"
				size="toolbar"
				rounded
				objectContain
			/>
		{:else}
			<AppIcon src={logoStatic} alt="OrqaStudio" size="toolbar" />
		{/if}
	</Panel>

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
</WindowTitleBar>

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

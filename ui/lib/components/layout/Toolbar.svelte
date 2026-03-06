<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import { invoke } from "$lib/ipc/invoke";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { getVersion, getName } from "@tauri-apps/api/app";
	import FolderPlusIcon from "@lucide/svelte/icons/folder-plus";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import FolderXIcon from "@lucide/svelte/icons/folder-x";
	import FolderCodeIcon from "@lucide/svelte/icons/folder-code";
	import SquarePlusIcon from "@lucide/svelte/icons/square-plus";
	import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
	import InfoIcon from "@lucide/svelte/icons/info";
	import LogOutIcon from "@lucide/svelte/icons/log-out";
	import XIcon from "@lucide/svelte/icons/x";
	import finMark from "$lib/assets/fin-mark.svg";
	import logoPulse from "$lib/assets/logo-pulse.svg";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import * as Dialog from "$lib/components/ui/dialog";
	import { Button } from "$lib/components/ui/button";
	import { projectStore } from "$lib/stores/project.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import type { ProjectSettings } from "$lib/types";
	import SettingsView from "$lib/components/settings/SettingsView.svelte";
	import SettingsCategoryNav from "$lib/components/navigation/SettingsCategoryNav.svelte";
	import WindowControls from "./WindowControls.svelte";

	const hasProject = $derived(projectStore.hasProject);

	// --- VS Code-style menu bar state ---
	let activeMenu = $state<string | null>(null);
	const menuMode = $derived(activeMenu !== null);

	let aboutOpen = $state(false);
	let settingsOpen = $state(false);
	let newProjectOpen = $state(false);
	let initConfirmOpen = $state(false);
	let pendingInitPath = $state<string | null>(null);
	let appName = $state("Orqa Studio");
	let appVersion = $state("0.1.0");

	// Load app info
	$effect(() => {
		getName().then((n) => { appName = n; });
		getVersion().then((v) => { appVersion = v; });
	});

	function handleMenuClick(menu: string) {
		if (activeMenu === menu) {
			activeMenu = null;
		} else {
			activeMenu = menu;
		}
	}

	function handleMenuHover(menu: string) {
		if (menuMode && activeMenu !== menu) {
			activeMenu = menu;
		}
	}

	function handleMenuItemClick(action: () => void) {
		activeMenu = null;
		action();
	}

	// --- Check if a folder is an initialized Orqa project ---
	async function checkIsOrqaProject(path: string): Promise<boolean> {
		const settings = await invoke<ProjectSettings | null>("project_settings_read", { path });
		return settings !== null;
	}

	// --- New Project dialog ---
	function handleNewProject() {
		activeMenu = null;
		newProjectOpen = true;
	}

	/** Initialize an existing codebase as an Orqa project */
	async function handleInitializeExisting() {
		newProjectOpen = false;
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Select Folder to Initialize",
		});
		if (selected && typeof selected === "string") {
			// Opens the folder — if already initialized it loads normally,
			// otherwise AppLayout shows the setup wizard
			await projectStore.openProject(selected);
		}
	}

	/** Create a brand new empty project */
	async function handleCreateFromScratch() {
		newProjectOpen = false;
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Select Location for New Project",
		});
		if (selected && typeof selected === "string") {
			// Open the selected folder — setup wizard will handle initialization
			await projectStore.openProject(selected);
		}
	}

	// --- Open Project (existing Orqa projects only) ---
	async function handleOpenProject() {
		activeMenu = null;
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Open Orqa Project",
		});
		if (selected && typeof selected === "string") {
			const isOrqa = await checkIsOrqaProject(selected);
			if (isOrqa) {
				await projectStore.openProject(selected);
			} else {
				// Not an Orqa project — ask user if they want to initialize
				pendingInitPath = selected;
				initConfirmOpen = true;
			}
		}
	}

	async function confirmInitialize() {
		initConfirmOpen = false;
		if (pendingInitPath) {
			await projectStore.openProject(pendingInitPath);
			pendingInitPath = null;
		}
	}

	function cancelInitialize() {
		initConfirmOpen = false;
		pendingInitPath = null;
	}

	function handleSettings() {
		activeMenu = null;
		settingsStore.setActiveSection("provider");
		settingsOpen = true;
	}

	function handleCloseProject() {
		activeMenu = null;
		projectStore.closeProject();
	}

	function handleAbout() {
		activeMenu = null;
		aboutOpen = true;
	}

	function handleExit() {
		activeMenu = null;
		getCurrentWindow().close();
	}

	// --- Titlebar drag ---
	function handleDragStart(e: MouseEvent) {
		if (e.button !== 0) return;
		const target = e.target as HTMLElement;
		if (target.closest("button, [data-menu-bar]")) return;
		getCurrentWindow().startDragging();
	}

	function handleDoubleClick(e: MouseEvent) {
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
	class="toolbar-drag relative z-50 flex h-10 items-center border-b border-border bg-background"
	onmousedown={handleDragStart}
	ondblclick={handleDoubleClick}
>
	<!-- App icon — centered over the 48px activity bar -->
	<div class="flex h-10 w-12 shrink-0 items-center justify-center border-r border-border">
		{#if projectStore.iconDataUrl}
			<img src={projectStore.iconDataUrl} alt="Orqa Studio" class="h-5 w-5 rounded object-contain pointer-events-none" />
		{:else}
			<img src={finMark} alt="Orqa Studio" class="h-5 w-5 pointer-events-none" />
		{/if}
	</div>

	<!-- Menu bar -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="flex items-center px-1" data-menu-bar>
		<!-- File menu -->
		<DropdownMenu.Root
			open={activeMenu === "file"}
			onOpenChange={(open) => { if (!open) activeMenu = null; }}
		>
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				onmouseenter={() => handleMenuHover("file")}
			>
				<DropdownMenu.Trigger
					class="flex h-7 items-center rounded px-2.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground data-[state=open]:bg-accent data-[state=open]:text-foreground"
					onclick={(e: MouseEvent) => { e.preventDefault(); handleMenuClick("file"); }}
				>
					File
				</DropdownMenu.Trigger>
			</div>
			<DropdownMenu.Content align="start" class="w-52">
				<DropdownMenu.Item onclick={() => handleMenuItemClick(handleNewProject)}>
					<FolderPlusIcon class="mr-2 h-4 w-4" />
					New Project...
				</DropdownMenu.Item>
				<DropdownMenu.Item onclick={() => handleMenuItemClick(handleOpenProject)}>
					<FolderOpenIcon class="mr-2 h-4 w-4" />
					Open Project...
				</DropdownMenu.Item>
				{#if hasProject}
					<DropdownMenu.Separator />
					<DropdownMenu.Item onclick={() => handleMenuItemClick(handleCloseProject)}>
						<FolderXIcon class="mr-2 h-4 w-4" />
						Close Project
					</DropdownMenu.Item>
				{/if}
				<DropdownMenu.Separator />
				<DropdownMenu.Item onclick={() => handleMenuItemClick(handleExit)}>
					<LogOutIcon class="mr-2 h-4 w-4" />
					Exit
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>

		<!-- Edit menu -->
		<DropdownMenu.Root
			open={activeMenu === "edit"}
			onOpenChange={(open) => { if (!open) activeMenu = null; }}
		>
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				onmouseenter={() => handleMenuHover("edit")}
			>
				<DropdownMenu.Trigger
					class="flex h-7 items-center rounded px-2.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground data-[state=open]:bg-accent data-[state=open]:text-foreground"
					onclick={(e: MouseEvent) => { e.preventDefault(); handleMenuClick("edit"); }}
				>
					Edit
				</DropdownMenu.Trigger>
			</div>
			<DropdownMenu.Content align="start" class="w-52">
				<DropdownMenu.Item onclick={() => handleMenuItemClick(handleSettings)}>
					<SlidersHorizontalIcon class="mr-2 h-4 w-4" />
					Settings
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>

		<!-- Help menu -->
		<DropdownMenu.Root
			open={activeMenu === "help"}
			onOpenChange={(open) => { if (!open) activeMenu = null; }}
		>
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				onmouseenter={() => handleMenuHover("help")}
			>
				<DropdownMenu.Trigger
					class="flex h-7 items-center rounded px-2.5 text-xs font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-foreground data-[state=open]:bg-accent data-[state=open]:text-foreground"
					onclick={(e: MouseEvent) => { e.preventDefault(); handleMenuClick("help"); }}
				>
					Help
				</DropdownMenu.Trigger>
			</div>
			<DropdownMenu.Content align="start" class="w-52">
				<DropdownMenu.Item onclick={() => handleMenuItemClick(handleAbout)}>
					<InfoIcon class="mr-2 h-4 w-4" />
					About
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>

	<!-- Spacer for drag area -->
	<div class="flex-1"></div>

	<!-- Window controls -->
	<WindowControls />
</div>

<!-- About dialog -->
<Dialog.Root bind:open={aboutOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<div class="flex items-center gap-3">
				<img src={logoPulse} alt="Orqa Studio" class="h-10 w-10" />
				<div>
					<Dialog.Title>{appName}</Dialog.Title>
					<Dialog.Description>Version {appVersion}</Dialog.Description>
				</div>
			</div>
		</Dialog.Header>
		<div class="space-y-2 text-sm text-muted-foreground">
			<p>A managed agentic development environment powered by Claude.</p>
			<p class="text-xs">Built with Tauri, Svelte, and Rust.</p>
		</div>
	</Dialog.Content>
</Dialog.Root>

<!-- Settings dialog -->
<Dialog.Root bind:open={settingsOpen}>
	<Dialog.Content class="flex h-[85vh] w-[90vw] max-w-5xl flex-col gap-0 overflow-hidden p-0 sm:max-w-5xl">
		<div class="flex items-center justify-between border-b border-border px-6 py-4">
			<Dialog.Title>Settings</Dialog.Title>
			<Dialog.Description class="sr-only">Application settings</Dialog.Description>
			<button
				class="rounded-sm p-1 text-muted-foreground opacity-70 transition-opacity hover:opacity-100"
				onclick={() => { settingsOpen = false; }}
			>
				<XIcon class="h-4 w-4" />
			</button>
		</div>
		<div class="flex flex-1 overflow-hidden">
			<!-- Sidebar navigation -->
			<div class="w-56 shrink-0 border-r border-border">
				<SettingsCategoryNav mode="app" />
			</div>
			<!-- Settings content -->
			<div class="flex-1 overflow-hidden">
				<SettingsView />
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>

<!-- New Project dialog -->
<Dialog.Root bind:open={newProjectOpen}>
	<Dialog.Content class="max-w-md">
		<Dialog.Header>
			<Dialog.Title>New Project</Dialog.Title>
			<Dialog.Description>Choose how to create your Orqa project.</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-3 py-2">
			<button
				class="flex items-start gap-4 rounded-lg border border-border p-4 text-left transition-colors hover:bg-accent"
				onclick={handleCreateFromScratch}
			>
				<SquarePlusIcon class="mt-0.5 h-6 w-6 shrink-0 text-primary" />
				<div>
					<p class="text-sm font-medium">Create From Scratch</p>
					<p class="text-xs text-muted-foreground">
						Start with a fresh project in an empty folder.
					</p>
				</div>
			</button>
			<button
				class="flex items-start gap-4 rounded-lg border border-border p-4 text-left transition-colors hover:bg-accent"
				onclick={handleInitializeExisting}
			>
				<FolderCodeIcon class="mt-0.5 h-6 w-6 shrink-0 text-primary" />
				<div>
					<p class="text-sm font-medium">Initialize Existing Folder</p>
					<p class="text-xs text-muted-foreground">
						Set up Orqa in an existing codebase. Your files stay untouched — only an .orqa/ config directory is added.
					</p>
				</div>
			</button>
		</div>
	</Dialog.Content>
</Dialog.Root>

<!-- Initialize confirmation dialog -->
<Dialog.Root bind:open={initConfirmOpen}>
	<Dialog.Content class="max-w-sm">
		<Dialog.Header>
			<Dialog.Title>Not an Orqa Project</Dialog.Title>
			<Dialog.Description>
				This folder doesn't have an Orqa configuration. Would you like to initialize it as a new Orqa project?
			</Dialog.Description>
		</Dialog.Header>
		{#if pendingInitPath}
			<p class="rounded bg-muted px-3 py-2 text-xs font-mono text-muted-foreground truncate">
				{pendingInitPath}
			</p>
		{/if}
		<Dialog.Footer>
			<Button variant="outline" onclick={cancelInitialize}>Cancel</Button>
			<Button onclick={confirmInitialize}>Initialize Project</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<style>
	.toolbar-drag {
		-webkit-app-region: drag;
	}
	.toolbar-drag :global(button) {
		cursor: default;
		-webkit-app-region: no-drag;
	}
</style>

<script lang="ts">
	import {
		Icon,
		HStack,
		DropdownMenuRoot,
		MenuBarTrigger,
		DropdownMenuItem,
		DropdownMenuContent,
		DropdownMenuSeparator,
	} from "@orqastudio/svelte-components/pure";

	interface Props {
		hasProject: boolean;
		onNewProject: () => void;
		onOpenProject: () => void;
		onCloseProject: () => void;
		onSettings: () => void;
		onAbout: () => void;
		onExit: () => void;
	}

	const {
		hasProject,
		onNewProject,
		onOpenProject,
		onCloseProject,
		onSettings,
		onAbout,
		onExit,
	}: Props = $props();

	let activeMenu = $state<string | null>(null);
	const menuMode = $derived(activeMenu !== null);

	/**
	 * Toggle the specified menu open or close it if already open.
	 * @param menu - The key identifying which menu to toggle (e.g. "file", "view").
	 */
	function handleMenuClick(menu: string): void {
		activeMenu = activeMenu === menu ? null : menu;
	}

	/**
	 * Switch to a different menu on hover when menu mode is active.
	 * @param menu - The key identifying the menu being hovered over.
	 */
	function handleMenuHover(menu: string): void {
		if (menuMode && activeMenu !== menu) {
			activeMenu = menu;
		}
	}

	/**
	 * Close the menu and execute the provided menu action.
	 * @param action - The callback function for the menu item that was selected.
	 */
	function handleItem(action: () => void): void {
		activeMenu = null;
		action();
	}
</script>

<HStack gap={0} data-menu-bar={true}>
	<!-- File menu -->
	<DropdownMenuRoot
		open={activeMenu === "file"}
		onOpenChange={(isOpen) => {
			if (!isOpen) activeMenu = null;
		}}
	>
		<MenuBarTrigger
			onmouseenter={() => handleMenuHover("file")}
			onclick={(e: MouseEvent) => {
				e.preventDefault();
				handleMenuClick("file");
			}}
		>
			File
		</MenuBarTrigger>
		<DropdownMenuContent align="start">
			<DropdownMenuItem onclick={() => handleItem(onNewProject)}>
				<Icon name="folder-plus" size="md" />
				New Project...
			</DropdownMenuItem>
			<DropdownMenuItem onclick={() => handleItem(onOpenProject)}>
				<Icon name="folder-open" size="md" />
				Open Project...
			</DropdownMenuItem>
			{#if hasProject}
				<DropdownMenuSeparator />
				<DropdownMenuItem onclick={() => handleItem(onCloseProject)}>
					<Icon name="folder-x" size="md" />
					Close Project
				</DropdownMenuItem>
			{/if}
			<DropdownMenuSeparator />
			<DropdownMenuItem onclick={() => handleItem(onExit)}>
				<Icon name="log-out" size="md" />
				Exit
			</DropdownMenuItem>
		</DropdownMenuContent>
	</DropdownMenuRoot>

	<!-- Edit menu -->
	<DropdownMenuRoot
		open={activeMenu === "edit"}
		onOpenChange={(isOpen) => {
			if (!isOpen) activeMenu = null;
		}}
	>
		<MenuBarTrigger
			onmouseenter={() => handleMenuHover("edit")}
			onclick={(e: MouseEvent) => {
				e.preventDefault();
				handleMenuClick("edit");
			}}
		>
			Edit
		</MenuBarTrigger>
		<DropdownMenuContent align="start">
			<DropdownMenuItem onclick={() => handleItem(onSettings)}>
				<Icon name="sliders-horizontal" size="md" />
				Settings
			</DropdownMenuItem>
		</DropdownMenuContent>
	</DropdownMenuRoot>

	<!-- Help menu -->
	<DropdownMenuRoot
		open={activeMenu === "help"}
		onOpenChange={(isOpen) => {
			if (!isOpen) activeMenu = null;
		}}
	>
		<MenuBarTrigger
			onmouseenter={() => handleMenuHover("help")}
			onclick={(e: MouseEvent) => {
				e.preventDefault();
				handleMenuClick("help");
			}}
		>
			Help
		</MenuBarTrigger>
		<DropdownMenuContent align="start">
			<DropdownMenuItem onclick={() => handleItem(onAbout)}>
				<Icon name="info" size="md" />
				About
			</DropdownMenuItem>
		</DropdownMenuContent>
	</DropdownMenuRoot>
</HStack>

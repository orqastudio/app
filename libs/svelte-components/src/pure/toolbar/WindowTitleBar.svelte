<!-- WindowTitleBar — Tauri window toolbar with built-in drag and maximize toggle.
     The full toolbar area is draggable; buttons and [data-menu-bar] elements are excluded.
     Double-clicking the title bar toggles window maximize/restore. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";

	let {
		children,
	}: {
		children?: Snippet;
	} = $props();

	/**
	 * Start window drag on primary mouse button press outside interactive elements.
	 * @param e - The mouse event from the title bar's mousedown listener.
	 */
	function handleDragStart(e: MouseEvent): void {
		if (e.button !== 0) return;
		const target = e.target as HTMLElement;
		if (target.closest("button, [data-menu-bar]")) return;
		getCurrentWindow().startDragging();
	}

	/**
	 * Toggle window maximize state on double-click outside interactive elements.
	 * @param e - The mouse event from the title bar's dblclick listener.
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
	class="title-bar border-border bg-background relative z-50 flex h-10 items-center border-b pr-2"
	onmousedown={handleDragStart}
	ondblclick={handleDoubleClick}
>
	{@render children?.()}
</div>

<style>
	/* Tauri drag region — the entire title bar is draggable. */
	.title-bar {
		-webkit-app-region: drag;
	}

	/* Buttons and menu bars are interactive and must not be captured by the drag region. */
	.title-bar :global(button),
	.title-bar :global([data-menu-bar]) {
		cursor: pointer;
		-webkit-app-region: no-drag;
	}
</style>

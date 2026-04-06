<script lang="ts">
	// Root layout for OrqaDev. Wraps the entire app in TooltipProvider and
	// mounts the DevToolsShell, which owns tab navigation and the status bar.
	// Also registers the global keyboard shortcuts for the entire app:
	//   Ctrl+F / Cmd+F  — focus the log search input
	//   Ctrl+L / Cmd+L  — clear the log event buffer
	//   Ctrl+1–4        — switch to the corresponding tab
	//   Escape          — clear all active log filters
	import "../app.css";
	import { TooltipProvider, Stack } from "@orqastudio/svelte-components/pure";
	import DevToolsShell from "$lib/components/DevToolsShell.svelte";
	import { navigation } from "$lib/stores/devtools-navigation.svelte.js";
	import { clearFilters, clearEvents } from "$lib/stores/log-store.svelte.js";

	let { children } = $props();

	// Returns true when the event originated from an element that accepts keyboard
	// input — input, textarea, select, or any contenteditable. Used to suppress
	// shortcuts that should not fire while the user is typing.
	/**
	 *
	 * @param e
	 */
	function isInputFocused(e: KeyboardEvent): boolean {
		const target = e.target as HTMLElement | null;
		if (!target) return false;
		const tag = target.tagName.toLowerCase();
		return tag === "input" || tag === "textarea" || tag === "select" || target.isContentEditable;
	}

	// Global keydown handler. Checks modifier keys and the active element before
	// dispatching to the appropriate store action or DOM operation.
	/**
	 *
	 * @param e
	 */
	function handleKeydown(e: KeyboardEvent): void {
		const mod = e.ctrlKey || e.metaKey;

		// Ctrl+F / Cmd+F — focus the log search input.
		// This shortcut fires even when an input is focused so the user can always
		// jump to the search box without first clicking away. We switch to the Logs
		// tab first in case the user is on another tab.
		if (mod && e.key === "f") {
			e.preventDefault();
			navigation.activeTab = "logs";
			// Use rAF so the Logs tab content is mounted before we try to focus.
			requestAnimationFrame(() => {
				const searchInput = document.getElementById("log-search-input");
				searchInput?.focus();
				(searchInput as HTMLInputElement | null)?.select();
			});
			return;
		}

		// All remaining shortcuts must not fire when the user is typing in an input.
		if (isInputFocused(e)) return;

		// Ctrl+L / Cmd+L — clear the log display buffer.
		if (mod && e.key === "l") {
			e.preventDefault();
			clearEvents();
			return;
		}

		// Escape — clear all active log filters.
		if (e.key === "Escape") {
			clearFilters();
			return;
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<TooltipProvider>
	<Stack gap={0} height="screen" width="screen">
		<DevToolsShell>
			{@render children()}
		</DevToolsShell>
	</Stack>
</TooltipProvider>

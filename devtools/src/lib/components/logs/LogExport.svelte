<!-- Export button for the log toolbar. Serialises the currently filtered log
     events to a pretty-printed JSON array and saves them via Tauri's native
     save dialog. The exported file contains every field of each LogEvent so
     the output is useful for offline analysis and bug reports. -->
<script lang="ts">
	import { save } from "@tauri-apps/plugin-dialog";
	import { writeTextFile } from "@tauri-apps/plugin-fs";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { filteredEvents as getFilteredEvents } from "../../stores/log-store.svelte.js";
	import type { LogEvent } from "../../stores/log-store.svelte.js";

	const filteredEvents = $derived(getFilteredEvents());

	// Whether a save operation is in progress (prevents double-clicks).
	let saving = $state(false);

	/**
	 * Format today's date as YYYY-MM-DD for use in the default export file name.
	 * @returns The current date in YYYY-MM-DD format.
	 */
	function todayString(): string {
		const d = new Date();
		const yyyy = d.getFullYear();
		const mm = (d.getMonth() + 1).toString().padStart(2, "0");
		const dd = d.getDate().toString().padStart(2, "0");
		return `${yyyy}-${mm}-${dd}`;
	}

	/**
	 * Serialise the filtered event list to a pretty-printed JSON string for export.
	 * @param evs - The log events to serialise.
	 * @returns A formatted JSON array string.
	 */
	function serialiseEvents(evs: LogEvent[]): string {
		return JSON.stringify(evs, null, 2);
	}

	/**
	 * Open the native save dialog and write the serialised filtered events to the chosen path.
	 * @returns Resolves when the file is saved or the dialog is cancelled.
	 */
	async function handleExport(): Promise<void> {
		if (saving) return;
		saving = true;
		try {
			const defaultPath = `orqadev-logs-${todayString()}.json`;
			const path = await save({
				defaultPath,
				filters: [{ name: "JSON", extensions: ["json"] }],
				title: "Export OrqaDev Logs",
			});
			if (path === null) {
				// User cancelled the dialog.
				return;
			}
			const content = serialiseEvents(filteredEvents);
			await writeTextFile(path, content);
		} finally {
			saving = false;
		}
	}
</script>

<!-- Export button: xs size fits the compact 24px log toolbar row height. -->
<Button
	variant="ghost"
	size="xs"
	onclick={handleExport}
	disabled={saving || filteredEvents.length === 0}
	title="Export visible logs as JSON"
	aria-label="Export logs"
>
	{saving ? "Saving…" : "Export"}
</Button>

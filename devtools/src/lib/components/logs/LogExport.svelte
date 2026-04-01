<!-- Export button for the log toolbar. Serialises the currently filtered log
     events to a pretty-printed JSON array and saves them via Tauri's native
     save dialog. The exported file contains every field of each LogEvent so
     the output is useful for offline analysis and bug reports. -->
<script lang="ts">
	import { save } from "@tauri-apps/plugin-dialog";
	import { writeTextFile } from "@tauri-apps/plugin-fs";
	import { filteredEvents } from "../../stores/log-store.svelte.js";
	import type { LogEvent } from "../../stores/log-store.svelte.js";

	// Whether a save operation is in progress (prevents double-clicks).
	let saving = $state(false);

	// Format today's date as YYYY-MM-DD for the default file name.
	function todayString(): string {
		const d = new Date();
		const yyyy = d.getFullYear();
		const mm = (d.getMonth() + 1).toString().padStart(2, "0");
		const dd = d.getDate().toString().padStart(2, "0");
		return `${yyyy}-${mm}-${dd}`;
	}

	// Serialise the filtered event list to a pretty-printed JSON string.
	// All fields from LogEvent are included: timestamp, level, source,
	// category, message, metadata, session_id, and id.
	function serialiseEvents(evs: LogEvent[]): string {
		return JSON.stringify(evs, null, 2);
	}

	// Open the native save dialog, let the user choose a file path, then
	// write the serialised events to that path. No-ops if the user cancels.
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

<!-- Export button: compact, sits in the log toolbar alongside Clear. -->
<button
	class="rounded px-1.5 py-0.5 text-[10px] text-content-muted transition-colors hover:bg-surface-raised hover:text-content-base disabled:opacity-40"
	onclick={handleExport}
	disabled={saving || filteredEvents.length === 0}
	title="Export visible logs as JSON"
	aria-label="Export logs"
>
	{saving ? "Saving…" : "Export"}
</button>

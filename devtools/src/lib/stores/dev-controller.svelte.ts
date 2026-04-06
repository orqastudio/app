// Reactive state for the dev environment controller. Tracks whether dev
// processes are running and provides start/stop actions that call the Rust
// backend IPC commands.

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type DevControllerState = "stopped" | "starting" | "running" | "stopping";

export const devController = $state<{
	state: DevControllerState;
}>({
	state: "stopped",
});

// Guard against SSR — Tauri APIs require the browser window.
if (typeof window !== "undefined") {
	// Check initial status on load.
	invoke<boolean>("devtools_dev_status").then((running) => {
		if (running) {
			devController.state = "running";
		}
	});

	// Listen for state changes from the Rust backend.
	$effect.root(() => {
		listen<{ state: string }>("orqa://dev-controller-state", (event) => {
			const s = event.payload.state;
			if (s === "stopped" || s === "starting" || s === "running" || s === "stopping") {
				devController.state = s as DevControllerState;
			}
		});
	});
}

/**
 *
 */
export async function startDev(): Promise<void> {
	if (devController.state !== "stopped") return;
	devController.state = "starting";
	try {
		await invoke("devtools_start_dev");
	} catch (err) {
		console.error("Failed to start dev environment:", err);
		devController.state = "stopped";
	}
}

/**
 *
 */
export async function stopDev(): Promise<void> {
	if (devController.state !== "running") return;
	devController.state = "stopping";
	try {
		await invoke("devtools_stop_dev");
	} catch (err) {
		console.error("Failed to stop dev environment:", err);
	}
}

// Issue group store for OrqaDev. Wraps the `devtools_list_issue_groups` and
// `devtools_get_issue_group` IPC commands and subscribes to the Tauri
// `orqa://issue-group-update` event to keep the reactive list current without
// reloading the full set. Exposes sort and filter controls so the UI can drive
// the query parameters reactively.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

// Shape of an issue group as returned by the backend IPC commands.
export interface IssueGroup {
	fingerprint: string;
	title: string;
	component: string;
	level: string;
	first_seen: number; // Unix ms
	last_seen: number; // Unix ms
	count: number;
	sparkline_buckets: number[];
	recent_event_ids: number[];
}

// Valid sort columns for `devtools_list_issue_groups`.
export type IssueSortBy = "last_seen" | "count" | "level" | "component";

// Valid sort directions.
export type SortDir = "asc" | "desc";

// The Tauri event name emitted after each issue group upsert.
const TAURI_ISSUE_EVENT = "orqa://issue-group-update";

// Reactive list of issue groups. Components read this array directly.
export const issueGroups = $state<IssueGroup[]>([]);

// Fingerprint of the currently selected issue, or null when nothing is selected.
export const selectedFingerprint = $state<{ value: string | null }>({ value: null });

// Active sort column. Changes trigger a reload when loadIssueGroups is called.
export const sortBy = $state<{ value: IssueSortBy }>({ value: "last_seen" });

// Active sort direction. Changes trigger a reload when loadIssueGroups is called.
export const sortDir = $state<{ value: SortDir }>({ value: "desc" });

// Optional component filter. Empty string means no filter applied.
export const filterComponent = $state<{ value: string }>({ value: "" });

// Optional severity-level filter. Empty string means no filter applied.
export const filterLevel = $state<{ value: string }>({ value: "" });

// Whether a load is currently in flight. Used to prevent double-submits.
export const issueLoading = $state<{ value: boolean }>({ value: false });

// Holds the Tauri unlisten function so we can clean up on destroy.
let unlisten: UnlistenFn | null = null;

/**
 * Derive the selected issue from the current group list and selection state.
 * Returns the matching IssueGroup, or null when nothing is selected or the
 * fingerprint no longer exists in the loaded list.
 * @returns The currently selected IssueGroup, or null.
 */
export function selectedIssue(): IssueGroup | null {
	if (selectedFingerprint.value === null) return null;
	return issueGroups.find((g) => g.fingerprint === selectedFingerprint.value) ?? null;
}

/**
 * Load issue groups from the backend using the current sort and filter state.
 * Results replace the current `issueGroups` array. A no-op while a previous
 * load is in flight.
 */
export async function loadIssueGroups(): Promise<void> {
	if (issueLoading.value) return;

	issueLoading.value = true;
	try {
		const params: {
			sort_by?: IssueSortBy;
			sort_dir?: SortDir;
			filter_component?: string;
			filter_level?: string;
		} = {
			sort_by: sortBy.value,
			sort_dir: sortDir.value,
		};

		if (filterComponent.value.length > 0) {
			params.filter_component = filterComponent.value;
		}
		if (filterLevel.value.length > 0) {
			params.filter_level = filterLevel.value;
		}

		const result = await invoke<IssueGroup[]>("devtools_list_issue_groups", { params });
		issueGroups.splice(0, issueGroups.length, ...result);
	} catch (err) {
		console.error("[issue-store] loadIssueGroups failed:", err);
	} finally {
		issueLoading.value = false;
	}
}

/**
 * Set the selected fingerprint. The `selectedIssue` derived function will
 * return the matching group from the current list.
 * @param fingerprint - The fingerprint of the issue group to select.
 */
export function selectIssue(fingerprint: string): void {
	selectedFingerprint.value = fingerprint;
}

/**
 * Clear the current selection. `selectedIssue` will return null afterwards.
 */
export function clearSelection(): void {
	selectedFingerprint.value = null;
}

/**
 * Update a single issue group in the reactive array in-place. Finds the
 * existing entry by fingerprint and patches only the fields that change on
 * upsert: count, last_seen, and sparkline_buckets. If the fingerprint is not
 * found (new group) the updated group is appended to the list.
 * @param updated - The updated IssueGroup payload from the Tauri event.
 */
function applyUpdate(updated: IssueGroup): void {
	const idx = issueGroups.findIndex((g) => g.fingerprint === updated.fingerprint);
	if (idx >= 0) {
		// Patch mutable fields in-place to preserve reactivity granularity.
		issueGroups[idx].count = updated.count;
		issueGroups[idx].last_seen = updated.last_seen;
		issueGroups[idx].sparkline_buckets = updated.sparkline_buckets;
		issueGroups[idx].recent_event_ids = updated.recent_event_ids;
	} else {
		// New group not yet in the list — append it.
		issueGroups.push(updated);
	}
}

/**
 * Initialise the issue store. Loads the first page of issue groups from the
 * backend and subscribes to `orqa://issue-group-update` so new upserts are
 * reflected in the list without a full reload. Safe to call once on mount.
 * @returns A cleanup function that removes the Tauri event listener.
 */
export async function init(): Promise<() => void> {
	await loadIssueGroups();

	if (unlisten === null) {
		unlisten = await listen<IssueGroup>(TAURI_ISSUE_EVENT, (tauri_event) => {
			applyUpdate(tauri_event.payload);
		});
	}

	return () => {
		if (unlisten !== null) {
			unlisten();
			unlisten = null;
		}
	};
}

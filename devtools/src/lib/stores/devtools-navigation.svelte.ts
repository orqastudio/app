// Navigation state store for OrqaDev. Tracks which tab is active using
// Svelte 5 $state so components re-render reactively on tab changes.

export type DevToolsTab = "logs" | "processes" | "storybook" | "metrics";

// All four tabs with their display labels, ordered for the tab bar.
export const TABS: { value: DevToolsTab; label: string }[] = [
	{ value: "logs", label: "Logs" },
	{ value: "processes", label: "Processes" },
	{ value: "storybook", label: "Storybook" },
	{ value: "metrics", label: "Metrics" },
];

// Module-level reactive state. Exported as a plain object so any component
// can read `navigation.activeTab` and write `navigation.activeTab = "logs"`.
export const navigation = $state<{ activeTab: DevToolsTab }>({
	activeTab: "logs",
});

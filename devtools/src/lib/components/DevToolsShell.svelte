<!-- Main layout shell for OrqaDev. Renders a full-height three-row layout:
     top tab bar, scrollable content area, and a bottom status bar. The
     navigation store drives which tab is active; only the Logs tab renders
     real content — the other three show a placeholder until their tasks land.
     Mounts the HelpPanel and wires the ? shortcut to open it.
     The status bar shows daemon connection state sourced from navigation.connection,
     which is updated by the orqa://connection-state Tauri event. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { TabsRoot, TabsList, TabsTrigger, TabsContent } from "@orqastudio/svelte-components/pure";
	import { StatusBar } from "@orqastudio/svelte-components/connected";
	import { navigation, TABS, connectionLabel } from "../stores/devtools-navigation.svelte.js";
	import StorybookView from "./storybook/StorybookView.svelte";
	import MetricsView from "./metrics/MetricsView.svelte";
	import ProcessView from "./processes/ProcessView.svelte";
	import HelpPanel from "./help/HelpPanel.svelte";

	let {
		children,
	}: {
		// Content rendered inside the Logs tab (the only live tab in TASK-26).
		children: Snippet;
	} = $props();

	// Running count of events received this session, shown in the status bar.
	let eventCount = $state(0);

	// Whether the help panel is open. Toggled by the ? key and the help icon.
	let helpOpen = $state(false);

	// Dot colour for the connection indicator driven by navigation.connection:
	// green when connected, yellow when reconnecting, red when waiting for daemon.
	const daemonDotClass = $derived(
		navigation.connection.state === "connected"
			? "bg-green-500"
			: navigation.connection.state === "reconnecting"
				? "bg-yellow-500"
				: "bg-red-500",
	);

	// Handle the ? keyboard shortcut to open/close the help panel. Ignores the
	// keystroke when the user is typing in an input, textarea, or select so the
	// shortcut does not fire mid-search.
	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === "?" && !e.ctrlKey && !e.metaKey && !e.altKey) {
			const target = e.target as HTMLElement;
			const tag = target.tagName.toLowerCase();
			if (tag === "input" || tag === "textarea" || tag === "select") return;
			e.preventDefault();
			helpOpen = !helpOpen;
		}
	}
</script>

<svelte:document onkeydown={handleKeydown} />

<!-- Shell fills the full viewport height; layout.svelte owns the outer container. -->
<div class="flex h-full flex-1 flex-col overflow-hidden">
	<!-- Tab bar: full-width strip pinned to the top. Uses raw Tabs parts so we
	     can override the default w-fit/rounded-lg styling with a border-bottom
	     bar that matches common devtools conventions. -->
	<TabsRoot
		bind:value={navigation.activeTab}
		class="flex h-full flex-col overflow-hidden gap-0"
	>
		<TabsList
			class="bg-surface-base border-b border-border h-9 w-full rounded-none px-2 inline-flex items-center justify-start gap-1"
		>
			{#each TABS as tab (tab.value)}
				<TabsTrigger
					value={tab.value}
					class="rounded-sm px-3 py-1 text-sm font-medium text-content-muted h-7
					       data-[state=active]:bg-surface-raised data-[state=active]:text-content-base
					       data-[state=active]:shadow-none hover:text-content-base transition-colors"
				>
					{tab.label}
				</TabsTrigger>
			{/each}

			<!-- Help icon button: pushes to the right via ml-auto. Opens the help panel. -->
			<button
				class="ml-auto flex size-6 items-center justify-center rounded text-content-muted transition-colors hover:bg-surface-raised hover:text-content-base focus:outline-none focus-visible:ring-1 focus-visible:ring-blue-500"
				onclick={() => { helpOpen = !helpOpen; }}
				aria-label="Open help panel (shortcut: ?)"
				title="Help (?)"
			>
				<svg width="13" height="13" viewBox="0 0 13 13" fill="none" aria-hidden="true">
					<circle cx="6.5" cy="6.5" r="5.5" stroke="currentColor" stroke-width="1.2"/>
					<path d="M5 5c0-1.1.9-1.5 1.5-1.5S8 4 8 5c0 .8-.6 1.1-1.2 1.5C6.2 6.9 6 7.3 6 7.8" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
					<circle cx="6.5" cy="9.5" r=".6" fill="currentColor"/>
				</svg>
			</button>
		</TabsList>

		<!-- Logs tab — live content passed in from the route. -->
		<TabsContent value="logs" class="flex-1 overflow-hidden">
			{@render children()}
		</TabsContent>

		<!-- Processes tab — live process status grid. -->
		<TabsContent value="processes" class="flex-1 overflow-hidden">
			<ProcessView />
		</TabsContent>

		<TabsContent value="storybook" class="flex-1 overflow-hidden">
			<StorybookView />
		</TabsContent>

		<TabsContent value="metrics" class="flex-1 overflow-hidden">
			<MetricsView />
		</TabsContent>
	</TabsRoot>

	<!-- Status bar: pinned to the bottom, shows daemon connection state and event count.
	     Connection label is derived from navigation.connection via connectionLabel(). -->
	<StatusBar>
		{#snippet left()}
			<!-- Connection status dot + label. Dot is green/yellow/red per state. -->
			<span class="flex items-center gap-1.5">
				<span class="size-2 rounded-full {daemonDotClass}"></span>
				<span>{connectionLabel(navigation.connection)}</span>
			</span>
		{/snippet}
		{#snippet right()}
			<!-- Event count and buffer usage placeholder -->
			<span>{eventCount} events</span>
		{/snippet}
	</StatusBar>
</div>

<!-- Help panel: mounted outside the flex column so it can overlay the full viewport. -->
<HelpPanel bind:open={helpOpen} />

<!-- Main layout shell for OrqaDev. Renders a full-height three-row layout:
     top tab bar, scrollable content area, and a bottom status bar. The
     navigation store drives which tab is active; only the Logs tab renders
     real content — the other three show a placeholder until their tasks land. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { TabsRoot, TabsList, TabsTrigger, TabsContent } from "@orqastudio/svelte-components/pure";
	import { StatusBar } from "@orqastudio/svelte-components/connected";
	import { navigation, TABS } from "../stores/devtools-navigation.svelte.js";

	let {
		children,
	}: {
		// Content rendered inside the Logs tab (the only live tab in TASK-26).
		children: Snippet;
	} = $props();

	// Daemon connection status displayed in the status bar. Starts as
	// "connecting" and will be updated by the log store once it is wired up.
	let daemonStatus = $state<"connected" | "connecting" | "disconnected">("connecting");

	// Running count of events received this session, shown in the status bar.
	let eventCount = $state(0);

	// Human-readable label for the daemon connection state.
	const daemonLabel = $derived(
		daemonStatus === "connected"
			? "Daemon connected"
			: daemonStatus === "disconnected"
				? "Daemon disconnected"
				: "Connecting…",
	);

	// Dot colour for the connection indicator: green / yellow / red.
	const daemonDotClass = $derived(
		daemonStatus === "connected"
			? "bg-green-500"
			: daemonStatus === "disconnected"
				? "bg-red-500"
				: "bg-yellow-500",
	);
</script>

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
		</TabsList>

		<!-- Logs tab — live content passed in from the route. -->
		<TabsContent value="logs" class="flex-1 overflow-hidden">
			{@render children()}
		</TabsContent>

		<!-- Remaining tabs show placeholders until their implementation tasks land. -->
		<TabsContent value="processes" class="flex-1 overflow-hidden">
			<div class="text-content-muted flex h-full items-center justify-center text-sm">
				Processes view — coming soon
			</div>
		</TabsContent>

		<TabsContent value="storybook" class="flex-1 overflow-hidden">
			<div class="text-content-muted flex h-full items-center justify-center text-sm">
				Storybook view — coming soon
			</div>
		</TabsContent>

		<TabsContent value="metrics" class="flex-1 overflow-hidden">
			<div class="text-content-muted flex h-full items-center justify-center text-sm">
				Metrics view — coming soon
			</div>
		</TabsContent>
	</TabsRoot>

	<!-- Status bar: pinned to the bottom, shows daemon connection and counters. -->
	<StatusBar>
		{#snippet left()}
			<!-- Connection status dot + label -->
			<span class="flex items-center gap-1.5">
				<span class="size-2 rounded-full {daemonDotClass}"></span>
				<span>{daemonLabel}</span>
			</span>
		{/snippet}
		{#snippet right()}
			<!-- Event count and buffer usage placeholder -->
			<span>{eventCount} events</span>
		{/snippet}
	</StatusBar>
</div>

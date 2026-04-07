<!-- Main layout shell for OrqaDev. Composes exclusively from the shared
     component library — no raw Tailwind classes.
     Always shows the workspace — the CLI starts processes before opening devtools. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import {
		resolveIcon,
		ConnectionIndicator,
		Caption,
		EventDrawer,
		StackFrameList,
		ContextTable,
		RawJson,
		AiExplainButton,
		HStack,
		Box,
		type ContextEntry,
	} from "@orqastudio/svelte-components/pure";
	import { emit } from "@tauri-apps/api/event";
	import {
		AppShell,
		ActivityBar,
		StatusBar,
		type ActivityBarItem,
	} from "@orqastudio/svelte-components/connected";
	import {
		navigation,
		type DevToolsTab,
		connectionLabel,
	} from "../stores/devtools-navigation.svelte.js";
	import { assertNever } from "@orqastudio/types";
	import type { LogEvent } from "../stores/log-store.svelte.js";
	import {
		isDrawerOpen,
		getDrawerEvent,
		getDrawerTab,
		closeDrawer,
		nextEvent,
		prevEvent,
		setTab,
	} from "../stores/drawer-store.svelte.js";
	import { devController } from "../stores/dev-controller.svelte.js";
	import {
		viewingHistorical,
		activeSessionId,
		sessions,
		sessionDisplayLabel,
	} from "../stores/session-store.svelte.js";
	import StorybookView from "./storybook/StorybookView.svelte";
	import MetricsView from "./metrics/MetricsView.svelte";
	import ProcessView from "./processes/ProcessView.svelte";
	import HelpPanel from "./help/HelpPanel.svelte";
	import IssuesView from "./issues/IssuesView.svelte";
	import TraceView from "./trace/TraceView.svelte";
	import { selectTrace } from "../stores/trace-store.svelte.js";

	let {
		children,
	}: {
		children: Snippet;
	} = $props();

	let helpOpen = $state(false);

	const connectionState = $derived(
		navigation.connection.state === "connected"
			? ("connected" as const)
			: navigation.connection.state === "reconnecting"
				? ("reconnecting" as const)
				: ("waiting" as const),
	);

	const NAV_DEFS: { key: DevToolsTab; icon: string; label: string }[] = [
		{ key: "processes", icon: "cpu", label: "Processes" },
		{ key: "stream", icon: "list", label: "Stream" },
		{ key: "issues", icon: "alert-circle", label: "Issues" },
		{ key: "storybook", icon: "book-open", label: "Storybook" },
		{ key: "metrics", icon: "activity", label: "Metrics" },
		{ key: "trace", icon: "git-branch", label: "Trace" },
	];

	const topItems: ActivityBarItem[] = $derived(
		NAV_DEFS.map((def) => ({
			icon: resolveIcon(def.icon),
			label: def.label,
			key: def.key,
			active: navigation.activeTab === def.key,
			onclick: () => {
				navigation.activeTab = def.key;
			},
		})),
	);

	const bottomItems: ActivityBarItem[] = $derived([
		{
			icon: resolveIcon("settings"),
			label: "Help (?)",
			key: "help",
			active: helpOpen,
			onclick: () => {
				helpOpen = !helpOpen;
			},
		},
	]);

	// Exhaustiveness guard for the active tab switch in the template.
	// Called in the {:else} branch — if a new DevToolsTab variant is added without
	// updating the template, this will throw at compile time (never type check).
	/**
	 * Exhaustiveness guard: throws if tab is an unhandled DevToolsTab variant.
	 * @param tab - The unhandled tab value; TypeScript narrows this to never.
	 * @returns Never returns — always throws.
	 */
	function assertTabNever(tab: never): never {
		return assertNever(tab);
	}

	/**
	 * Build the context entries array for ContextTable from a log event's fields.
	 * Extracts scalar fields (source, level, category, session_id, correlation_id,
	 * fingerprint) and flattens metadata key-value pairs when metadata is an object.
	 * @param event - The log event to extract context from, or null.
	 * @returns Ordered array of ContextEntry objects for display.
	 */
	function buildContextEntries(event: LogEvent | null): ContextEntry[] {
		if (!event) return [];
		const entries: ContextEntry[] = [
			{ key: "source", value: event.source },
			{ key: "level", value: event.level },
			{ key: "category", value: event.category },
		];
		if (event.session_id) {
			entries.push({ key: "session_id", value: event.session_id, copyable: true });
		}
		if (event.correlation_id) {
			entries.push({ key: "correlation_id", value: event.correlation_id, copyable: true });
		}
		if (event.fingerprint) {
			entries.push({ key: "fingerprint", value: event.fingerprint, copyable: true });
		}
		// Flatten metadata when it is a plain object — add each key-value pair.
		if (
			event.metadata !== null &&
			typeof event.metadata === "object" &&
			!Array.isArray(event.metadata)
		) {
			for (const [k, v] of Object.entries(event.metadata as Record<string, unknown>)) {
				entries.push({ key: k, value: String(v) });
			}
		}
		return entries;
	}

	/**
	 * Handle the AI explain action: emit the built prompt as a Tauri event
	 * so the app's chat view can pick it up and populate the input.
	 * @param prompt - The formatted explanation prompt built from the selected event.
	 */
	async function handleAiExplain(prompt: string): Promise<void> {
		await emit("orqa://ai-explain-request", prompt);
	}

	/**
	 * Handle a ContextTable value click. When the user clicks the correlation_id
	 * entry, selects that ID in the trace store and navigates to the Trace tab so
	 * the timeline renders the correlated events immediately.
	 * @param key - The context entry key that was clicked.
	 * @param value - The context entry value that was clicked.
	 */
	function handleContextValueClick(key: string, value: string): void {
		if (key === "correlation_id") {
			selectTrace(value);
			navigation.activeTab = "trace";
		}
	}

	// Reactive references to drawer state via getter functions. These re-run
	// any time the underlying $state values change so the template stays current.
	const drawerOpen = $derived(isDrawerOpen());
	const drawerEvent = $derived(getDrawerEvent());
	const drawerTab = $derived(getDrawerTab());

	/**
	 * Handle keydown: ? key toggles the help panel when not typing in an input field.
	 * Ctrl+1–6 navigate directly to the corresponding tab.
	 * @param e - The keyboard event from the document keydown listener.
	 */
	function handleKeydown(e: KeyboardEvent): void {
		if (e.key === "?" && !e.ctrlKey && !e.metaKey && !e.altKey) {
			const target = e.target as HTMLElement;
			const tag = target.tagName.toLowerCase();
			if (tag === "input" || tag === "textarea" || tag === "select") return;
			e.preventDefault();
			helpOpen = !helpOpen;
		}

		const mod = e.ctrlKey || e.metaKey;
		if (mod && e.key >= "1" && e.key <= String(NAV_DEFS.length)) {
			const index = parseInt(e.key, 10) - 1;
			const def = NAV_DEFS[index];
			if (def) {
				e.preventDefault();
				navigation.activeTab = def.key;
			}
		}
	}
</script>

<svelte:document onkeydown={handleKeydown} />

<AppShell showNavPanel={false} showChatPanel={false}>
	{#snippet activityBar()}
		<ActivityBar {topItems} {bottomItems} />
	{/snippet}

	{#snippet mainContent()}
		<!-- Main content area + optional EventDrawer side panel. The drawer renders
			     alongside the active tab so it persists across tab switches. -->
		<HStack height="full">
			<Box flex={1} minWidth={0} height="full">
				{#if navigation.activeTab === "issues"}
					<IssuesView />
				{:else if navigation.activeTab === "stream"}
					{@render children()}
				{:else if navigation.activeTab === "processes"}
					<ProcessView />
				{:else if navigation.activeTab === "storybook"}
					<StorybookView />
				{:else if navigation.activeTab === "metrics"}
					<MetricsView />
				{:else if navigation.activeTab === "trace"}
					<TraceView />
				{:else}
					{@const _exhaustive = assertTabNever(navigation.activeTab)}
					{_exhaustive}
				{/if}
			</Box>

			<EventDrawer
				open={drawerOpen}
				event={drawerEvent}
				activeTab={drawerTab}
				onclose={closeDrawer}
				onnext={nextEvent}
				onprev={prevEvent}
				ontabchange={setTab}
			>
				{#snippet toolbarContent()}
					<AiExplainButton event={drawerEvent} onexplain={handleAiExplain} />
				{/snippet}
				{#snippet stackContent()}
					<StackFrameList frames={drawerEvent?.stack_frames ?? []} />
				{/snippet}
				{#snippet contextContent()}
					<ContextTable
						entries={buildContextEntries(drawerEvent)}
						onValueClick={handleContextValueClick}
					/>
				{/snippet}
				{#snippet rawContent()}
					<RawJson data={drawerEvent} />
				{/snippet}
			</EventDrawer>
		</HStack>
	{/snippet}

	{#snippet statusBar()}
		<StatusBar>
			{#snippet left()}
				<ConnectionIndicator
					state={connectionState}
					label={connectionLabel(navigation.connection)}
				/>
			{/snippet}
			{#snippet right()}
				{#if viewingHistorical.value}
					{@const session = sessions.find((s) => s.id === activeSessionId.value)}
					<!-- Box constrains width so Caption truncate works within the status bar slot. -->
					<Box maxWidth="60">
						<Caption truncate tone="primary" italic>
							Viewing: {session ? sessionDisplayLabel(session) : "historical session"}
						</Caption>
					</Box>
				{:else}
					<Caption>{devController.state}</Caption>
				{/if}
			{/snippet}
		</StatusBar>
	{/snippet}
</AppShell>

<HelpPanel bind:open={helpOpen} />

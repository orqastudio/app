<!-- Main layout shell for OrqaDev. Composes exclusively from the shared
     component library — no raw Tailwind classes.
     - Stopped: WelcomeHero with "Start Dev Environment" button
     - Running: AppShell with ActivityBar, StatusBar, and content views -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { Button, LoadingSpinner, resolveIcon, ConnectionIndicator, Caption } from "@orqastudio/svelte-components/pure";
	import { AppShell, ActivityBar, StatusBar, WelcomeHero, type ActivityBarItem } from "@orqastudio/svelte-components/connected";
	import { navigation, type DevToolsTab, connectionLabel } from "../stores/devtools-navigation.svelte.js";
	import { assertNever } from "@orqastudio/types";
	import {
		devController,
		startDev,
		stopDev,
	} from "../stores/dev-controller.svelte.js";
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
	import setupBackground from "$lib/assets/setup-background.png";
	import finMark from "$lib/assets/fin-mark.svg";

	let {
		children,
	}: {
		children: Snippet;
	} = $props();

	let helpOpen = $state(false);

	const connectionState = $derived(
		navigation.connection.state === "connected"
			? "connected" as const
			: navigation.connection.state === "reconnecting"
				? "reconnecting" as const
				: "waiting" as const,
	);

	const isBusy = $derived(
		devController.state === "starting" || devController.state === "stopping",
	);
	const showWorkspace = $derived(devController.state === "running");

	const NAV_DEFS: { key: DevToolsTab; icon: string; label: string }[] = [
		{ key: "logs", icon: "terminal", label: "Logs" },
		{ key: "processes", icon: "cpu", label: "Processes" },
		{ key: "storybook", icon: "book-open", label: "Storybook" },
		{ key: "metrics", icon: "activity", label: "Metrics" },
	];

	const topItems: ActivityBarItem[] = $derived(
		NAV_DEFS.map((def) => ({
			icon: resolveIcon(def.icon),
			label: def.label,
			key: def.key,
			active: navigation.activeTab === def.key,
			onclick: () => { navigation.activeTab = def.key; },
		})),
	);

	const bottomItems: ActivityBarItem[] = $derived([
		{
			icon: resolveIcon("circle-stop"),
			label: "Stop Dev Environment",
			key: "stop",
			active: false,
			onclick: () => { if (!isBusy) stopDev(); },
		},
		{
			icon: resolveIcon("settings"),
			label: "Help (?)",
			key: "help",
			active: helpOpen,
			onclick: () => { helpOpen = !helpOpen; },
		},
	]);

	// Exhaustiveness guard for the active tab switch in the template.
	// Called in the {:else} branch — if a new DevToolsTab variant is added without
	// updating the template, this will throw at compile time (never type check).
	function assertTabNever(tab: never): never {
		return assertNever(tab);
	}

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

{#if showWorkspace}
	<AppShell showNavPanel={false} showChatPanel={false}>
		{#snippet activityBar()}
			<ActivityBar {topItems} {bottomItems} />
		{/snippet}

		{#snippet mainContent()}
			{#if navigation.activeTab === "logs"}
				{@render children()}
			{:else if navigation.activeTab === "processes"}
				<ProcessView />
			{:else if navigation.activeTab === "storybook"}
				<StorybookView />
			{:else if navigation.activeTab === "metrics"}
				<MetricsView />
			{:else}
				{@const _exhaustive = assertTabNever(navigation.activeTab)}
				{_exhaustive}
			{/if}
		{/snippet}

		{#snippet statusBar()}
			<StatusBar>
				{#snippet left()}
					<ConnectionIndicator state={connectionState} label={connectionLabel(navigation.connection)} />
				{/snippet}
				{#snippet right()}
					{#if viewingHistorical.value}
						{@const session = sessions.find((s) => s.id === activeSessionId.value)}
						<Caption class="overflow-hidden text-ellipsis whitespace-nowrap italic text-primary max-w-[240px]">
							Viewing: {session ? sessionDisplayLabel(session) : "historical session"}
						</Caption>
					{:else}
						<Caption>{devController.state}</Caption>
					{/if}
				{/snippet}
			</StatusBar>
		{/snippet}
	</AppShell>
{:else}
	<WelcomeHero
		backgroundImage={setupBackground}
		logoSrc={finMark}
		logoAlt="OrqaStudio"
		title="OrqaDev"
		subtitle="Developer tools for OrqaStudio"
	>
		{#if devController.state === "starting"}
			<LoadingSpinner />
		{:else if devController.state === "stopping"}
			<LoadingSpinner />
		{:else}
			<Button variant="default" size="lg" onclick={startDev}>
				Start Dev Environment
			</Button>
		{/if}
	</WelcomeHero>
{/if}

<HelpPanel bind:open={helpOpen} />


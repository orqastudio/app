<script lang="ts">
	import { onMount, onDestroy, getContext } from "svelte";
	import { listen } from "@tauri-apps/api/event";
	import type { UnlistenFn } from "@tauri-apps/api/event";
	import ActivityBar from "./ActivityBar.svelte";
	import NavSubPanel from "./NavSubPanel.svelte";
	import Toolbar from "./Toolbar.svelte";
	import StatusBar from "./StatusBar.svelte";
	import WelcomeScreen from "./WelcomeScreen.svelte";
	import ExplorerRouter from "./ExplorerRouter.svelte";
	import ArtifactNav from "$lib/components/navigation/ArtifactNav.svelte";
	import SettingsView from "$lib/components/settings/SettingsView.svelte";
	import ConversationView from "$lib/components/conversation/ConversationView.svelte";
	import ProjectSetupWizard from "$lib/components/settings/ProjectSetupWizard.svelte";
	import SetupWizard from "$lib/components/setup/SetupWizard.svelte";
	import ArtifactSearchOverlay from "$lib/components/navigation/ArtifactSearchOverlay.svelte";
	import { ErrorToast } from "@orqastudio/svelte-components/connected";
	import { getStores, logger } from "@orqastudio/sdk";
	import { initDevConsole } from "$lib/utils/dev-console";

	const log = logger("lifecycle");

	import { ResizablePaneGroup, ResizableHandle, ResizablePane, Stack, HStack, Box } from "@orqastudio/svelte-components/pure";
	import setupBackground from "$lib/assets/setup-background.png";

	const { errorStore, navigationStore, settingsStore, artifactStore, projectStore, setupStore, enforcementStore, artifactGraphSDK } = getStores();

	/** Promise that resolves once all plugins are registered (schemas available). */
	const pluginsReady = getContext<Promise<void>>("pluginsReady");

	/** Unlisten function for the artifact-changed event, cleaned up on destroy. */
	let unlistenArtifactChanged: UnlistenFn | null = null;

	const hasProject = $derived(projectStore.hasProject);
	const groupHasMultipleSubCategories = $derived(
		navigationStore.activeGroup !== null &&
		navigationStore.groupSubCategories[navigationStore.activeGroup].length > 1,
	);
	const needsSetup = $derived(projectStore.settingsLoaded && !projectStore.hasSettings);
	const hideChatPanel = $derived(
		navigationStore.activeActivity === "settings",
	);
	const setupNeeded = $derived(!setupStore.setupComplete);

	function handleGlobalKeydown(e: KeyboardEvent) {
		// Ctrl+Space (or Cmd+Space on Mac) toggles the search overlay
		if (e.code === "Space" && (e.ctrlKey || e.metaKey)) {
			e.preventDefault();
			navigationStore.toggleSearch();
		}
	}

	onMount(async () => {
		log.info("app shell mounted");
		settingsStore.initialize();
		errorStore.initialize();
		errorStore.initBrowserHandlers();
		initDevConsole();
		await setupStore.checkSetupStatus();
		if (setupStore.setupComplete) {
			projectStore.loadActiveProject();
		}

		// Listen for backend file-watcher events and refresh the nav tree.
		// Also reload project settings so new artifact types in project.json
		// appear immediately without requiring an app restart.
		unlistenArtifactChanged = await listen("artifact-changed", async () => {
			artifactStore.invalidateNavTree();
			if (projectStore.projectPath) {
				await projectStore.loadProjectSettings(projectStore.projectPath);
			}
		});

		window.addEventListener("keydown", handleGlobalKeydown);
	});

	onDestroy(() => {
		settingsStore.destroy();
		errorStore.destroy();
		unlistenArtifactChanged?.();
		window.removeEventListener("keydown", handleGlobalKeydown);
	});

	// When a project becomes active, switch to the project dashboard
	$effect(() => {
		if (hasProject && !needsSetup && navigationStore.activeActivity === "chat") {
			navigationStore.setActivity("project");
		}
	});


	// Load the unified navigation tree once the project is ready
	$effect(() => {
		if (hasProject && !needsSetup && artifactStore.navTree === null) {
			artifactStore.loadNavTree();
		}
	});

	// Initialize the artifact graph SDK when a project becomes active.
	// Wait for plugins to be registered first so all schema type keys are
	// available — otherwise _fetchAll runs with zero types and the graph
	// appears empty until the schema-count $effect fires a re-fetch.
	$effect(() => {
		const project = projectStore.activeProject;
		if (!project || needsSetup) return;
		void pluginsReady.then(() =>
			artifactGraphSDK.initialize({ projectPath: project.path }),
		);
	});

	// Load enforcement rules and violation history when the rules activity is active
	$effect(() => {
		const activity = navigationStore.activeActivity;
		if (hasProject && !needsSetup && activity === "rules") {
			enforcementStore.loadRules();
			enforcementStore.loadViolationHistory();
		}
	});

	// Auto-load artifact content when the selected artifact path changes
	$effect(() => {
		const path = navigationStore.selectedArtifactPath;
		if (!path || !hasProject || needsSetup) return;
		artifactStore.loadContent(path);
	});
</script>

<Stack gap={0} height="screen" width="screen" align="stretch" overflow="hidden">
	<!-- Toolbar -->
	<Toolbar />

	<!-- Main Content Area. align="stretch" is required so children fill the
	     available height — HStack's default is align="center" which collapses
	     children to content height and breaks nested h-full centering. -->
	<HStack gap={0} flex={1} align="stretch" overflow="hidden">
		{#if setupNeeded}
			<!-- First-run setup wizard — blocks all other content -->
			<SetupWizard
				onComplete={() => {
					projectStore.loadActiveProject();
				}}
			/>
		{:else if hasProject && needsSetup}
			<!-- Project needs setup — show wizard only, no chat/nav/activity bar -->
			<div
				class="relative flex-1 overflow-hidden"
				style="background-image: url({setupBackground}); background-size: cover; background-position: center;"
			>
				<div class="absolute inset-0 bg-background/70"></div>
				<div class="relative z-10 flex h-full w-full items-center justify-center px-4">
					<div class="w-full max-w-lg">
						<ProjectSetupWizard
							projectPath={projectStore.projectPath ?? ""}
							onComplete={() => {}}
						/>
					</div>
				</div>
			</div>
		{:else if hasProject}
			<!-- Activity Bar (48px fixed width) — project only -->
			<ActivityBar />

			<!-- Level 2: Nav Sub-Panel (200px) — group sub-categories or settings nav -->
			{#if navigationStore.showNavPanel && (navigationStore.activeGroup === null || groupHasMultipleSubCategories)}
				<NavSubPanel />
			{/if}

			<!-- Level 3: Artifact List Panel — shows individual artifacts within the active category -->
			{#if navigationStore.isArtifactActivity}
				<div class="flex w-[240px] flex-col overflow-hidden border-r border-border">
					<ArtifactNav category={navigationStore.activeActivity} />
				</div>
			{/if}

			<!-- Explorer + Chat (resizable) -->
			{#if hideChatPanel}
				<Box flex={1} minWidth={0} overflow="hidden">
					{#if navigationStore.activeActivity === "settings"}
						<SettingsView />
					{:else}
						<WelcomeScreen />
					{/if}
				</Box>
			{:else}
				<Box flex={1} minWidth={0}>
					<ResizablePaneGroup direction="horizontal">
						<ResizablePane defaultSize={70} minSize={30}>
							<ExplorerRouter />
						</ResizablePane>
						<ResizableHandle />
						<ResizablePane defaultSize={30} minSize={20}>
							<div class="flex h-full flex-col bg-chat">
								<ConversationView />
							</div>
						</ResizablePane>
					</ResizablePaneGroup>
				</Box>
			{/if}

		{:else}
			<!-- No project loaded — welcome screen, no sidebar -->
			<Box flex={1} overflow="hidden">
				<WelcomeScreen />
			</Box>
		{/if}
	</HStack>

	<!-- Status Bar -->
	<StatusBar />

	<!-- Global artifact search overlay -->
	<ArtifactSearchOverlay />

	<!-- Global error toast — surfaces backend, sidecar, and frontend errors -->
	<ErrorToast />
</Stack>

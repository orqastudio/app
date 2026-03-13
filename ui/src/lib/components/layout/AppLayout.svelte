<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { listen } from "@tauri-apps/api/event";
	import type { UnlistenFn } from "@tauri-apps/api/event";
	import ActivityBar from "./ActivityBar.svelte";
	import NavSubPanel from "./NavSubPanel.svelte";
	import Toolbar from "./Toolbar.svelte";
	import StatusBar from "./StatusBar.svelte";
	import WelcomeScreen from "./WelcomeScreen.svelte";
	import ProjectDashboard from "$lib/components/dashboard/ProjectDashboard.svelte";
	import ArtifactViewer from "$lib/components/artifact/ArtifactViewer.svelte";
	import SettingsView from "$lib/components/settings/SettingsView.svelte";
	import ConversationView from "$lib/components/conversation/ConversationView.svelte";
	import ProjectSetupWizard from "$lib/components/settings/ProjectSetupWizard.svelte";
	import SetupWizard from "$lib/components/setup/SetupWizard.svelte";
	import ArtifactSearchOverlay from "$lib/components/navigation/ArtifactSearchOverlay.svelte";
	import ErrorToast from "$lib/components/shared/ErrorToast.svelte";
	import { errorStore } from "$lib/stores/errors.svelte";
	import { initDevConsole } from "$lib/utils/dev-console";

	import ArtifactMasterDetail from "$lib/components/artifact/ArtifactMasterDetail.svelte";
	import * as Resizable from "$lib/components/ui/resizable";
	import setupBackground from "$lib/assets/setup-background.png";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { setupStore } from "$lib/stores/setup.svelte";
	import { enforcementStore } from "$lib/stores/enforcement.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";

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
		settingsStore.initialize();
		errorStore.initialize();
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
	// The SDK starts the file watcher, builds the graph, and listens for
	// "artifact-graph-updated" events for auto-refresh.
	$effect(() => {
		const project = projectStore.activeProject;
		if (!project || needsSetup) return;
		void artifactGraphSDK.initialize({ projectPath: project.path });
	});

	// Load enforcement rules when the rules activity is active
	$effect(() => {
		const activity = navigationStore.activeActivity;
		if (hasProject && !needsSetup && activity === "rules") {
			enforcementStore.loadRules();
		}
	});

	// Auto-load artifact content when the selected artifact path changes
	$effect(() => {
		const path = navigationStore.selectedArtifactPath;
		if (!path || !hasProject || needsSetup) return;
		artifactStore.loadContent(path);
	});
</script>

<div class="flex h-screen flex-col bg-background text-foreground">
	<!-- Toolbar -->
	<Toolbar />

	<!-- Main Content Area -->
	<div class="flex flex-1 overflow-hidden">
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
				class="relative flex flex-1 items-center justify-center overflow-hidden"
				style="background-image: url({setupBackground}); background-size: cover; background-position: center;"
			>
				<div class="absolute inset-0 bg-background/70"></div>
				<div class="relative z-10 w-full max-w-lg px-4">
					<ProjectSetupWizard
						projectPath={projectStore.projectPath ?? ""}
						onComplete={() => {}}
					/>
				</div>
			</div>
		{:else if hasProject}
			<!-- Activity Bar (48px fixed width) — project only -->
			<ActivityBar />

			<!-- Nav Sub-Panel (200px) — shown for settings/chat and for groups with
			     more than one sub-category (Planning, Team, Governance). Hidden for
			     Documentation (single sub-category) and single-item views like Orchestrator. -->
			{#if navigationStore.showNavPanel && (navigationStore.activeGroup === null || groupHasMultipleSubCategories)}
				<NavSubPanel />
			{/if}

			<!-- Explorer + Chat (resizable) -->
			{#if hideChatPanel}
				<div class="min-w-0 flex-1 overflow-hidden">
					{#if navigationStore.activeActivity === "settings"}
						<SettingsView />
					{:else}
						<WelcomeScreen />
					{/if}
				</div>
			{:else}
				<Resizable.PaneGroup direction="horizontal" class="flex-1">
					<Resizable.Pane defaultSize={70} minSize={30}>
						<div class="h-full overflow-hidden">
							{#if navigationStore.activeActivity === "project"}
								<ProjectDashboard />
							{:else if navigationStore.activeActivity === "chat"}
								<WelcomeScreen />
							{:else if navigationStore.activeGroup !== null}
								{#if navigationStore.activeActivity === "orchestrator"}
									<ArtifactViewer />
								{:else}
									<ArtifactMasterDetail activity={navigationStore.activeActivity} />
								{/if}
							{:else if navigationStore.isArtifactActivity}
								{#if navigationStore.explorerView === "artifact-viewer"}
									<ArtifactViewer />
								{:else}
									<div class="flex h-full items-center justify-center text-sm text-muted-foreground">
										Select an item to view it
									</div>
								{/if}
							{:else}
								<WelcomeScreen />
							{/if}
						</div>
					</Resizable.Pane>
					<Resizable.Handle />
					<Resizable.Pane defaultSize={30} minSize={20}>
						<div class="flex h-full flex-col bg-chat">
							<ConversationView />
						</div>
					</Resizable.Pane>
				</Resizable.PaneGroup>
			{/if}

		{:else}
			<!-- No project loaded — welcome screen, no sidebar -->
			<div class="flex flex-1 overflow-hidden">
				<div class="flex-1 overflow-hidden">
					<WelcomeScreen />
				</div>
			</div>
		{/if}
	</div>

	<!-- Status Bar -->
	<StatusBar />

	<!-- Global artifact search overlay -->
	<ArtifactSearchOverlay />

	<!-- Global error toast — surfaces backend, sidecar, and frontend errors -->
	<ErrorToast />
</div>

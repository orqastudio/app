<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
	import { Input } from "$lib/components/ui/input";
	import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
	import ScanSearchIcon from "@lucide/svelte/icons/scan-search";
	import SaveIcon from "@lucide/svelte/icons/save";
	import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
	import { projectStore } from "$lib/stores/project.svelte";
	import type { ProjectSettings, ProjectScanResult } from "$lib/types";

	interface Props {
		projectPath: string;
		onComplete: (settings: ProjectSettings) => void;
	}

	const props: Props = $props();

	const defaultName = $derived(() => {
		const segments = props.projectPath.replace(/\\/g, "/").split("/").filter(Boolean);
		const name = segments[segments.length - 1] ?? "project";
		return name.charAt(0).toUpperCase() + name.slice(1);
	});
	let projectName = $state("");
	let scanResult = $state<ProjectScanResult | null>(null);
	let scanned = $state(false);
	let nameInitialized = $state(false);

	$effect(() => {
		if (!nameInitialized) {
			projectName = defaultName();
			nameInitialized = true;
		}
	});

	async function handleScan() {
		const result = await projectStore.scanProject(props.projectPath);
		if (result) {
			scanResult = result;
			scanned = true;
		}
	}

	async function handleSave() {
		if (!scanResult) return;
		const settings: ProjectSettings = {
			name: projectName,
			description: null,
			default_model: "auto",
			excluded_paths: ["node_modules", ".git", "target", "dist", "build"],
			stack: scanResult.stack,
			governance: scanResult.governance,
			icon: null,
			show_thinking: false,
			custom_system_prompt: null,
		};
		await projectStore.saveProjectSettings(props.projectPath, settings);
		await projectStore.loadProjectSettings(props.projectPath);
		props.onComplete(settings);
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Set Up Project</Card.Title>
		<Card.Description>
			No configuration found. Scan this project to detect its stack and create settings.
		</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-4">
		<div class="flex items-center gap-2 text-sm">
			<FolderOpenIcon class="h-4 w-4 text-muted-foreground" />
			<span class="font-mono text-xs text-muted-foreground">{props.projectPath}</span>
		</div>

		<Separator />

		<div>
			<label class="text-sm font-medium" for="wizard-project-name">Project Name</label>
			<Input
				id="wizard-project-name"
				class="mt-1 max-w-xs"
				bind:value={projectName}
				placeholder="Project name"
			/>
		</div>

		{#if !scanned}
			<Button
				variant="outline"
				onclick={handleScan}
				disabled={projectStore.scanning}
			>
				{#if projectStore.scanning}
					<LoaderCircleIcon class="mr-1.5 h-3.5 w-3.5 animate-spin" />
					Scanning...
				{:else}
					<ScanSearchIcon class="mr-1.5 h-3.5 w-3.5" />
					Scan Project
				{/if}
			</Button>
		{/if}

		{#if scanResult}
			<Separator />
			<div class="space-y-3">
				<h4 class="text-sm font-medium">Detected Stack</h4>
				{#if scanResult.stack.languages.length > 0}
					<div class="flex flex-wrap gap-1.5">
						{#each scanResult.stack.languages as lang (lang)}
							<Badge variant="secondary">{lang}</Badge>
						{/each}
					</div>
				{:else}
					<p class="text-xs text-muted-foreground">No languages detected</p>
				{/if}

				{#if scanResult.stack.frameworks.length > 0}
					<div class="flex flex-wrap gap-1.5">
						{#each scanResult.stack.frameworks as fw (fw)}
							<Badge variant="outline">{fw}</Badge>
						{/each}
					</div>
				{/if}

				{#if scanResult.stack.package_manager}
					<p class="text-xs text-muted-foreground">
						Package manager: {scanResult.stack.package_manager}
					</p>
				{/if}
			</div>

			<div class="space-y-2">
				<h4 class="text-sm font-medium">Governance</h4>
				<div class="grid grid-cols-3 gap-2 text-xs">
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{scanResult.governance.docs}</div>
						<div class="text-muted-foreground">Docs</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{scanResult.governance.agents}</div>
						<div class="text-muted-foreground">Agents</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{scanResult.governance.rules}</div>
						<div class="text-muted-foreground">Rules</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{scanResult.governance.skills}</div>
						<div class="text-muted-foreground">Skills</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{scanResult.governance.hooks}</div>
						<div class="text-muted-foreground">Hooks</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">
							{scanResult.governance.has_claude_config ? "Yes" : "No"}
						</div>
						<div class="text-muted-foreground">CLAUDE.md</div>
					</div>
				</div>
			</div>

			<p class="text-xs text-muted-foreground">
				Scanned in {scanResult.scan_duration_ms}ms
			</p>

			<Separator />

			<Button onclick={handleSave}>
				<SaveIcon class="mr-1.5 h-3.5 w-3.5" />
				Save Configuration
			</Button>
		{/if}
	</Card.Content>
</Card.Root>

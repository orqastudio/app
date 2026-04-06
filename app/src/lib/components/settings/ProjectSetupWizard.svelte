<script lang="ts">
	import {
		Icon,
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
		FormGroup,
		Heading,
	} from "@orqastudio/svelte-components/pure";
	import {
		Badge,
		Button,
		HStack,
		Stack,
		Grid,
		Caption,
		Text,
		Panel,
	} from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { Input } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { projectStore } = getStores();
	import type { ProjectSettings, ProjectScanResult } from "@orqastudio/types";

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

	/** Scans the project path for stack and governance detection, updating the scan result state. */
	async function handleScan() {
		const result = await projectStore.scanProject(props.projectPath);
		if (result) {
			scanResult = result;
			scanned = true;
		}
	}

	/** Saves the project settings from the wizard form and calls the onSave prop. */
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

<CardRoot>
	<CardHeader>
		<CardTitle>Set Up Project</CardTitle>
		<CardDescription>
			No configuration found. Scan this project to detect its stack and create settings.
		</CardDescription>
	</CardHeader>
	<CardContent>
		<HStack gap={2}>
			<Icon name="folder-open" size="md" />
			<Caption variant="caption-mono" tone="muted">{props.projectPath}</Caption>
		</HStack>

		<Separator />

		<FormGroup label="Project Name" for="wizard-project-name">
			<Input id="wizard-project-name" bind:value={projectName} placeholder="Project name" />
		</FormGroup>

		{#if !scanned}
			<Button variant="outline" onclick={handleScan} disabled={projectStore.scanning}>
				{#if projectStore.scanning}
					<Icon name="loader-circle" size="sm" />
					Scanning...
				{:else}
					<Icon name="scan-search" size="sm" />
					Scan Project
				{/if}
			</Button>
		{/if}

		{#if scanResult}
			<Separator />
			<Stack gap={3}>
				<Heading level={4}>Detected Stack</Heading>
				{#if scanResult.stack.languages.length > 0}
					<HStack gap={1} wrap>
						{#each scanResult.stack.languages as lang (lang)}
							<Badge variant="secondary">{lang}</Badge>
						{/each}
					</HStack>
				{:else}
					<Caption tone="muted">No languages detected</Caption>
				{/if}

				{#if scanResult.stack.frameworks.length > 0}
					<HStack gap={1} wrap>
						{#each scanResult.stack.frameworks as fw (fw)}
							<Badge variant="outline">{fw}</Badge>
						{/each}
					</HStack>
				{/if}

				{#if scanResult.stack.package_manager}
					<Caption tone="muted">Package manager: {scanResult.stack.package_manager}</Caption>
				{/if}
			</Stack>

			<Stack gap={2}>
				<Heading level={4}>Governance</Heading>
				<Grid cols={3} gap={2}>
					<Panel padding="tight" border="all" rounded="md">
						<Stack gap={0} align="center">
							<Text variant="heading-base">{scanResult.governance.docs}</Text>
							<Caption tone="muted">Docs</Caption>
						</Stack>
					</Panel>
					<Panel padding="tight" border="all" rounded="md">
						<Stack gap={0} align="center">
							<Text variant="heading-base">{scanResult.governance.agents}</Text>
							<Caption tone="muted">Agents</Caption>
						</Stack>
					</Panel>
					<Panel padding="tight" border="all" rounded="md">
						<Stack gap={0} align="center">
							<Text variant="heading-base">{scanResult.governance.rules}</Text>
							<Caption tone="muted">Rules</Caption>
						</Stack>
					</Panel>
					<Panel padding="tight" border="all" rounded="md">
						<Stack gap={0} align="center">
							<Text variant="heading-base">{scanResult.governance.knowledge}</Text>
							<Caption tone="muted">Knowledge</Caption>
						</Stack>
					</Panel>
					<Panel padding="tight" border="all" rounded="md">
						<Stack gap={0} align="center">
							<Text variant="heading-base">{scanResult.governance.hooks}</Text>
							<Caption tone="muted">Hooks</Caption>
						</Stack>
					</Panel>
					<Panel padding="tight" border="all" rounded="md">
						<Stack gap={0} align="center">
							<Text variant="heading-base"
								>{scanResult.governance.has_claude_config ? "Yes" : "No"}</Text
							>
							<Caption tone="muted">CLAUDE.md</Caption>
						</Stack>
					</Panel>
				</Grid>
			</Stack>

			<Caption tone="muted">Scanned in {scanResult.scan_duration_ms}ms</Caption>

			<Separator />

			<Button onclick={handleSave}>
				<Icon name="save" size="sm" />
				Save Configuration
			</Button>
		{/if}
	</CardContent>
</CardRoot>

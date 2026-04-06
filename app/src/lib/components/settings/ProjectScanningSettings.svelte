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
		Caption,
		Text,
		Panel,
	} from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { Input } from "@orqastudio/svelte-components/pure";
	import { Textarea } from "@orqastudio/svelte-components/pure";
	import { SelectMenu } from "@orqastudio/svelte-components/pure";
	import { Switch } from "@orqastudio/svelte-components/pure";
	import type { ProjectSettings, ProjectScanResult } from "@orqastudio/types";
	import { CLAUDE_MODEL_OPTIONS } from "$lib/components/conversation/model-options";

	interface Props {
		settings: ProjectSettings;
		onSave: (settings: ProjectSettings) => void;
		onRescan: () => Promise<ProjectScanResult | null>;
		rescanning: boolean;
	}

	const props: Props = $props();

	let localModel = $state("auto");
	let localExcludedPaths = $state<string[]>([]);
	let newExcludedPath = $state("");
	let localShowThinking = $state(false);
	let localCustomPrompt = $state("");

	$effect(() => {
		localModel = props.settings.default_model;
		localExcludedPaths = [...props.settings.excluded_paths];
		localShowThinking = props.settings.show_thinking;
		localCustomPrompt = props.settings.custom_system_prompt ?? "";
	});

	/**
	 * Constructs a ProjectSettings object from the current local scanning form state.
	 * @returns The merged ProjectSettings with updated model, paths, and prompt.
	 */
	function buildSettings(): ProjectSettings {
		return {
			...props.settings,
			default_model: localModel,
			excluded_paths: localExcludedPaths,
			show_thinking: localShowThinking,
			custom_system_prompt: localCustomPrompt.trim() || null,
		};
	}

	const modelOptions = CLAUDE_MODEL_OPTIONS;

	/**
	 * Applies the selected model and saves settings.
	 * @param value - The model identifier to apply.
	 */
	function handleModelChange(value: string) {
		localModel = value;
		props.onSave(buildSettings());
	}

	/** Adds the current newExcludedPath input to the excluded paths list and saves. */
	function addExcludedPath() {
		const trimmed = newExcludedPath.trim();
		if (trimmed && !localExcludedPaths.includes(trimmed)) {
			localExcludedPaths = [...localExcludedPaths, trimmed];
			newExcludedPath = "";
			props.onSave(buildSettings());
		}
	}

	/**
	 * Removes an excluded path from the list and saves.
	 * @param path - The path string to remove.
	 */
	function removeExcludedPath(path: string) {
		localExcludedPaths = localExcludedPaths.filter((p) => p !== path);
		props.onSave(buildSettings());
	}

	/** Triggers a project rescan and merges the detected stack/governance into settings. */
	async function handleRescan() {
		const result = await props.onRescan();
		if (result) {
			props.onSave({
				...buildSettings(),
				stack: result.stack,
				governance: result.governance,
			});
		}
	}
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Model & Scanning</CardTitle>
		<CardDescription>Default model, excluded paths, and detected project stack</CardDescription>
	</CardHeader>
	<CardContent>
		<FormGroup
			label="Default Model"
			description={modelOptions.find((o) => o.value === localModel)?.description ?? ""}
		>
			<SelectMenu
				items={modelOptions}
				selected={localModel}
				onSelect={handleModelChange}
				triggerLabel={modelOptions.find((o) => o.value === localModel)?.label ?? "Auto"}
				triggerSize="default"
				align="start"
			/>
		</FormGroup>

		<Separator />

		<HStack justify="between">
			<Stack gap={0}>
				<Text variant="body-strong">Show Thinking</Text>
				<Caption tone="muted">Stream Claude's reasoning process during responses</Caption>
			</Stack>
			<Switch
				bind:checked={localShowThinking}
				size="sm"
				aria-label="Toggle show thinking"
				onCheckedChange={() => props.onSave(buildSettings())}
			/>
		</HStack>

		<Separator />

		<FormGroup
			label="Custom System Prompt"
			description="Prepended to the auto-generated governance prompt on every turn"
		>
			<Textarea
				bind:value={localCustomPrompt}
				placeholder="Enter custom instructions..."
				onblur={() => props.onSave(buildSettings())}
			/>
			{#if localCustomPrompt.trim()}
				<span class="text-muted-foreground text-xs"
					>{localCustomPrompt.trim().length} characters</span
				>
			{/if}
		</FormGroup>

		<Separator />

		<Stack gap={2}>
			<Heading level={4}>Excluded Paths</Heading>
			<HStack gap={1} wrap>
				{#each localExcludedPaths as path (path)}
					<Panel padding="tight" border="all" rounded="md">
						<HStack gap={1}>
							<Caption>{path}</Caption>
							<Button variant="ghost" size="icon-sm" onclick={() => removeExcludedPath(path)}>
								<Icon name="x" size="xs" />
							</Button>
						</HStack>
					</Panel>
				{/each}
			</HStack>
			<HStack gap={2}>
				<Input
					bind:value={newExcludedPath}
					placeholder="Add path..."
					onkeydown={(e: KeyboardEvent) => {
						if (e.key === "Enter") addExcludedPath();
					}}
				/>
				<Button
					variant="outline"
					size="sm"
					onclick={addExcludedPath}
					disabled={!newExcludedPath.trim()}
				>
					<Icon name="plus" size="sm" />
				</Button>
			</HStack>
		</Stack>

		<Separator />

		{#if props.settings.stack}
			<Stack gap={2}>
				<HStack justify="between">
					<Heading level={4}>Detected Stack</Heading>
					<Button variant="ghost" size="sm" onclick={handleRescan} disabled={props.rescanning}>
						{#if props.rescanning}
							<Icon name="loader-circle" size="sm" />
							Scanning...
						{:else}
							<Icon name="refresh-cw" size="sm" />
							Re-scan
						{/if}
					</Button>
				</HStack>
				{#if props.settings.stack.languages.length > 0}
					<Stack gap={1}>
						<Caption tone="muted">Languages</Caption>
						<HStack gap={1} wrap>
							{#each props.settings.stack.languages as lang (lang)}
								<Badge variant="secondary">{lang}</Badge>
							{/each}
						</HStack>
					</Stack>
				{/if}
				{#if props.settings.stack.frameworks.length > 0}
					<Stack gap={1}>
						<Caption tone="muted">Frameworks</Caption>
						<HStack gap={1} wrap>
							{#each props.settings.stack.frameworks as fw (fw)}
								<Badge variant="outline">{fw}</Badge>
							{/each}
						</HStack>
					</Stack>
				{/if}
				{#if props.settings.stack.package_manager}
					<Caption tone="muted">Package manager: {props.settings.stack.package_manager}</Caption>
				{/if}
			</Stack>
		{:else}
			<Stack gap={2}>
				<HStack justify="between">
					<Heading level={4}>Detected Stack</Heading>
					<Button variant="ghost" size="sm" onclick={handleRescan} disabled={props.rescanning}>
						{#if props.rescanning}
							<Icon name="loader-circle" size="sm" />
							Scanning...
						{:else}
							<Icon name="refresh-cw" size="sm" />
							Scan
						{/if}
					</Button>
				</HStack>
				<Caption tone="muted">No scan results yet. Click Scan to detect your project stack.</Caption
				>
			</Stack>
		{/if}
	</CardContent>
</CardRoot>

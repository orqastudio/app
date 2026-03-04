<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
	import { Input } from "$lib/components/ui/input";
	import SelectMenu from "$lib/components/shared/SelectMenu.svelte";
	import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
	import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
	import XIcon from "@lucide/svelte/icons/x";
	import PlusIcon from "@lucide/svelte/icons/plus";
	import type { ProjectSettings, ProjectScanResult } from "$lib/types";

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

	$effect(() => {
		localModel = props.settings.default_model;
		localExcludedPaths = [...props.settings.excluded_paths];
	});

	function buildSettings(): ProjectSettings {
		return {
			...props.settings,
			default_model: localModel,
			excluded_paths: localExcludedPaths,
		};
	}

	const modelOptions: { value: string; label: string; description: string }[] = [
		{ value: "auto", label: "Auto (recommended)", description: "Automatically selects the best model" },
		{ value: "claude-opus-4-6", label: "Opus", description: "Most capable, slower" },
		{ value: "claude-sonnet-4-6", label: "Sonnet", description: "Balanced performance" },
		{ value: "claude-haiku-4-5", label: "Haiku", description: "Fastest responses" },
	];

	function handleModelChange(value: string) {
		localModel = value;
		props.onSave(buildSettings());
	}

	function addExcludedPath() {
		const trimmed = newExcludedPath.trim();
		if (trimmed && !localExcludedPaths.includes(trimmed)) {
			localExcludedPaths = [...localExcludedPaths, trimmed];
			newExcludedPath = "";
			props.onSave(buildSettings());
		}
	}

	function removeExcludedPath(path: string) {
		localExcludedPaths = localExcludedPaths.filter((p) => p !== path);
		props.onSave(buildSettings());
	}

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

<Card.Root>
	<Card.Header>
		<Card.Title>Model & Scanning</Card.Title>
		<Card.Description>Default model, excluded paths, and detected project stack</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-4">
		<div>
			<span class="text-sm font-medium">Default Model</span>
			<div class="mt-1">
				<SelectMenu
					items={modelOptions}
					selected={localModel}
					onSelect={handleModelChange}
					triggerLabel={modelOptions.find((o) => o.value === localModel)?.label ?? "Auto"}
					triggerSize="default"
					align="start"
				/>
			</div>
			<p class="mt-1 text-xs text-muted-foreground">
				{modelOptions.find((o) => o.value === localModel)?.description ?? ""}
			</p>
		</div>

		<Separator />

		<div class="space-y-2">
			<h4 class="text-sm font-medium">Excluded Paths</h4>
			<div class="flex flex-wrap gap-1.5">
				{#each localExcludedPaths as path}
					<Badge variant="outline" class="gap-1 pr-1">
						{path}
						<button
							class="ml-0.5 rounded-sm hover:bg-muted"
							onclick={() => removeExcludedPath(path)}
						>
							<XIcon class="h-3 w-3" />
						</button>
					</Badge>
				{/each}
			</div>
			<div class="flex items-center gap-2">
				<Input
					class="max-w-[200px]"
					bind:value={newExcludedPath}
					placeholder="Add path..."
					onkeydown={(e: KeyboardEvent) => {
						if (e.key === "Enter") addExcludedPath();
					}}
				/>
				<Button variant="outline" size="sm" onclick={addExcludedPath} disabled={!newExcludedPath.trim()}>
					<PlusIcon class="h-3.5 w-3.5" />
				</Button>
			</div>
		</div>

		<Separator />

		{#if props.settings.stack}
			<div class="space-y-2">
				<div class="flex items-center justify-between">
					<h4 class="text-sm font-medium">Detected Stack</h4>
					<Button variant="ghost" size="sm" onclick={handleRescan} disabled={props.rescanning}>
						{#if props.rescanning}
							<LoaderCircleIcon class="mr-1.5 h-3.5 w-3.5 animate-spin" />
							Scanning...
						{:else}
							<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
							Re-scan
						{/if}
					</Button>
				</div>
				{#if props.settings.stack.languages.length > 0}
					<div class="space-y-1">
						<span class="text-xs text-muted-foreground">Languages</span>
						<div class="flex flex-wrap gap-1.5">
							{#each props.settings.stack.languages as lang}
								<Badge variant="secondary">{lang}</Badge>
							{/each}
						</div>
					</div>
				{/if}
				{#if props.settings.stack.frameworks.length > 0}
					<div class="space-y-1">
						<span class="text-xs text-muted-foreground">Frameworks</span>
						<div class="flex flex-wrap gap-1.5">
							{#each props.settings.stack.frameworks as fw}
								<Badge variant="outline">{fw}</Badge>
							{/each}
						</div>
					</div>
				{/if}
				{#if props.settings.stack.package_manager}
					<p class="text-xs text-muted-foreground">
						Package manager: {props.settings.stack.package_manager}
					</p>
				{/if}
			</div>
		{:else}
			<div class="space-y-2">
				<div class="flex items-center justify-between">
					<h4 class="text-sm font-medium">Detected Stack</h4>
					<Button variant="ghost" size="sm" onclick={handleRescan} disabled={props.rescanning}>
						{#if props.rescanning}
							<LoaderCircleIcon class="mr-1.5 h-3.5 w-3.5 animate-spin" />
							Scanning...
						{:else}
							<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
							Scan
						{/if}
					</Button>
				</div>
				<p class="text-xs text-muted-foreground">No scan results yet. Click Scan to detect your project stack.</p>
			</div>
		{/if}
	</Card.Content>
</Card.Root>

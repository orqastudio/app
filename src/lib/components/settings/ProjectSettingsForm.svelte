<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
	import { Input } from "$lib/components/ui/input";
	import { Textarea } from "$lib/components/ui/textarea";
	import { open } from "@tauri-apps/plugin-dialog";
	import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
	import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
	import XIcon from "@lucide/svelte/icons/x";
	import PlusIcon from "@lucide/svelte/icons/plus";
	import ImageIcon from "@lucide/svelte/icons/image";
	import UploadIcon from "@lucide/svelte/icons/upload";
	import TrashIcon from "@lucide/svelte/icons/trash-2";
	import type { ProjectSettings, ProjectScanResult } from "$lib/types";

	interface Props {
		settings: ProjectSettings;
		projectPath: string;
		onSave: (settings: ProjectSettings) => void;
		onRescan: () => Promise<ProjectScanResult | null>;
		rescanning: boolean;
		iconDataUrl: string | null;
		onUploadIcon: (sourcePath: string) => void;
		onRemoveIcon: () => void;
	}

	const props: Props = $props();

	let localName = $state("");
	let localDescription = $state("");
	let localModel = $state("auto");
	let localExcludedPaths = $state<string[]>([]);
	let newExcludedPath = $state("");

	// Sync local state from settings prop (initial + on change, e.g. after rescan)
	$effect(() => {
		localName = props.settings.name;
		localDescription = props.settings.description ?? "";
		localModel = props.settings.default_model;
		localExcludedPaths = [...props.settings.excluded_paths];
	});

	const modelOptions: { value: string; label: string; description: string }[] = [
		{ value: "auto", label: "Auto (recommended)", description: "Automatically selects the best model" },
		{ value: "claude-opus-4-6", label: "Opus", description: "Most capable, slower" },
		{ value: "claude-sonnet-4-6", label: "Sonnet", description: "Balanced performance" },
		{ value: "claude-haiku-4-5", label: "Haiku", description: "Fastest responses" },
	];

	function buildSettings(): ProjectSettings {
		return {
			...props.settings,
			name: localName,
			description: localDescription || null,
			default_model: localModel,
			excluded_paths: localExcludedPaths,
		};
	}

	function handleBlurSave() {
		props.onSave(buildSettings());
	}

	function handleModelChange(e: Event) {
		const target = e.target as HTMLSelectElement;
		localModel = target.value;
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

	async function handleIconUpload() {
		const selected = await open({
			multiple: false,
			title: "Select Project Icon",
			filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg", "svg", "ico"] }],
		});
		if (selected && typeof selected === "string") {
			props.onUploadIcon(selected);
		}
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Project Settings</Card.Title>
		<Card.Description>
			Configuration stored in <code class="text-xs">.forge/project.json</code>
		</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-4">
		<div class="space-y-2">
			<span class="text-sm font-medium">Project Icon</span>
			<div class="flex items-center gap-3">
				{#if props.iconDataUrl}
					<img
						src={props.iconDataUrl}
						alt="Project icon"
						class="h-10 w-10 rounded border object-contain"
					/>
				{:else}
					<div class="flex h-10 w-10 items-center justify-center rounded border bg-muted">
						<ImageIcon class="h-5 w-5 text-muted-foreground" />
					</div>
				{/if}
				<div class="flex gap-2">
					<Button variant="outline" size="sm" onclick={handleIconUpload}>
						<UploadIcon class="mr-1.5 h-3.5 w-3.5" />
						{props.iconDataUrl ? "Change" : "Upload"}
					</Button>
					{#if props.iconDataUrl}
						<Button variant="outline" size="sm" onclick={props.onRemoveIcon}>
							<TrashIcon class="mr-1.5 h-3.5 w-3.5" />
							Remove
						</Button>
					{/if}
				</div>
			</div>
		</div>

		<Separator />

		<div>
			<label class="text-sm font-medium" for="settings-name">Name</label>
			<Input
				id="settings-name"
				class="mt-1 max-w-xs"
				bind:value={localName}
				onblur={handleBlurSave}
			/>
		</div>

		<div>
			<label class="text-sm font-medium" for="settings-description">Description</label>
			<Textarea
				id="settings-description"
				class="mt-1 max-w-md"
				bind:value={localDescription}
				onblur={handleBlurSave}
				placeholder="Brief project description"
				rows={2}
			/>
		</div>

		<div>
			<label class="text-sm font-medium" for="settings-model">Default Model</label>
			<select
				id="settings-model"
				class="mt-1 flex h-9 w-full max-w-xs rounded-md border border-input bg-background px-3 py-1 text-sm"
				value={localModel}
				onchange={handleModelChange}
			>
				{#each modelOptions as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
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
			<div class="flex gap-2">
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
		{/if}

		{#if props.settings.governance}
			<div class="space-y-2">
				<h4 class="text-sm font-medium">Governance</h4>
				<div class="grid grid-cols-3 gap-2 text-xs">
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{props.settings.governance.docs}</div>
						<div class="text-muted-foreground">Docs</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{props.settings.governance.agents}</div>
						<div class="text-muted-foreground">Agents</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{props.settings.governance.rules}</div>
						<div class="text-muted-foreground">Rules</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{props.settings.governance.skills}</div>
						<div class="text-muted-foreground">Skills</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">{props.settings.governance.hooks}</div>
						<div class="text-muted-foreground">Hooks</div>
					</div>
					<div class="rounded border p-2 text-center">
						<div class="text-lg font-semibold">
							{props.settings.governance.has_claude_config ? "Yes" : "No"}
						</div>
						<div class="text-muted-foreground">CLAUDE.md</div>
					</div>
				</div>
			</div>
		{/if}
	</Card.Content>
</Card.Root>

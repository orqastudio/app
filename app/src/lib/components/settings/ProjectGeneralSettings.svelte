<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { Input } from "@orqastudio/svelte-components/pure";
	import { Textarea } from "@orqastudio/svelte-components/pure";
	import { open } from "@tauri-apps/plugin-dialog";
	import type { ProjectSettings } from "@orqastudio/types";

	interface Props {
		settings: ProjectSettings;
		onSave: (settings: ProjectSettings) => void;
		iconDataUrl: string | null;
		onUploadIcon: (sourcePath: string) => void;
		onRemoveIcon: () => void;
	}

	const props: Props = $props();

	let localName = $state("");
	let localDescription = $state("");

	$effect(() => {
		localName = props.settings.name;
		localDescription = props.settings.description ?? "";
	});

	/**
	 * Constructs a ProjectSettings object from the current local form state.
	 * @returns The merged ProjectSettings with updated name and description.
	 */
	function buildSettings(): ProjectSettings {
		return {
			...props.settings,
			name: localName,
			description: localDescription || null,
		};
	}

	/** Saves the current settings when an input loses focus. */
	function handleBlurSave() {
		props.onSave(buildSettings());
	}

	/** Opens a file picker and notifies the parent with the selected icon path. */
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

<CardRoot>
	<CardHeader>
		<CardTitle>General</CardTitle>
		<CardDescription>Project identity and description</CardDescription>
	</CardHeader>
	<CardContent class="space-y-4">
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
						<Icon name="image" size="lg" />
					</div>
				{/if}
				<div class="flex gap-2">
					<Button variant="outline" size="sm" onclick={handleIconUpload}>
						<Icon name="upload" size="sm" />
						{props.iconDataUrl ? "Change" : "Upload"}
					</Button>
					{#if props.iconDataUrl}
						<Button variant="outline" size="sm" onclick={props.onRemoveIcon}>
							<Icon name="trash-2" size="sm" />
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
	</CardContent>
</CardRoot>

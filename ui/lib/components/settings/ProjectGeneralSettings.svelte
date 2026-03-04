<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
	import { Input } from "$lib/components/ui/input";
	import { Textarea } from "$lib/components/ui/textarea";
	import { open } from "@tauri-apps/plugin-dialog";
	import ImageIcon from "@lucide/svelte/icons/image";
	import UploadIcon from "@lucide/svelte/icons/upload";
	import TrashIcon from "@lucide/svelte/icons/trash-2";
	import type { ProjectSettings } from "$lib/types";

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

	function buildSettings(): ProjectSettings {
		return {
			...props.settings,
			name: localName,
			description: localDescription || null,
		};
	}

	function handleBlurSave() {
		props.onSave(buildSettings());
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
		<Card.Title>General</Card.Title>
		<Card.Description>Project identity and description</Card.Description>
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
	</Card.Content>
</Card.Root>

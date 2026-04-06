<script lang="ts">
	import {
		Icon,
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
		FormGroup,
	} from "@orqastudio/svelte-components/pure";
	import { Button, HStack, Panel } from "@orqastudio/svelte-components/pure";
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
	<CardContent>
		<FormGroup label="Project Icon">
			<HStack gap={3}>
				{#if props.iconDataUrl}
					<!-- img is a legitimate exception — Image primitive follow-up needed -->
					<img
						src={props.iconDataUrl}
						alt="Project icon"
						class="h-10 w-10 rounded border object-contain"
					/>
				{:else}
					<Panel padding="tight" rounded="md" border="all" background="muted">
						<Icon name="image" size="lg" />
					</Panel>
				{/if}
				<HStack gap={2}>
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
				</HStack>
			</HStack>
		</FormGroup>

		<Separator />

		<FormGroup label="Name" for="settings-name">
			<Input id="settings-name" bind:value={localName} onblur={handleBlurSave} />
		</FormGroup>

		<FormGroup label="Description" for="settings-description">
			<Textarea
				id="settings-description"
				bind:value={localDescription}
				onblur={handleBlurSave}
				placeholder="Brief project description"
				rows={2}
			/>
		</FormGroup>
	</CardContent>
</CardRoot>

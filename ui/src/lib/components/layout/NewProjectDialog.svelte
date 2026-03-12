<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import * as Dialog from "$lib/components/ui/dialog";
	import FolderCodeIcon from "@lucide/svelte/icons/folder-code";
	import SquarePlusIcon from "@lucide/svelte/icons/square-plus";
	import { projectStore } from "$lib/stores/project.svelte";

	interface Props {
		open: boolean;
		onClose: () => void;
	}

	const { open: dialogOpen, onClose }: Props = $props();

	async function handleCreateFromScratch(): Promise<void> {
		onClose();
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Select Location for New Project",
		});
		if (selected && typeof selected === "string") {
			await projectStore.openProject(selected);
		}
	}

	async function handleInitializeExisting(): Promise<void> {
		onClose();
		const selected = await open({
			directory: true,
			multiple: false,
			title: "Select Folder to Initialize",
		});
		if (selected && typeof selected === "string") {
			await projectStore.openProject(selected);
		}
	}
</script>

<Dialog.Root
	open={dialogOpen}
	onOpenChange={(isOpen) => { if (!isOpen) onClose(); }}
>
	<Dialog.Content class="max-w-md">
		<Dialog.Header>
			<Dialog.Title>New Project</Dialog.Title>
			<Dialog.Description>Choose how to create your Orqa project.</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-3 py-2">
			<button
				class="flex items-start gap-4 rounded-lg border border-border p-4 text-left transition-colors hover:bg-accent"
				onclick={handleCreateFromScratch}
			>
				<SquarePlusIcon class="mt-0.5 h-6 w-6 shrink-0 text-primary" />
				<div>
					<p class="text-sm font-medium">Create From Scratch</p>
					<p class="text-xs text-muted-foreground">
						Start with a fresh project in an empty folder.
					</p>
				</div>
			</button>
			<button
				class="flex items-start gap-4 rounded-lg border border-border p-4 text-left transition-colors hover:bg-accent"
				onclick={handleInitializeExisting}
			>
				<FolderCodeIcon class="mt-0.5 h-6 w-6 shrink-0 text-primary" />
				<div>
					<p class="text-sm font-medium">Initialize Existing Folder</p>
					<p class="text-xs text-muted-foreground">
						Set up Orqa in an existing codebase. Your files stay untouched — only an .orqa/ config directory is added.
					</p>
				</div>
			</button>
		</div>
	</Dialog.Content>
</Dialog.Root>

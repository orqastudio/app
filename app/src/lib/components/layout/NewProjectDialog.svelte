<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import { Icon, DialogRoot, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { projectStore } = getStores();

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

<DialogRoot
	open={dialogOpen}
	onOpenChange={(isOpen) => { if (!isOpen) onClose(); }}
>
	<DialogContent class="max-w-md">
		<DialogHeader>
			<DialogTitle>New Project</DialogTitle>
			<DialogDescription>Choose how to create your Orqa project.</DialogDescription>
		</DialogHeader>
		<div class="grid gap-3 py-2">
			<button
				class="flex items-start gap-4 rounded-lg border border-border p-4 text-left transition-colors hover:bg-accent"
				onclick={handleCreateFromScratch}
			>
				<Icon name="square-plus" size="xl" />
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
				<Icon name="folder-code" size="xl" />
				<div>
					<p class="text-sm font-medium">Initialize Existing Folder</p>
					<p class="text-xs text-muted-foreground">
						Set up Orqa in an existing codebase. Your files stay untouched — only an .orqa/ config directory is added.
					</p>
				</div>
			</button>
		</div>
	</DialogContent>
</DialogRoot>

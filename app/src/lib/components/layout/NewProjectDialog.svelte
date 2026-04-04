<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import { Icon, Button, Stack, HStack, Text, DialogRoot, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@orqastudio/svelte-components/pure";
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
	<DialogContent>
		<DialogHeader>
			<DialogTitle>New Project</DialogTitle>
			<DialogDescription>Choose how to create your Orqa project.</DialogDescription>
		</DialogHeader>
		<Stack gap={2}>
			<Button
				variant="outline"
				onclick={handleCreateFromScratch}
			>
				<Icon name="square-plus" size="xl" />
				<Stack gap={1} align="start">
					<Text variant="label">Create From Scratch</Text>
					<Text variant="caption">Start with a fresh project in an empty folder.</Text>
				</Stack>
			</Button>
			<Button
				variant="outline"
				onclick={handleInitializeExisting}
			>
				<Icon name="folder-code" size="xl" />
				<Stack gap={1} align="start">
					<Text variant="label">Initialize Existing Folder</Text>
					<Text variant="caption">Set up Orqa in an existing codebase. Your files stay untouched — only an .orqa/ config directory is added.</Text>
				</Stack>
			</Button>
		</Stack>
	</DialogContent>
</DialogRoot>

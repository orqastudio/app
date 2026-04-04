<script lang="ts">
	import { DialogRoot, DialogContent, DialogHeader, DialogTitle, DialogDescription, HStack, Stack, Text } from "@orqastudio/svelte-components/pure";
	import logoPulse from "$lib/assets/logo-pulse.svg";

	interface Props {
		open: boolean;
		onClose: () => void;
		appName: string;
		appVersion: string;
	}

	const { open, onClose, appName, appVersion }: Props = $props();
</script>

<DialogRoot
	{open}
	onOpenChange={(isOpen) => { if (!isOpen) onClose(); }}
>
	<DialogContent>
		<DialogHeader>
			<HStack gap={3}>
				<img src={logoPulse} alt="OrqaStudio" class="h-10 w-10" />
				<Stack gap={1}>
					<DialogTitle>{appName}</DialogTitle>
					<DialogDescription>Version {appVersion}</DialogDescription>
				</Stack>
			</HStack>
		</DialogHeader>
		<Stack gap={2}>
			<Text tone="muted">A managed agentic development environment powered by Claude.</Text>
			<Text variant="caption" tone="muted">Built with Tauri, Svelte, and Rust.</Text>
		</Stack>
	</DialogContent>
</DialogRoot>

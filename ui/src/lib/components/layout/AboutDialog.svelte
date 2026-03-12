<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
	import logoPulse from "$lib/assets/logo-pulse.svg";

	interface Props {
		open: boolean;
		onClose: () => void;
		appName: string;
		appVersion: string;
	}

	const { open, onClose, appName, appVersion }: Props = $props();
</script>

<Dialog.Root
	{open}
	onOpenChange={(isOpen) => { if (!isOpen) onClose(); }}
>
	<Dialog.Content>
		<Dialog.Header>
			<div class="flex items-center gap-3">
				<img src={logoPulse} alt="OrqaStudio" class="h-10 w-10" />
				<div>
					<Dialog.Title>{appName}</Dialog.Title>
					<Dialog.Description>Version {appVersion}</Dialog.Description>
				</div>
			</div>
		</Dialog.Header>
		<div class="space-y-2 text-sm text-muted-foreground">
			<p>A managed agentic development environment powered by Claude.</p>
			<p class="text-xs">Built with Tauri, Svelte, and Rust.</p>
		</div>
	</Dialog.Content>
</Dialog.Root>

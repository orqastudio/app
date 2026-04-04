<script lang="ts">
	import { DialogRoot, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@orqastudio/svelte-components/pure";
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
			<div class="flex items-center gap-3">
				<img src={logoPulse} alt="OrqaStudio" class="h-10 w-10" />
				<div class="flex flex-col gap-1">
					<DialogTitle>{appName}</DialogTitle>
					<DialogDescription>Version {appVersion}</DialogDescription>
				</div>
			</div>
		</DialogHeader>
		<div class="flex flex-col gap-2">
			<span class="text-sm text-muted-foreground">A managed agentic development environment powered by Claude.</span>
			<span class="text-xs text-muted-foreground">Built with Tauri, Svelte, and Rust.</span>
		</div>
	</DialogContent>
</DialogRoot>

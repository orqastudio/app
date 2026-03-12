<script lang="ts">
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import MinusIcon from "@lucide/svelte/icons/minus";
	import SquareIcon from "@lucide/svelte/icons/square";
	import CopyIcon from "@lucide/svelte/icons/copy";
	import XIcon from "@lucide/svelte/icons/x";

	let isMaximized = $state(false);

	$effect(() => {
		getCurrentWindow()
			.isMaximized()
			.then((m) => {
				isMaximized = m;
			});
	});

	async function minimize() {
		await getCurrentWindow().minimize();
	}

	async function toggleMaximize() {
		const win = getCurrentWindow();
		if (isMaximized) {
			await win.unmaximize();
		} else {
			await win.maximize();
		}
		isMaximized = !isMaximized;
	}

	async function close() {
		await getCurrentWindow().close();
	}
</script>

<div class="flex items-center">
	<button
		class="flex h-8 w-10 items-center justify-center text-muted-foreground hover:bg-accent hover:text-foreground"
		onclick={minimize}
		title="Minimize"
	>
		<MinusIcon class="h-3.5 w-3.5" />
	</button>
	<button
		class="flex h-8 w-10 items-center justify-center text-muted-foreground hover:bg-accent hover:text-foreground"
		onclick={toggleMaximize}
		title={isMaximized ? "Restore" : "Maximize"}
	>
		{#if isMaximized}
			<CopyIcon class="h-3 w-3" />
		{:else}
			<SquareIcon class="h-3 w-3" />
		{/if}
	</button>
	<button
		class="flex h-8 w-10 items-center justify-center text-muted-foreground hover:bg-destructive hover:text-destructive-foreground"
		onclick={close}
		title="Close"
	>
		<XIcon class="h-3.5 w-3.5" />
	</button>
</div>

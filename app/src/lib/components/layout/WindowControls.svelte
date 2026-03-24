<script lang="ts">
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { Icon } from "@orqastudio/svelte-components/pure";

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
		aria-label="Minimize"
	>
		<Icon name="minus" size="sm" />
	</button>
	<button
		class="flex h-8 w-10 items-center justify-center text-muted-foreground hover:bg-accent hover:text-foreground"
		onclick={toggleMaximize}
		aria-label={isMaximized ? "Restore" : "Maximize"}
	>
		{#if isMaximized}
			<Icon name="copy" size="xs" />
		{:else}
			<Icon name="square" size="xs" />
		{/if}
	</button>
	<button
		class="flex h-8 w-10 items-center justify-center text-muted-foreground hover:bg-destructive hover:text-destructive-foreground"
		onclick={close}
		aria-label="Close"
	>
		<Icon name="x" size="sm" />
	</button>
</div>

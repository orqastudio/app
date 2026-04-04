<script lang="ts">
	import { onMount } from "svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { Icon, Button, HStack } from "@orqastudio/svelte-components/pure";

	let isMaximized = $state(false);

	// Fetch maximized state once on mount — no reactive dependencies need re-running this.
	onMount(() => {
		void getCurrentWindow()
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

<HStack gap={0}>
	<Button variant="ghost" size="icon-sm" onclick={minimize} aria-label="Minimize">
		<Icon name="minus" size="sm" />
	</Button>
	<Button variant="ghost" size="icon-sm" onclick={toggleMaximize} aria-label={isMaximized ? "Restore" : "Maximize"}>
		{#if isMaximized}
			<Icon name="copy" size="xs" />
		{:else}
			<Icon name="square" size="xs" />
		{/if}
	</Button>
	<Button variant="ghost" size="icon-sm" onclick={close} aria-label="Close">
		<Icon name="x" size="sm" />
	</Button>
</HStack>

<!-- WindowControls — minimize, maximize/restore, close buttons for custom Tauri title bars.
     Uses the Tauri window API to control the current window. Renders as a compact
     horizontal strip of ghost icon buttons matching standard window chrome layout. -->
<script lang="ts">
	import { onMount } from "svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { Icon } from "../icon/index.js";
	import { Button } from "../button/index.js";
	import { HStack } from "../layout/index.js";

	let isMaximized = $state(false);

	onMount(() => {
		void getCurrentWindow()
			.isMaximized()
			.then((m) => {
				isMaximized = m;
			});
	});

	/** Minimize the current window. */
	async function minimize() {
		await getCurrentWindow().minimize();
	}

	/** Toggle the current window between maximized and restored states. */
	async function toggleMaximize() {
		const win = getCurrentWindow();
		if (isMaximized) {
			await win.unmaximize();
		} else {
			await win.maximize();
		}
		isMaximized = !isMaximized;
	}

	/** Close the current window. */
	async function close() {
		await getCurrentWindow().close();
	}
</script>

<HStack gap={0}>
	<Button variant="ghost" size="icon-sm" onclick={minimize} aria-label="Minimize">
		<Icon name="minus" size="sm" />
	</Button>
	<Button
		variant="ghost"
		size="icon-sm"
		onclick={toggleMaximize}
		aria-label={isMaximized ? "Restore" : "Maximize"}
	>
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

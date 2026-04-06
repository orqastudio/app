<script lang="ts">
	// Welcome screen shown when no project is open. Delegates all layout to WelcomeHero.
	import { open } from "@tauri-apps/plugin-dialog";
	import { Icon, Button, LoadingSpinner, Text } from "@orqastudio/svelte-components/pure";
	import { WelcomeHero } from "@orqastudio/svelte-components/connected";
	import setupBackground from "$lib/assets/setup-background.png";
	import { getStores } from "@orqastudio/sdk";

	const { projectStore } = getStores();

	let opening = $state(false);

	/** Open the native folder picker and load the selected project. */
	async function handleOpenProject() {
		opening = true;
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: "Open Project Folder",
			});
			if (selected && typeof selected === "string") {
				await projectStore.openProject(selected);
			}
		} finally {
			opening = false;
		}
	}
</script>

<WelcomeHero
	backgroundImage={setupBackground}
	title="Welcome to OrqaStudio"
	subtitle="Open a project to get started"
>
	{#if opening}
		<LoadingSpinner />
	{:else}
		<Button variant="outline" onclick={handleOpenProject}>
			<Icon name="folder-open" size="md" />
			Open Project
		</Button>
	{/if}
	{#if projectStore.error}
		<Text variant="body" tone="destructive" block>{projectStore.error}</Text>
	{/if}
</WelcomeHero>

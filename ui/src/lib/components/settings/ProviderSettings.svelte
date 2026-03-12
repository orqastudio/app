<script lang="ts">
	import { setupStore } from "$lib/stores/setup.svelte";
	import SidecarStatusCard from "./SidecarStatusCard.svelte";
	import CliStatusCard from "./CliStatusCard.svelte";

	let cliChecking = $state(false);
	let reauthenticating = $state(false);

	async function handleCheckCli(): Promise<void> {
		cliChecking = true;
		await setupStore.checkCli();
		await setupStore.checkAuth();
		cliChecking = false;
	}

	async function handleReauthenticate(): Promise<void> {
		reauthenticating = true;
		await setupStore.reauthenticate();
		reauthenticating = false;
	}

	// Auto-check CLI info when this section mounts
	$effect(() => {
		if (!setupStore.cliInfo) {
			handleCheckCli();
		}
	});
</script>

<SidecarStatusCard />

<CliStatusCard
	{cliChecking}
	{reauthenticating}
	onCheckCli={handleCheckCli}
	onReauthenticate={handleReauthenticate}
/>

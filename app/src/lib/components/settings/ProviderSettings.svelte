<script lang="ts">
	import { getStores } from "@orqastudio/sdk";

	const { setupStore } = getStores();
	import SidecarStatusCard from "./SidecarStatusCard.svelte";
	import CliStatusCard from "./CliStatusCard.svelte";
	import ProviderSwitcher from "./ProviderSwitcher.svelte";

	let cliChecking = $state(false);
	let reauthenticating = $state(false);

	/** Checks the CLI installation status and auth state, updating the setup store. */
	async function handleCheckCli(): Promise<void> {
		cliChecking = true;
		await setupStore.checkCli();
		await setupStore.checkAuth();
		cliChecking = false;
	}

	/** Initiates re-authentication with Claude and waits for the result. */
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

<ProviderSwitcher />

<SidecarStatusCard />

<CliStatusCard
	{cliChecking}
	{reauthenticating}
	onCheckCli={handleCheckCli}
	onReauthenticate={handleReauthenticate}
/>

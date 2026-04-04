<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { pluginRegistry, toast } = getStores();

	const sidecars = $derived(pluginRegistry.sidecarProviders);
	const activeKey = $derived(pluginRegistry.activeSidecarKey);
	const hasMultiple = $derived(sidecars.length > 1);

	/**
	 * Switches the active sidecar provider and notifies the user to restart.
	 * @param key - The provider key to activate.
	 */
	function switchProvider(key: string) {
		pluginRegistry.setActiveSidecar(key);
		toast.success(`Switched to ${key}. Restart the app to apply.`);
	}
</script>

{#if sidecars.length > 0}
<CardRoot>
	<CardHeader compact>
		<CardTitle>
			<div class="flex items-center gap-1.5">
				<Icon name="cpu" size="md" />
				AI Provider
			</div>
		</CardTitle>
		<CardDescription>
			{#if hasMultiple}
				Select which AI provider to use for the sidecar.
			{:else}
				One provider registered. Install additional provider plugins for switching.
			{/if}
		</CardDescription>
	</CardHeader>
	<CardContent>
		<div class="flex flex-col gap-2">
			{#each sidecars as sidecar (sidecar.key)}
				{@const isActive = sidecar.key === activeKey}
				<div class="flex items-center justify-between rounded border px-3 py-2 {isActive ? 'border-primary bg-primary/5' : 'border-border'}">
					<div class="flex items-center gap-2">
						<Icon name={isActive ? "circle-check" : "circle-dashed"} size="sm" />
						<div class="flex flex-col gap-0">
							<span class="text-xs font-medium">{sidecar.label}</span>
							<span class="text-xs text-muted-foreground">{sidecar.runtime} &middot; {sidecar.entrypoint}</span>
						</div>
					</div>
					<div class="flex items-center gap-2">
						{#if isActive}
							<Badge variant="outline" size="xs">Active</Badge>
						{:else}
							<button
								class="flex h-7 items-center rounded px-2 text-xs hover:bg-accent"
								onclick={() => switchProvider(sidecar.key)}
							>
								Switch
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>

		{#if sidecars.length > 0}
			<div class="mt-3 flex items-start gap-2 rounded bg-muted/50 p-2">
				<Icon name="info" size="sm" />
				<span class="text-xs text-muted-foreground">
					Provider changes take effect after restarting the app. The sidecar
					will reconnect using the selected provider's configuration.
				</span>
			</div>
		{/if}
	</CardContent>
</CardRoot>
{/if}

<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
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
<CardRoot class="gap-2">
	<CardHeader class="pb-2">
		<CardTitle class="flex items-center gap-1.5 text-sm font-semibold">
			<Icon name="cpu" size="md" />
			AI Provider
		</CardTitle>
		<CardDescription class="text-xs">
			{#if hasMultiple}
				Select which AI provider to use for the sidecar.
			{:else}
				One provider registered. Install additional provider plugins for switching.
			{/if}
		</CardDescription>
	</CardHeader>
	<CardContent class="pt-0">
		<div class="space-y-2">
			{#each sidecars as sidecar (sidecar.key)}
				{@const isActive = sidecar.key === activeKey}
				<div
					class="flex items-center justify-between rounded border px-3 py-2 {isActive
						? 'border-primary bg-primary/5'
						: 'border-border'}"
				>
					<div class="flex items-center gap-2">
						<Icon name={isActive ? "circle-check" : "circle-dashed"} size="sm" />
						<div>
							<p class="text-xs font-medium">{sidecar.label}</p>
							<p class="text-[10px] text-muted-foreground">
								{sidecar.runtime} &middot; {sidecar.entrypoint}
							</p>
						</div>
					</div>
					<div class="flex items-center gap-2">
						{#if isActive}
							<Badge variant="outline" class="text-[10px] px-1.5 py-0">Active</Badge>
						{:else}
							<Button
								variant="ghost"
								size="sm"
								class="h-7 px-2 text-xs"
								onclick={() => switchProvider(sidecar.key)}
							>
								Switch
							</Button>
						{/if}
					</div>
				</div>
			{/each}
		</div>

		{#if sidecars.length > 0}
			<div class="mt-3 flex items-start gap-2 rounded bg-muted/50 p-2">
				<Icon name="info" size="sm" />
				<p class="text-[10px] text-muted-foreground">
					Provider changes take effect after restarting the app. The sidecar
					will reconnect using the selected provider's configuration.
				</p>
			</div>
		{/if}
	</CardContent>
</CardRoot>
{/if}

<script lang="ts">
	import { Icon, HStack, Stack, Box, Caption, Text, Button } from "@orqastudio/svelte-components/pure";
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
		<Stack gap={2}>
			{#each sidecars as sidecar (sidecar.key)}
				{@const isActive = sidecar.key === activeKey}
				<Box border rounded="md" paddingX={3} paddingY={2}>
					<HStack justify="between">
						<HStack gap={2}>
							<Icon name={isActive ? "circle-check" : "circle-dashed"} size="sm" />
							<Stack gap={0}>
								<Caption variant="caption-strong">{sidecar.label}</Caption>
								<Caption tone="muted">{sidecar.runtime} · {sidecar.entrypoint}</Caption>
							</Stack>
						</HStack>
						<HStack gap={2}>
							{#if isActive}
								<Badge variant="outline" size="xs">Active</Badge>
							{:else}
								<Button variant="ghost" size="sm" onclick={() => switchProvider(sidecar.key)}>
									Switch
								</Button>
							{/if}
						</HStack>
					</HStack>
				</Box>
			{/each}
		</Stack>

		{#if sidecars.length > 0}
			<Box rounded="md" background="muted" paddingX={2} paddingY={2} marginTop={3}>
				<HStack gap={2} align="start">
					<Icon name="info" size="sm" />
					<Caption tone="muted">
						Provider changes take effect after restarting the app. The sidecar
						will reconnect using the selected provider's configuration.
					</Caption>
				</HStack>
			</Box>
		{/if}
	</CardContent>
</CardRoot>
{/if}

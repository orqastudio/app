<script lang="ts">
	import {
		Icon,
		Heading,
		HStack,
		Stack,
		Caption,
		Text,
		Button,
		Panel,
	} from "@orqastudio/svelte-components/pure";
	import type { PluginManifest } from "@orqastudio/types";

	interface Props {
		manifest: PluginManifest;
		onAccept: () => void;
		onReject: () => void;
		onClose: () => void;
	}

	const { manifest, onAccept, onReject, onClose }: Props = $props();

	const navItems = $derived(manifest.defaultNavigation ?? []);
	const hasNavItems = $derived(navItems.length > 0);

	/**
	 *
	 * @param key
	 */
	function humanizeKey(key: string): string {
		return key.replace(/[-_]/g, " ").replace(/\b\w/g, (c) => c.toUpperCase());
	}
</script>

<div class="bg-background/80 fixed inset-0 z-50 flex items-center justify-center">
	<Panel padding="loose" rounded="lg" border="all" background="card">
		<Stack gap={4}>
			<HStack gap={3}>
				<Panel padding="tight" rounded="lg" background="muted">
					<Icon name="puzzle" size="lg" />
				</Panel>
				<Stack gap={0}>
					<Heading level={3}>Install Plugin</Heading>
					<Text variant="body-muted">{manifest.displayName ?? manifest.name}</Text>
				</Stack>
			</HStack>

			{#if manifest.description}
				<Text variant="body-muted" block>{manifest.description}</Text>
			{/if}

			<Stack gap={3}>
				<HStack gap={1}>
					<Text variant="body-strong">Provides:</Text>
					<Text variant="body-muted">
						{manifest.provides.schemas.length} artifact types,
						{manifest.provides.views.length} views,
						{manifest.provides.relationships.length} relationships
					</Text>
				</HStack>

				{#if hasNavItems}
					<Stack gap={2}>
						<Text variant="body-strong">This plugin wants to add to your navigation:</Text>
						<Panel padding="normal" rounded="md" border="all" background="muted">
							<Stack gap={1}>
								{#each navItems as item (item.key)}
									<HStack gap={2}>
										<Icon name={item.icon} size="sm" />
										<Text>{item.label ?? humanizeKey(item.key)}</Text>
										{#if item.children}
											<Caption tone="muted">({item.children.length} items)</Caption>
										{/if}
									</HStack>
								{/each}
							</Stack>
						</Panel>
					</Stack>
				{/if}
			</Stack>

			<HStack gap={2} justify="end">
				<Button variant="ghost" size="sm" onclick={onClose}>Cancel</Button>
				{#if hasNavItems}
					<Button variant="outline" size="sm" onclick={onReject}>Install Without Navigation</Button>
				{/if}
				<Button variant="default" size="sm" onclick={onAccept}>
					{hasNavItems ? "Accept & Install" : "Install"}
				</Button>
			</HStack>
		</Stack>
	</Panel>
</div>

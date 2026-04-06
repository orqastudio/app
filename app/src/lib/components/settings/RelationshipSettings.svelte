<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import {
		ScrollArea,
		Heading,
		Text,
		HStack,
		Stack,
		Panel,
		Caption,
	} from "@orqastudio/svelte-components/pure";
	import { PLATFORM_RELATIONSHIPS } from "@orqastudio/types";

	const { pluginRegistry } = getStores();

	const pluginRelationships = $derived(pluginRegistry.allRelationships);

	/**
	 * Formats a list of artifact type constraints as a readable string.
	 * @param types - The list of allowed artifact type keys.
	 * @returns A comma-separated string, or "any" if the list is empty.
	 */
	function typeConstraint(types: string[]): string {
		if (types.length === 0) return "any";
		return types.join(", ");
	}
</script>

<Panel padding="loose">
	<Stack gap={6}>
		<Stack gap={1}>
			<Heading level={2}>Relationships</Heading>
			<Text tone="muted">
				Canonical relationships ship with the platform and cannot be removed. Plugins can contribute
				additional relationship types.
			</Text>
		</Stack>

		<!-- Canonical Relationships -->
		<Stack gap={2}>
			<Caption tone="muted">Platform (Canonical)</Caption>
			<ScrollArea maxHeight="lg">
				<Stack gap={1}>
					{#each PLATFORM_RELATIONSHIPS as rel (rel.key)}
						<Panel padding="tight" border="all" rounded="md">
							<HStack gap={3}>
								<Stack flex={1} gap={0}>
									<HStack gap={2}>
										<Text variant="body-strong">{rel.label}</Text>
										<Text tone="muted">/</Text>
										<Text variant="body-strong">{rel.inverseLabel}</Text>
									</HStack>
									<HStack gap={2}>
										<Caption tone="muted">{rel.key} / {rel.inverse}</Caption>
										<Caption tone="muted">|</Caption>
										<Caption tone="muted"
											>{typeConstraint(rel.from as unknown as string[])} → {typeConstraint(
												rel.to as unknown as string[],
											)}</Caption
										>
									</HStack>
								</Stack>
								<Caption>Platform</Caption>
							</HStack>
						</Panel>
					{/each}
				</Stack>
			</ScrollArea>
		</Stack>

		<!-- Plugin Relationships -->
		{#if pluginRelationships.length > 0}
			<Stack gap={2}>
				<Caption tone="muted">Plugin-Contributed</Caption>
				<Stack gap={1}>
					{#each pluginRelationships as rel (rel.key)}
						<Panel padding="tight" border="all" rounded="md">
							<HStack gap={3}>
								<Stack flex={1} gap={0}>
									<HStack gap={2}>
										<Text variant="body-strong">{rel.label}</Text>
										<Text tone="muted">/</Text>
										<Text variant="body-strong">{rel.inverseLabel}</Text>
									</HStack>
									<HStack gap={2}>
										<Caption tone="muted">{rel.key} / {rel.inverse}</Caption>
										<Caption tone="muted">|</Caption>
										<Caption tone="muted"
											>{typeConstraint(rel.from)} → {typeConstraint(rel.to)}</Caption
										>
									</HStack>
									{#if rel.description}
										<Caption tone="muted">{rel.description}</Caption>
									{/if}
								</Stack>
								<Caption>Plugin</Caption>
							</HStack>
						</Panel>
					{/each}
				</Stack>
			</Stack>
		{/if}
	</Stack>
</Panel>

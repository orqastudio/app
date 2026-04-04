<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import { ScrollArea, Heading, Text } from "@orqastudio/svelte-components/pure";
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

<div class="space-y-6 p-6">
	<div class="flex flex-col gap-1">
		<Heading level={2}>Relationships</Heading>
		<Text tone="muted">
			Canonical relationships ship with the platform and cannot be removed.
			Plugins can contribute additional relationship types.
		</Text>
	</div>

	<!-- Canonical Relationships -->
	<div class="flex flex-col gap-2">
		<Text variant="caption" tone="muted">Platform (Canonical)</Text>
		<ScrollArea maxHeight="lg">
			<div class="flex flex-col gap-1">
				{#each PLATFORM_RELATIONSHIPS as rel (rel.key)}
					<div class="flex items-center gap-3 rounded-md border border-border bg-card px-3 py-2">
						<div class="min-w-0 flex-1">
							<div class="flex items-center gap-2 text-sm">
								<span class="font-medium">{rel.label}</span>
								<Text tone="muted">/</Text>
								<span class="font-medium">{rel.inverseLabel}</span>
							</div>
							<div class="flex items-center gap-2">
								<Text variant="caption" tone="muted">{rel.key} / {rel.inverse}</Text>
								<Text variant="caption" tone="muted">|</Text>
								<Text variant="caption" tone="muted">{typeConstraint(rel.from as unknown as string[])} → {typeConstraint(rel.to as unknown as string[])}</Text>
							</div>
						</div>
						<span class="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">Platform</span>
					</div>
				{/each}
			</div>
		</ScrollArea>
	</div>

	<!-- Plugin Relationships -->
	{#if pluginRelationships.length > 0}
		<div class="flex flex-col gap-2">
			<Text variant="caption" tone="muted">Plugin-Contributed</Text>
			<div class="flex flex-col gap-1">
				{#each pluginRelationships as rel (rel.key)}
					<div class="flex items-center gap-3 rounded-md border border-border bg-card px-3 py-2">
						<div class="min-w-0 flex-1">
							<div class="flex items-center gap-2 text-sm">
								<span class="font-medium">{rel.label}</span>
								<Text tone="muted">/</Text>
								<span class="font-medium">{rel.inverseLabel}</span>
							</div>
							<div class="flex items-center gap-2">
								<Text variant="caption" tone="muted">{rel.key} / {rel.inverse}</Text>
								<Text variant="caption" tone="muted">|</Text>
								<Text variant="caption" tone="muted">{typeConstraint(rel.from)} → {typeConstraint(rel.to)}</Text>
							</div>
							{#if rel.description}
								<Text variant="caption" tone="muted">{rel.description}</Text>
							{/if}
						</div>
						<span class="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">Plugin</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

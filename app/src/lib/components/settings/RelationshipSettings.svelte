<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import { ScrollArea } from "@orqastudio/svelte-components/pure";
	import { PLATFORM_RELATIONSHIPS } from "@orqastudio/types";

	const { pluginRegistry } = getStores();

	const pluginRelationships = $derived(pluginRegistry.allRelationships);

	function typeConstraint(types: string[]): string {
		if (types.length === 0) return "any";
		return types.join(", ");
	}
</script>

<div class="space-y-6 p-6">
	<div>
		<h2 class="text-lg font-semibold">Relationships</h2>
		<p class="text-sm text-muted-foreground">
			Canonical relationships ship with the platform and cannot be removed.
			Plugins can contribute additional relationship types.
		</p>
	</div>

	<!-- Canonical Relationships -->
	<div>
		<h3 class="mb-2 text-sm font-medium text-muted-foreground">Platform (Canonical)</h3>
		<ScrollArea class="max-h-[400px]">
			<div class="space-y-1">
				{#each PLATFORM_RELATIONSHIPS as rel (rel.key)}
					<div class="flex items-center gap-3 rounded-md border border-border bg-card px-3 py-2">
						<div class="min-w-0 flex-1">
							<div class="flex items-center gap-2 text-sm">
								<span class="font-medium">{rel.label}</span>
								<span class="text-muted-foreground">/</span>
								<span class="font-medium">{rel.inverseLabel}</span>
							</div>
							<div class="flex items-center gap-2 text-xs text-muted-foreground">
								<span>{rel.key} / {rel.inverse}</span>
								<span class="text-muted-foreground/50">|</span>
								<span>{typeConstraint(rel.from as unknown as string[])} → {typeConstraint(rel.to as unknown as string[])}</span>
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
		<div>
			<h3 class="mb-2 text-sm font-medium text-muted-foreground">Plugin-Contributed</h3>
			<div class="space-y-1">
				{#each pluginRelationships as rel (rel.key)}
					<div class="flex items-center gap-3 rounded-md border border-border bg-card px-3 py-2">
						<div class="min-w-0 flex-1">
							<div class="flex items-center gap-2 text-sm">
								<span class="font-medium">{rel.label}</span>
								<span class="text-muted-foreground">/</span>
								<span class="font-medium">{rel.inverseLabel}</span>
							</div>
							<div class="flex items-center gap-2 text-xs text-muted-foreground">
								<span>{rel.key} / {rel.inverse}</span>
								<span class="text-muted-foreground/50">|</span>
								<span>{typeConstraint(rel.from)} → {typeConstraint(rel.to)}</span>
							</div>
							{#if rel.description}
								<div class="text-xs text-muted-foreground">{rel.description}</div>
							{/if}
						</div>
						<span class="rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">Plugin</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

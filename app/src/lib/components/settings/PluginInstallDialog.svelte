<script lang="ts">
	import { Icon, Heading } from "@orqastudio/svelte-components/pure";
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

	function humanizeKey(key: string): string {
		return key
			.replace(/[-_]/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-background/80">
	<div class="w-full max-w-md rounded-lg border border-border bg-card p-6 shadow-lg">
		<div class="flex items-center gap-3">
			<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
				<Icon name="puzzle" size="lg" />
			</div>
			<div class="flex flex-col gap-0">
				<Heading level={3}>Install Plugin</Heading>
				<span class="text-sm text-muted-foreground">{manifest.displayName ?? manifest.name}</span>
			</div>
		</div>

		{#if manifest.description}
			<p class="mt-3 text-sm text-muted-foreground">{manifest.description}</p>
		{/if}

		<div class="mt-4 flex flex-col gap-3">
			<div class="flex items-center gap-1">
				<span class="text-sm font-medium">Provides:</span>
				<span class="text-sm text-muted-foreground">
					{manifest.provides.schemas.length} artifact types,
					{manifest.provides.views.length} views,
					{manifest.provides.relationships.length} relationships
				</span>
			</div>

			{#if hasNavItems}
				<div class="flex flex-col gap-2">
					<span class="text-sm font-medium">This plugin wants to add to your navigation:</span>
					<div class="flex flex-col gap-1 rounded-md border border-border bg-muted/30 p-3">
						{#each navItems as item (item.key)}
							<div class="flex items-center gap-2">
								<Icon name={item.icon} size="sm" />
								<span class="text-sm">{item.label ?? humanizeKey(item.key)}</span>
								{#if item.children}
									<span class="text-xs text-muted-foreground">({item.children.length} items)</span>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>

		<div class="mt-6 flex items-center justify-end gap-2">
			<button class="flex items-center rounded px-3 py-1.5 text-sm hover:bg-accent" onclick={onClose}>Cancel</button>
			{#if hasNavItems}
				<button class="flex items-center rounded border border-border px-3 py-1.5 text-sm hover:bg-accent" onclick={onReject}>Install Without Navigation</button>
			{/if}
			<button class="flex items-center rounded bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90" onclick={onAccept}>
				{hasNavItems ? "Accept & Install" : "Install"}
			</button>
		</div>
	</div>
</div>

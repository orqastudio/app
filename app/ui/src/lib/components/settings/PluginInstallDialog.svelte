<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import type { PluginManifest, DefaultNavItem } from "@orqastudio/types";

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

	function flattenNavItems(items: DefaultNavItem[]): DefaultNavItem[] {
		const flat: DefaultNavItem[] = [];
		for (const item of items) {
			flat.push(item);
			if (item.children) {
				flat.push(...flattenNavItems(item.children));
			}
		}
		return flat;
	}
</script>

<div class="fixed inset-0 z-50 flex items-center justify-center bg-background/80">
	<div class="w-full max-w-md rounded-lg border border-border bg-card p-6 shadow-lg">
		<div class="flex items-center gap-3">
			<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
				<Icon name="puzzle" size="lg" />
			</div>
			<div>
				<h2 class="text-lg font-semibold">Install Plugin</h2>
				<p class="text-sm text-muted-foreground">{manifest.displayName ?? manifest.name}</p>
			</div>
		</div>

		{#if manifest.description}
			<p class="mt-3 text-sm text-muted-foreground">{manifest.description}</p>
		{/if}

		<div class="mt-4 space-y-3">
			<div class="text-sm">
				<span class="font-medium">Provides:</span>
				<span class="text-muted-foreground">
					{manifest.provides.schemas.length} artifact types,
					{manifest.provides.views.length} views,
					{manifest.provides.relationships.length} relationships
				</span>
			</div>

			{#if hasNavItems}
				<div>
					<p class="text-sm font-medium">This plugin wants to add to your navigation:</p>
					<div class="mt-2 space-y-1 rounded-md border border-border bg-muted/30 p-3">
						{#each navItems as item (item.key)}
							<div class="flex items-center gap-2 text-sm">
								<Icon name={item.icon} size="sm" />
								<span>{item.label ?? humanizeKey(item.key)}</span>
								{#if item.children}
									<span class="text-xs text-muted-foreground">
										({item.children.length} items)
									</span>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>

		<div class="mt-6 flex justify-end gap-2">
			<button
				class="rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:bg-muted"
				onclick={onClose}
			>
				Cancel
			</button>
			{#if hasNavItems}
				<button
					class="rounded-md border border-border px-3 py-1.5 text-sm hover:bg-muted"
					onclick={onReject}
				>
					Install Without Navigation
				</button>
			{/if}
			<button
				class="rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90"
				onclick={onAccept}
			>
				{hasNavItems ? "Accept & Install" : "Install"}
			</button>
		</div>
	</div>
</div>

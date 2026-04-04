<script lang="ts" module>
	import type { Snippet } from "svelte";

	export type TabDef = {
		value: string;
		label: string;
		content: Snippet;
		disabled?: boolean;
	};

	type TabsWithDefs = {
		value?: string;
		tabs: TabDef[];
		children?: never;
	};

	type TabsWithContent = {
		value?: string;
		children: Snippet;
		tabs?: never;
	};

	export type TabsProps = TabsWithDefs | TabsWithContent;
</script>

<script lang="ts">
	import TabsRoot from "./tabs.svelte";
	import TabsList from "./tabs-list.svelte";
	import TabsTrigger from "./tabs-trigger.svelte";
	import TabsContent from "./tabs-content.svelte";

	let {
		value = $bindable(""),
		tabs,
		children,
	}: TabsProps = $props();

	// $derived cannot be used here because `value` is $bindable — a parent may update it at any time.
	// This $effect initialises the default selection once when tabs load and no value is provided.
	// It is a true side effect (mutating a bindable prop), not derived computation.
	$effect(() => {
		if (!value && tabs && tabs.length > 0) {
			value = tabs[0].value;
		}
	});
</script>

<TabsRoot bind:value>
	{#if children}
		{@render children()}
	{:else if tabs}
		<TabsList>
			{#each tabs as tab (tab.value)}
				<TabsTrigger value={tab.value} disabled={tab.disabled}>{tab.label}</TabsTrigger>
			{/each}
		</TabsList>
		{#each tabs as tab (tab.value)}
			<TabsContent value={tab.value}>
				{@render tab.content()}
			</TabsContent>
		{/each}
	{/if}
</TabsRoot>

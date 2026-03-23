<script lang="ts" module>
	import type { Snippet } from "svelte";

	export type TabDef = {
		value: string;
		label: string;
		content: Snippet;
		disabled?: boolean;
	};

	type TabsBaseProps = {
		value?: string;
		class?: string;
	};

	type TabsWithDefs = TabsBaseProps & {
		tabs: TabDef[];
		children?: never;
	};

	type TabsWithContent = TabsBaseProps & {
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
		class: className,
		tabs,
		children,
	}: TabsProps = $props();

	// Default to first tab if no value provided
	$effect(() => {
		if (!value && tabs && tabs.length > 0) {
			value = tabs[0].value;
		}
	});
</script>

<TabsRoot bind:value class={className}>
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

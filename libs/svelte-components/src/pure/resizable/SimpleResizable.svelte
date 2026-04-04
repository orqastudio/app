<script lang="ts" module>
	import type { Snippet } from "svelte";

	type ResizableTwoPane = {
		direction?: "horizontal" | "vertical";
		main: Snippet;
		side: Snippet;
		mainSize?: number;
		sideSize?: number;
		mainMinSize?: number;
		sideMinSize?: number;
		children?: never;
	};

	type ResizableCustom = {
		direction?: "horizontal" | "vertical";
		children: Snippet;
		main?: never;
		side?: never;
		mainSize?: never;
		sideSize?: never;
		mainMinSize?: never;
		sideMinSize?: never;
	};

	export type ResizableProps = ResizableTwoPane | ResizableCustom;
</script>

<script lang="ts">
	import PaneGroup from "./resizable-pane-group.svelte";
	import Handle from "./resizable-handle.svelte";
	import { Pane } from "paneforge";

	let {
		direction = "horizontal",
		main,
		side,
		mainSize = 70,
		sideSize = 30,
		mainMinSize = 30,
		sideMinSize = 20,
		children,
	}: ResizableProps = $props();
</script>

<PaneGroup {direction}>
	{#if children}
		{@render children()}
	{:else if main && side}
		<Pane defaultSize={mainSize} minSize={mainMinSize}>
			{@render main()}
		</Pane>
		<Handle />
		<Pane defaultSize={sideSize} minSize={sideMinSize}>
			{@render side()}
		</Pane>
	{/if}
</PaneGroup>

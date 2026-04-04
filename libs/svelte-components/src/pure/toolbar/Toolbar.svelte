<script lang="ts">
	import type { Snippet } from "svelte";
	import { HStack, Spacer } from "../layout/index.js";

	let {
		left,
		center,
		right,
	}: {
		left?: Snippet;
		center?: Snippet;
		right?: Snippet;
	} = $props();
</script>

<div class="flex h-10 items-center border-b border-border px-3">
	<HStack gap={2}>
		{#if left}
			{@render left()}
		{/if}
	</HStack>
	<Spacer />
	<HStack gap={2} justify="center">
		{#if center}
			{@render center()}
		{/if}
	</HStack>
	<Spacer />
	<HStack gap={2}>
		{#if right}
			{@render right()}
		{/if}
	</HStack>
</div>

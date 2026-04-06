<!-- Message bubble wrapper with role-based styling. Content is passed as a child snippet. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { HStack } from "../layout/index.js";

	let {
		role,
		children,
	}: {
		/** The speaker role — controls alignment and color treatment. */
		role: "user" | "assistant" | "system";
		children?: Snippet;
	} = $props();
</script>

{#if role === "user"}
	<HStack justify="end">
		<div
			class="bg-primary text-primary-foreground max-w-[80%] rounded-2xl rounded-tr-sm px-4 py-2.5"
		>
			{#if children}{@render children()}{/if}
		</div>
	</HStack>
{:else if role === "assistant"}
	<HStack justify="start">
		<div class="border-border bg-muted/50 max-w-[85%] rounded-2xl rounded-tl-sm border px-4 py-2.5">
			{#if children}{@render children()}{/if}
		</div>
	</HStack>
{:else}
	<HStack justify="center">
		<div class="bg-muted/30 max-w-[90%] rounded-lg px-4 py-2">
			{#if children}{@render children()}{/if}
		</div>
	</HStack>
{/if}

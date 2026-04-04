<!-- Message bubble wrapper with role-based styling. Content is passed as a child snippet. -->
<script lang="ts">
	import type { Snippet } from "svelte";

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
	<div class="flex justify-end">
		<div class="max-w-[80%] rounded-2xl rounded-tr-sm bg-primary px-4 py-2.5 text-primary-foreground">
			{#if children}{@render children()}{/if}
		</div>
	</div>
{:else if role === "assistant"}
	<div class="flex justify-start">
		<div class="max-w-[85%] rounded-2xl rounded-tl-sm border border-border bg-muted/50 px-4 py-2.5">
			{#if children}{@render children()}{/if}
		</div>
	</div>
{:else}
	<div class="flex justify-center">
		<div class="max-w-[90%] rounded-lg bg-muted/30 px-4 py-2">
			{#if children}{@render children()}{/if}
		</div>
	</div>
{/if}

<!-- Message bubble wrapper with role-based styling. Content is passed as a child snippet.
     The system role supports a tone prop to switch between default (muted) and destructive styles. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { HStack } from "../layout/index.js";

	let {
		role,
		tone = "default",
		children,
	}: {
		/** The speaker role — controls alignment and color treatment. */
		role: "user" | "assistant" | "system";
		/** Only applies to role="system". "destructive" renders error styling; "default" renders muted. */
		tone?: "default" | "destructive";
		children?: Snippet;
	} = $props();

	/** Resolves the background class for system bubbles based on tone. */
	const systemBgClass = $derived(tone === "destructive" ? "bg-destructive/10" : "bg-muted/30");
</script>

{#if role === "user"}
	<HStack justify="end">
		<div class="bg-primary text-primary-foreground max-w-[80%] rounded-2xl rounded-tr-sm px-4 py-3">
			{#if children}{@render children()}{/if}
		</div>
	</HStack>
{:else if role === "assistant"}
	<HStack justify="start">
		<div class="border-border bg-muted/50 max-w-[85%] rounded-2xl rounded-tl-sm border px-4 py-3">
			{#if children}{@render children()}{/if}
		</div>
	</HStack>
{:else}
	<HStack justify="center">
		<div class="max-w-[90%] rounded-lg px-4 py-2 {systemBgClass}">
			{#if children}{@render children()}{/if}
		</div>
	</HStack>
{/if}

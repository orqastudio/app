<script lang="ts">
	import type { Snippet } from "svelte";
	import { open } from "@tauri-apps/plugin-shell";
	import ArtifactLink from "$lib/components/artifact/ArtifactLink.svelte";
	import { getStores } from "@orqastudio/sdk";

	let {
		href = "",
		title = undefined,
		children,
	}: {
		href?: string;
		title?: string;
		children?: Snippet;
	} = $props();

	const { pluginRegistry } = getStores();

	/**
	 * Build an artifact ID regex dynamically from registered plugin schemas.
	 * Matches IDs like EPIC-001, TASK-023, etc. using prefixes declared by
	 * plugins rather than a hardcoded list.
	 */
	const artifactIdRe = $derived.by(() => {
		const prefixes = pluginRegistry.allSchemas.map((s) => s.idPrefix).filter(Boolean);
		if (prefixes.length === 0) {
			// No schemas registered yet — fall back to a broad uppercase-prefix pattern.
			return /^[A-Z]+-\d+$/;
		}
		const escaped = prefixes.map((p) => p.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
		return new RegExp(`^(${escaped.join("|")})-\\d+$`);
	});

	const isArtifactLink = $derived(artifactIdRe.test(href));
	const isExternal = $derived(href.startsWith("http://") || href.startsWith("https://"));

	function handleExternalClick(e: MouseEvent) {
		e.preventDefault();
		open(href);
	}
</script>

{#if isArtifactLink}
	<ArtifactLink id={href} />
{:else if isExternal}
	<a {href} {title} class="cursor-pointer" onclick={handleExternalClick}>
		{@render children?.()}
	</a>
{:else}
	<a {href} {title}>
		{@render children?.()}
	</a>
{/if}

<script lang="ts">
	import type { Snippet } from "svelte";
	import { open } from "@tauri-apps/plugin-shell";
	import { ArtifactLink } from "@orqastudio/svelte-components/connected";
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

	/**
	 * Open external href in the OS browser via Tauri shell opener.
	 * @param e - The click event to prevent default navigation on.
	 */
	function handleExternalClick(e: MouseEvent) {
		e.preventDefault();
		open(href);
	}
</script>

{#if isArtifactLink}
	<ArtifactLink id={href} />
{:else if isExternal}
	<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -- External links use Tauri shell opener, not SvelteKit router -->
	<a {href} {title} onclick={handleExternalClick}>
		{@render children?.()}
	</a>
{:else}
	<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -- Relative links in markdown content are rendered as-is in the Tauri webview -->
	<a {href} {title}>
		{@render children?.()}
	</a>
{/if}

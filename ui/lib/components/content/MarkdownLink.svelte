<script lang="ts">
	import type { Snippet } from "svelte";
	import { open } from "@tauri-apps/plugin-shell";
	import ArtifactLink from "$lib/components/artifact/ArtifactLink.svelte";

	let {
		href = "",
		title = undefined,
		children,
	}: {
		href?: string;
		title?: string;
		children?: Snippet;
	} = $props();

	const ARTIFACT_ID_RE = /^(EPIC|TASK|AD|MS|IDEA|IMPL|RES|PILLAR|RULE|DOC)-\d{3,}$/;

	const isArtifactLink = $derived(ARTIFACT_ID_RE.test(href));
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

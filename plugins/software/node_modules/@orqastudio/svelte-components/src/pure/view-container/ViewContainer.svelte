<script lang="ts">
	import type { Snippet } from "svelte";
	import { LoadingSpinner } from "../loading-spinner/index.js";
	import { ErrorDisplay } from "../error-display/index.js";
	import { EmptyState } from "../empty-state/index.js";
	import type { Component } from "svelte";

	let {
		loading = false,
		error,
		empty = false,
		emptyTitle = "Nothing here yet",
		emptyDescription,
		emptyIcon,
		emptyAction,
		onRetry,
		children,
	}: {
		loading?: boolean;
		error?: string;
		empty?: boolean;
		emptyTitle?: string;
		emptyDescription?: string;
		emptyIcon?: Component;
		emptyAction?: { label: string; onclick: () => void };
		onRetry?: () => void;
		children: Snippet;
	} = $props();
</script>

{#if loading}
	<LoadingSpinner />
{:else if error}
	<ErrorDisplay message={error} {onRetry} />
{:else if empty}
	<EmptyState icon={emptyIcon} title={emptyTitle} description={emptyDescription} action={emptyAction} />
{:else}
	{@render children()}
{/if}

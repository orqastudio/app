<!-- SearchResultItem — single row in a search results list.
     Renders a full-width button with optional status icon, project badge, ID badge,
     title, and artifact type. The `active` prop applies the accent highlight used
     for keyboard-selected items. -->
<script lang="ts">
	import { Icon } from "../icon/index.js";
	import { Box } from "../layout/index.js";
	import { Text } from "../typography/index.js";
	import { cn } from "../../utils/cn.js";

	let {
		/** Icon name to show on the left (e.g. a status icon or "file-text"). */
		iconName = "file-text",
		/** Project identifier badge text; omit to hide the badge. */
		project,
		/** Artifact ID shown as a monospace badge. */
		id,
		/** Primary title text, truncated. */
		title,
		/** Artifact type shown at the trailing end. */
		artifactType,
		/** Whether this row is currently keyboard-selected. */
		active = false,
		onclick,
		onmouseenter,
	}: {
		iconName?: string;
		project?: string;
		id: string;
		title: string;
		artifactType?: string;
		active?: boolean;
		onclick?: () => void;
		onmouseenter?: () => void;
	} = $props();
</script>

<button
	class={cn(
		"flex w-full items-center justify-start gap-2 rounded-md px-2 py-1.5 text-sm",
		active ? "bg-accent text-accent-foreground" : "hover:bg-accent/50",
	)}
	{onclick}
	{onmouseenter}
>
	<Icon name={iconName} size="sm" />

	{#if project}
		<!-- Project badge: primary-tinted; inline raw span is valid inside library code. -->
		<span class="bg-primary/10 text-primary shrink-0 rounded px-1 py-0.5 text-[9px] font-medium"
			>{project}</span
		>
	{/if}

	<!-- ID badge: monospace secondary badge; inline raw span is valid inside library code. -->
	<span class="bg-muted text-muted-foreground shrink-0 rounded px-1 py-0.5 font-mono text-[11px]"
		>{id}</span
	>

	<Box flex={1} minWidth={0} truncate>
		{title}
	</Box>

	{#if artifactType}
		<Text variant="caption" truncate>{artifactType}</Text>
	{/if}
</button>

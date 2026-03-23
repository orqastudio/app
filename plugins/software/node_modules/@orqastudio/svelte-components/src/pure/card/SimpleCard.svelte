<script lang="ts" module>
	import type { Snippet } from "svelte";

	type CardBaseProps = {
		class?: string;
		children: Snippet;
		footer?: Snippet;
	};

	type CardWithTitle = CardBaseProps & {
		title: string;
		description?: string;
		action?: Snippet;
		header?: never;
	};

	type CardWithHeader = CardBaseProps & {
		header: Snippet;
		title?: never;
		description?: never;
		action?: never;
	};

	type CardContentOnly = CardBaseProps & {
		title?: never;
		description?: never;
		action?: never;
		header?: never;
	};

	export type CardProps = CardWithTitle | CardWithHeader | CardContentOnly;
</script>

<script lang="ts">
	import Root from "./card.svelte";
	import Header from "./card-header.svelte";
	import Title from "./card-title.svelte";
	import Description from "./card-description.svelte";
	import Content from "./card-content.svelte";
	import Action from "./card-action.svelte";

	let {
		title,
		description,
		class: className,
		header,
		footer,
		action,
		children,
	}: CardProps = $props();
</script>

<Root class={className}>
	{#if header}
		{@render header()}
	{:else if title}
		<Header>
			<Title>{title}</Title>
			{#if description}<Description>{description}</Description>{/if}
			{#if action}<Action>{@render action()}</Action>{/if}
		</Header>
	{/if}
	<Content>
		{@render children()}
	</Content>
	{#if footer}
		{@render footer()}
	{/if}
</Root>

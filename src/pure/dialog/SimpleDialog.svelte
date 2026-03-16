<script lang="ts" module>
	import type { Snippet } from "svelte";

	type DialogBaseProps = {
		open: boolean;
		trigger?: Snippet<[{ props: Record<string, unknown> }]>;
		children: Snippet;
		footer?: Snippet;
	};

	type DialogWithTitle = DialogBaseProps & {
		title: string;
		description?: string;
		header?: never;
	};

	type DialogWithHeader = DialogBaseProps & {
		header: Snippet;
		title?: never;
		description?: never;
	};

	export type DialogProps = DialogWithTitle | DialogWithHeader;
</script>

<script lang="ts">
	import { Dialog as DialogPrimitive } from "bits-ui";
	import DialogContent from "./dialog-content.svelte";
	import DialogHeader from "./dialog-header.svelte";
	import DialogTitle from "./dialog-title.svelte";
	import DialogDescription from "./dialog-description.svelte";
	import DialogFooter from "./dialog-footer.svelte";

	let {
		open = $bindable(false),
		trigger,
		title,
		description,
		header,
		footer,
		children,
	}: DialogProps = $props();
</script>

<DialogPrimitive.Root bind:open>
	{#if trigger}
		<DialogPrimitive.Trigger>
			{#snippet child({ props })}
				{@render trigger({ props })}
			{/snippet}
		</DialogPrimitive.Trigger>
	{/if}
	<DialogContent>
		{#if header}
			{@render header()}
		{:else if title}
			<DialogHeader>
				<DialogTitle>{title}</DialogTitle>
				{#if description}<DialogDescription>{description}</DialogDescription>{/if}
			</DialogHeader>
		{/if}
		{@render children()}
		{#if footer}
			<DialogFooter>
				{@render footer()}
			</DialogFooter>
		{/if}
	</DialogContent>
</DialogPrimitive.Root>

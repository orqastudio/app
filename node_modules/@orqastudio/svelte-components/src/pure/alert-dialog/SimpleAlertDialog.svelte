<script lang="ts" module>
	import type { Snippet } from "svelte";

	type AlertDialogBaseProps = {
		open: boolean;
		trigger?: Snippet<[{ props: Record<string, unknown> }]>;
	};

	type AlertDialogWithTitle = AlertDialogBaseProps & {
		title: string;
		description?: string;
		confirmLabel?: string;
		cancelLabel?: string;
		onConfirm: () => void;
		onCancel?: () => void;
		header?: never;
		footer?: never;
		children?: never;
	};

	type AlertDialogWithContent = AlertDialogBaseProps & {
		header?: Snippet;
		footer?: Snippet;
		children: Snippet;
		title?: never;
		description?: never;
		confirmLabel?: never;
		cancelLabel?: never;
		onConfirm?: never;
		onCancel?: never;
	};

	export type AlertDialogProps = AlertDialogWithTitle | AlertDialogWithContent;
</script>

<script lang="ts">
	import { AlertDialog as AlertDialogPrimitive } from "bits-ui";
	import AlertDialogContent from "./alert-dialog-content.svelte";
	import AlertDialogHeader from "./alert-dialog-header.svelte";
	import AlertDialogTitle from "./alert-dialog-title.svelte";
	import AlertDialogDescription from "./alert-dialog-description.svelte";
	import AlertDialogFooter from "./alert-dialog-footer.svelte";
	import AlertDialogCancel from "./alert-dialog-cancel.svelte";
	import AlertDialogAction from "./alert-dialog-action.svelte";

	let {
		open = $bindable(false),
		trigger,
		title,
		description,
		confirmLabel = "Confirm",
		cancelLabel = "Cancel",
		onConfirm,
		onCancel,
		header,
		footer,
		children,
	}: AlertDialogProps = $props();
</script>

<AlertDialogPrimitive.Root bind:open>
	{#if trigger}
		<AlertDialogPrimitive.Trigger>
			{#snippet child({ props })}
				{@render trigger({ props })}
			{/snippet}
		</AlertDialogPrimitive.Trigger>
	{/if}
	<AlertDialogContent>
		{#if header}
			{@render header()}
		{:else if title}
			<AlertDialogHeader>
				<AlertDialogTitle>{title}</AlertDialogTitle>
				{#if description}<AlertDialogDescription>{description}</AlertDialogDescription>{/if}
			</AlertDialogHeader>
		{/if}
		{#if children}
			{@render children()}
		{/if}
		{#if footer}
			{@render footer()}
		{:else if onConfirm}
			<AlertDialogFooter>
				<AlertDialogCancel onclick={() => onCancel?.()}>{cancelLabel}</AlertDialogCancel>
				<AlertDialogAction onclick={() => { onConfirm(); open = false; }}>{confirmLabel}</AlertDialogAction>
			</AlertDialogFooter>
		{/if}
	</AlertDialogContent>
</AlertDialogPrimitive.Root>

<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
	import { Button } from "$lib/components/ui/button";

	interface Props {
		open: boolean;
		pendingPath: string | null;
		onConfirm: () => void;
		onCancel: () => void;
	}

	const { open, pendingPath, onConfirm, onCancel }: Props = $props();
</script>

<Dialog.Root
	{open}
	onOpenChange={(isOpen) => { if (!isOpen) onCancel(); }}
>
	<Dialog.Content class="max-w-sm">
		<Dialog.Header>
			<Dialog.Title>Not an Orqa Project</Dialog.Title>
			<Dialog.Description>
				This folder doesn't have an Orqa configuration. Would you like to initialize it as a new Orqa project?
			</Dialog.Description>
		</Dialog.Header>
		{#if pendingPath}
			<p class="rounded bg-muted px-3 py-2 text-xs font-mono text-muted-foreground truncate">
				{pendingPath}
			</p>
		{/if}
		<Dialog.Footer>
			<Button variant="outline" onclick={onCancel}>Cancel</Button>
			<Button onclick={onConfirm}>Initialize Project</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

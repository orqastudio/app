<script lang="ts">
	import { DialogRoot, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";

	interface Props {
		open: boolean;
		pendingPath: string | null;
		onConfirm: () => void;
		onCancel: () => void;
	}

	const { open, pendingPath, onConfirm, onCancel }: Props = $props();
</script>

<DialogRoot
	{open}
	onOpenChange={(isOpen) => { if (!isOpen) onCancel(); }}
>
	<DialogContent>
		<DialogHeader>
			<DialogTitle>Not an Orqa Project</DialogTitle>
			<DialogDescription>
				This folder doesn't have an Orqa configuration. Would you like to initialize it as a new Orqa project?
			</DialogDescription>
		</DialogHeader>
		{#if pendingPath}
			<p class="rounded bg-muted px-3 py-2 text-xs font-mono text-muted-foreground truncate">
				{pendingPath}
			</p>
		{/if}
		<DialogFooter>
			<Button variant="outline" onclick={onCancel}>Cancel</Button>
			<Button onclick={onConfirm}>Initialize Project</Button>
		</DialogFooter>
	</DialogContent>
</DialogRoot>

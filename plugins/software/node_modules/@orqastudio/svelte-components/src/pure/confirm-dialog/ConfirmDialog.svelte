<script lang="ts">
	import {
		AlertDialogRoot,
		AlertDialogContent,
		AlertDialogHeader,
		AlertDialogTitle,
		AlertDialogDescription,
		AlertDialogFooter,
		AlertDialogCancel,
	} from "../alert-dialog/index.js";
	import { Button } from "../button/index.js";

	let {
		open = $bindable(false),
		title = "Delete permanently?",
		description = "This action cannot be undone.",
		confirmLabel = "Delete",
		confirmVariant = "destructive",
		onConfirm,
	}: {
		open: boolean;
		title?: string;
		description?: string;
		confirmLabel?: string;
		confirmVariant?: "destructive" | "default";
		onConfirm: () => void;
	} = $props();

	function handleConfirm() {
		onConfirm();
		open = false;
	}
</script>

<AlertDialogRoot bind:open>
	<AlertDialogContent>
		<AlertDialogHeader>
			<AlertDialogTitle>{title}</AlertDialogTitle>
			<AlertDialogDescription>{description}</AlertDialogDescription>
		</AlertDialogHeader>
		<AlertDialogFooter>
			<AlertDialogCancel>Cancel</AlertDialogCancel>
			<Button variant={confirmVariant} onclick={handleConfirm}>{confirmLabel}</Button>
		</AlertDialogFooter>
	</AlertDialogContent>
</AlertDialogRoot>

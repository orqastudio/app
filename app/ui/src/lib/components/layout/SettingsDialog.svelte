<script lang="ts">
	import { Icon, DialogRoot, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@orqastudio/svelte-components/pure";
	import SettingsView from "$lib/components/settings/SettingsView.svelte";
	import SettingsCategoryNav from "$lib/components/navigation/SettingsCategoryNav.svelte";

	interface Props {
		open: boolean;
		onClose: () => void;
	}

	const { open, onClose }: Props = $props();

	/** Local section state so the dialog doesn't interfere with the inline project settings view. */
	let dialogSection = $state("provider");
</script>

<DialogRoot
	{open}
	onOpenChange={(isOpen) => { if (!isOpen) onClose(); }}
>
	<DialogContent class="flex h-[85vh] w-[90vw] max-w-5xl flex-col gap-0 overflow-hidden p-0 sm:max-w-5xl">
		<div class="flex items-center justify-between border-b border-border px-6 py-4">
			<DialogTitle>Settings</DialogTitle>
			<DialogDescription class="sr-only">Application settings</DialogDescription>
			<button
				class="rounded-sm p-1 text-muted-foreground opacity-70 transition-opacity hover:opacity-100"
				onclick={onClose}
			>
				<Icon name="x" size="md" />
			</button>
		</div>
		<div class="flex flex-1 overflow-hidden">
			<div class="w-56 shrink-0 border-r border-border">
				<SettingsCategoryNav mode="app" activeSection={dialogSection} onSectionChange={(s) => { dialogSection = s; }} />
			</div>
			<div class="flex-1 overflow-hidden">
				<SettingsView activeSection={dialogSection} />
			</div>
		</div>
	</DialogContent>
</DialogRoot>

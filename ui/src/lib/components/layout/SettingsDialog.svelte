<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
	import XIcon from "@lucide/svelte/icons/x";
	import SettingsView from "$lib/components/settings/SettingsView.svelte";
	import SettingsCategoryNav from "$lib/components/navigation/SettingsCategoryNav.svelte";

	interface Props {
		open: boolean;
		onClose: () => void;
	}

	const { open, onClose }: Props = $props();
</script>

<Dialog.Root
	{open}
	onOpenChange={(isOpen) => { if (!isOpen) onClose(); }}
>
	<Dialog.Content class="flex h-[85vh] w-[90vw] max-w-5xl flex-col gap-0 overflow-hidden p-0 sm:max-w-5xl">
		<div class="flex items-center justify-between border-b border-border px-6 py-4">
			<Dialog.Title>Settings</Dialog.Title>
			<Dialog.Description class="sr-only">Application settings</Dialog.Description>
			<button
				class="rounded-sm p-1 text-muted-foreground opacity-70 transition-opacity hover:opacity-100"
				onclick={onClose}
			>
				<XIcon class="h-4 w-4" />
			</button>
		</div>
		<div class="flex flex-1 overflow-hidden">
			<div class="w-56 shrink-0 border-r border-border">
				<SettingsCategoryNav mode="app" />
			</div>
			<div class="flex-1 overflow-hidden">
				<SettingsView />
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>

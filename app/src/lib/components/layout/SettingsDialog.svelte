<script lang="ts">
	import {
		Icon,
		Button,
		HStack,
		Box,
		Panel,
		DialogRoot,
		DialogContent,
		DialogTitle,
		DialogDescription,
	} from "@orqastudio/svelte-components/pure";
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
	onOpenChange={(isOpen) => {
		if (!isOpen) onClose();
	}}
>
	<DialogContent>
		<HStack justify="between">
			<DialogTitle>Settings</DialogTitle>
			<DialogDescription>Application settings</DialogDescription>
			<Button variant="ghost" size="icon-sm" onclick={onClose}>
				<Icon name="x" size="md" />
			</Button>
		</HStack>
		<HStack gap={0} flex={1} align="stretch">
			<Panel fixedWidth="nav-lg" border="right" padding="none">
				<SettingsCategoryNav
					mode="app"
					activeSection={dialogSection}
					onSectionChange={(s) => {
						dialogSection = s;
					}}
				/>
			</Panel>
			<Box flex={1}>
				<SettingsView activeSection={dialogSection} />
			</Box>
		</HStack>
	</DialogContent>
</DialogRoot>

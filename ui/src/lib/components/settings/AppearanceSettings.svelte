<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import SelectMenu from "$lib/components/shared/SelectMenu.svelte";
	import { settingsStore, type ThemeMode } from "$lib/stores/settings.svelte";

	const themeModeOptions: { value: ThemeMode; label: string }[] = [
		{ value: "system", label: "System (default)" },
		{ value: "light", label: "Light" },
		{ value: "dark", label: "Dark" },
	];

	function handleThemeChange(value: string): void {
		settingsStore.setThemeMode(value as ThemeMode);
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Appearance</Card.Title>
		<Card.Description>Theme and display preferences</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-4">
		<div>
			<span class="text-sm font-medium">Theme</span>
			<div class="mt-1">
				<SelectMenu
					items={themeModeOptions}
					selected={settingsStore.themeMode}
					onSelect={handleThemeChange}
					triggerLabel={themeModeOptions.find((o) => o.value === settingsStore.themeMode)?.label ?? "System"}
					triggerSize="default"
					align="start"
				/>
			</div>
		</div>
	</Card.Content>
</Card.Root>

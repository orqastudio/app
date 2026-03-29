<script lang="ts">
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { SelectMenu } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import type { ThemeMode } from "@orqastudio/sdk";

	const { settingsStore } = getStores();

	const themeModeOptions: { value: ThemeMode; label: string }[] = [
		{ value: "system", label: "System (default)" },
		{ value: "light", label: "Light" },
		{ value: "dark", label: "Dark" },
	];

	/**
	 * Applies the selected theme mode to the settings store.
	 * @param value - The theme mode value to apply.
	 */
	function handleThemeChange(value: string): void {
		settingsStore.setThemeMode(value as ThemeMode);
	}
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Appearance</CardTitle>
		<CardDescription>Theme and display preferences</CardDescription>
	</CardHeader>
	<CardContent class="space-y-4">
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
	</CardContent>
</CardRoot>

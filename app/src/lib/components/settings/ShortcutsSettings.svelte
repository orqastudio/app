<script lang="ts">
	import {
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
	} from "@orqastudio/svelte-components/pure";
	import { HStack, Stack, Caption, Kbd, Panel } from "@orqastudio/svelte-components/pure";

	const shortcuts: { key: string; action: string }[] = [
		{ key: "Ctrl+Space", action: "Search" },
		{ key: "Ctrl+B", action: "Toggle Nav Sub-Panel" },
		{ key: "Ctrl+N", action: "New session" },
		{ key: "Ctrl+1-5", action: "Switch activity view" },
		{ key: "Ctrl+,", action: "Open settings" },
	];
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Keyboard Shortcuts</CardTitle>
		<CardDescription>Reference card for available shortcuts</CardDescription>
	</CardHeader>
	<CardContent>
		<Stack gap={2}>
			{#each shortcuts as shortcut (shortcut.key)}
				<Panel padding="tight">
					<HStack justify="between">
						<Caption tone="muted">{shortcut.action}</Caption>
						<Kbd>{shortcut.key}</Kbd>
					</HStack>
				</Panel>
			{/each}
		</Stack>
	</CardContent>
</CardRoot>

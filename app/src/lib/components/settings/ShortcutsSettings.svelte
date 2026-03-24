<script lang="ts">
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";

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
		<div class="space-y-2">
			{#each shortcuts as shortcut (shortcut.key)}
				<div class="flex items-center justify-between rounded px-2 py-1.5 text-sm hover:bg-muted/50">
					<span class="text-muted-foreground">{shortcut.action}</span>
					<Badge variant="outline" class="font-mono text-xs">{shortcut.key}</Badge>
				</div>
			{/each}
		</div>
	</CardContent>
</CardRoot>

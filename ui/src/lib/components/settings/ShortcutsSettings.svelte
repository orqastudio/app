<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge";

	const shortcuts: { key: string; action: string }[] = [
		{ key: "Ctrl+B", action: "Toggle Nav Sub-Panel" },
		{ key: "Ctrl+N", action: "New session" },
		{ key: "Ctrl+1-5", action: "Switch activity view" },
		{ key: "Ctrl+,", action: "Open settings" },
	];
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Keyboard Shortcuts</Card.Title>
		<Card.Description>Reference card for available shortcuts</Card.Description>
	</Card.Header>
	<Card.Content>
		<div class="space-y-2">
			{#each shortcuts as shortcut (shortcut.key)}
				<div class="flex items-center justify-between rounded px-2 py-1.5 text-sm hover:bg-muted/50">
					<span class="text-muted-foreground">{shortcut.action}</span>
					<Badge variant="outline" class="font-mono text-xs">{shortcut.key}</Badge>
				</div>
			{/each}
		</div>
	</Card.Content>
</Card.Root>

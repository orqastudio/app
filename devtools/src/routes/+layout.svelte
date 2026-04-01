<script lang="ts">
	// Root layout for OrqaDev. Wraps the entire app in TooltipProvider and
	// mounts the DevToolsShell, which owns tab navigation and the status bar.
	import "../app.css";
	import { TooltipProvider } from "@orqastudio/svelte-components/pure";
	import DevToolsShell from "$lib/components/DevToolsShell.svelte";

	let { children } = $props();
</script>

<TooltipProvider>
	<div class="bg-surface-base text-content-base flex h-screen flex-col overflow-hidden">
		<DevToolsShell>
			{@render children()}
		</DevToolsShell>
	</div>
</TooltipProvider>

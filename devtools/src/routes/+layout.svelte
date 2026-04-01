<script lang="ts">
	import "../app.css";
	import { TooltipProvider } from "@orqastudio/svelte-components/pure";

	let { children } = $props();
</script>

<TooltipProvider>
	<div class="bg-surface-base text-content-base flex h-screen flex-col overflow-hidden">
		{@render children()}
	</div>
</TooltipProvider>

<!-- Purely visual checkbox indicator — renders the checked or unchecked state
     without any interactive surface. Use inside a Button when the button itself
     is the interactive element and the checkbox is a visual cue only. -->
<script lang="ts">
	import { cn } from "../../utils/cn.js";
	import { Icon } from "../icon/index.js";

	let { checked = false }: { checked?: boolean } = $props();
</script>

<span
	class={cn(
		"pointer-events-none flex h-3.5 w-3.5 shrink-0 items-center justify-center rounded-sm border",
		checked ? "border-primary bg-primary" : "border-muted-foreground/40",
	)}
	aria-hidden="true"
>
	{#if checked}
		<Icon name="check" size="sm" />
	{/if}
</span>

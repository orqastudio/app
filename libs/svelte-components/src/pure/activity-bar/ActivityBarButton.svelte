<!-- ActivityBarButton — a square icon-only button for the vertical activity bar.
     Renders a 40×40px button with an icon centered inside.
     Active state applies accent background and foreground; inactive state uses muted text
     with an accent hover. Intended for app-level navigation switching (e.g. artifacts, chat, settings). -->
<script lang="ts">
	import { cn } from "../../utils/cn.js";
	import { Icon } from "../icon/index.js";
	import { VisuallyHidden } from "../visually-hidden/index.js";

	let {
		icon,
		label,
		active = false,
		onclick,
	}: {
		/** Lucide icon name to render inside the button. */
		icon: string;
		/** Accessible label (visually hidden). */
		label: string;
		/** Whether this activity is currently active. */
		active?: boolean;
		onclick?: () => void;
	} = $props();
</script>

<button
	class={cn(
		"mb-1 flex h-10 w-10 items-center justify-center rounded-md transition-colors",
		active
			? "bg-accent text-accent-foreground"
			: "text-muted-foreground hover:bg-accent/50 hover:text-foreground",
	)}
	{onclick}
>
	<Icon name={icon} size="lg" />
	<VisuallyHidden>{label}</VisuallyHidden>
</button>

<script lang="ts" module>
	export type ConnectionState = "connected" | "reconnecting" | "disconnected" | "waiting";
</script>

<script lang="ts">
	import { HStack } from "../layout/index.js";
	import { Text } from "../typography/index.js";

	let {
		state = "waiting",
		label,
	}: {
		state?: ConnectionState;
		label?: string;
	} = $props();

	const dotClass = $derived(
		state === "connected"
			? "bg-green-500"
			: state === "reconnecting"
				? "bg-yellow-500"
				: "bg-red-500",
	);

	const defaultLabel = $derived(
		state === "connected"
			? "Connected"
			: state === "reconnecting"
				? "Reconnecting..."
				: state === "disconnected"
					? "Disconnected"
					: "Waiting...",
	);
</script>

<HStack gap={1.5}>
	<span class="size-2 rounded-full {dotClass}"></span>
	<Text variant="caption">{label ?? defaultLabel}</Text>
</HStack>

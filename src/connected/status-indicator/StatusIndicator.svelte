<script lang="ts">
	import { Status } from "../../pure/status/index.js";
	import { getStores } from "../../testing/mock-stores.js";
	import type { StatusConfig } from "../../pure/status/status-utils.js";

	let {
		status,
		mode = "badge",
	}: {
		status: string;
		mode?: "badge" | "dot" | "inline";
	} = $props();

	// Resolve statuses from the store's project config
	const stores = getStores();
	const statuses = $derived<StatusConfig[] | undefined>(
		stores.projectStore?.settings?.statuses?.map((s: { key: string; label: string; icon?: string; spin?: boolean }) => ({
			key: s.key,
			label: s.label,
			icon: s.icon ?? "circle",
			spin: s.spin,
		}))
	);
</script>

<Status {status} {statuses} {mode} />

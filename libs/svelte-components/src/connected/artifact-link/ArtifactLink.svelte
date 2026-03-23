<script lang="ts">
	import { getStores } from "../../testing/mock-stores.js";

	let {
		artifactId,
	}: {
		artifactId: string;
	} = $props();

	const stores = getStores();
	const artifact = $derived(stores.artifactStore.getArtifact(artifactId));
</script>

<button
	class="text-primary hover:underline underline-offset-2 text-sm"
	onclick={() => stores.navigationStore.navigate("artifact", artifactId)}
>
	{artifact?.title ?? artifactId}
</button>

<script lang="ts">
	interface CardActionProps {
		children?: import("svelte").Snippet;
	}

	let { children }: CardActionProps = $props();
</script>

<div data-slot="card-action" class="col-start-2 row-span-2 row-start-1 self-start justify-self-end">
	{@render children?.()}
</div>

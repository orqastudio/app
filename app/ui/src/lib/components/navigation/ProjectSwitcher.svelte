<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { projectStore } = getStores();

	const isOrg = $derived(projectStore.isOrganisation);
	const children = $derived(projectStore.childProjects);
	const active = $derived(projectStore.activeChildProject);

	let open = $state(false);

	function select(name: string | null) {
		projectStore.activeChildProject = name;
		open = false;
	}
</script>

{#if isOrg}
	<div class="relative">
		<button
			class="flex items-center gap-1.5 rounded border border-border bg-background px-2 py-1 text-xs font-medium text-foreground hover:bg-muted transition-colors"
			onclick={() => { open = !open; }}
		>
			<Icon name="layers" size="sm" />
			<span class="max-w-[120px] truncate">{active ?? "All Projects"}</span>
			<Icon name="chevron-down" size="xs" />
		</button>

		{#if open}
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				class="fixed inset-0 z-40"
				onclick={() => { open = false; }}
				onkeydown={(e) => { if (e.key === "Escape") open = false; }}
			></div>
			<div class="absolute left-0 top-full z-50 mt-1 min-w-[160px] rounded border border-border bg-popover shadow-md">
				<button
					class="flex w-full items-center gap-2 px-3 py-1.5 text-xs hover:bg-muted transition-colors {active === null ? 'font-semibold text-primary' : 'text-foreground'}"
					onclick={() => select(null)}
				>
					<Icon name="layers" size="sm" />
					All Projects
				</button>
				{#each children as child}
					<button
						class="flex w-full items-center gap-2 px-3 py-1.5 text-xs hover:bg-muted transition-colors {active === child.name ? 'font-semibold text-primary' : 'text-foreground'}"
						onclick={() => select(child.name)}
					>
						<Icon name="folder" size="sm" />
						{child.name}
					</button>
				{/each}
			</div>
		{/if}
	</div>
{/if}

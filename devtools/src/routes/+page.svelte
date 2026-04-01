<!-- OrqaDev Logs tab — the default view. Renders the filter bar above the
     virtualised log table. Both components share the filter state in
     log-store.svelte.ts so filtering is reactive without prop-drilling. -->
<script lang="ts">
	import LogFilters from "$lib/components/logs/LogFilters.svelte";
	import LogTable from "$lib/components/logs/LogTable.svelte";
</script>

<!-- Flex column so LogFilters is pinned above LogTable and LogTable fills the rest. -->
<div class="flex h-full flex-col overflow-hidden">
	<LogFilters />
	<LogTable />
</div>

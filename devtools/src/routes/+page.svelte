<!-- OrqaDev Logs tab — the default view. Renders the session picker, filter bar,
     and virtualised log table. All components share reactive state from the
     session and log stores so session switching and filtering are reactive
     without prop-drilling. -->
<script lang="ts">
	import { Stack } from "@orqastudio/svelte-components/pure";
	import SessionPicker from "$lib/components/logs/SessionPicker.svelte";
	import LogFilters from "$lib/components/logs/LogFilters.svelte";
	import LogTable from "$lib/components/logs/LogTable.svelte";
</script>

<!-- Vertical stack: SessionPicker and LogFilters pinned above, LogTable fills the rest. -->
<Stack gap={0} class="h-full overflow-hidden">
	<SessionPicker />
	<LogFilters />
	<LogTable />
</Stack>

<!-- OrqaDev Logs tab — the default view. Renders the session picker, filter bar,
     and virtualised log table. All components share reactive state from the
     session and log stores so session switching and filtering are reactive
     without prop-drilling. -->
<script lang="ts">
	import { Stack, Box } from "@orqastudio/svelte-components/pure";
	import SessionPicker from "$lib/components/logs/SessionPicker.svelte";
	import LogFilters from "$lib/components/logs/LogFilters.svelte";
	import LogTable from "$lib/components/logs/LogTable.svelte";
</script>

<!-- Vertical stack: SessionPicker and LogFilters pinned above, LogTable fills the rest.
     height="full" bounds the virtualized log table within the tab content area.
     Box with flex={1} minHeight={0} constrains LogTable so it fills remaining space
     without pushing SessionPicker and LogFilters off screen. -->
<Stack gap={0} height="full">
	<SessionPicker />
	<LogFilters />
	<Box flex={1} minHeight={0} height="full">
		<LogTable />
	</Box>
</Stack>

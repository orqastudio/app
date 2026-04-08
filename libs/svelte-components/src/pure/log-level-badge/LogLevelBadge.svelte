<!-- LogLevelBadge — compact uppercase monospace badge for log level columns in
     virtualised log tables. Fixed 42px width with centered text for column alignment.

     Selects the correct Badge variant automatically from the log level string.
     Supports Debug | Info | Warn | Error | Perf; falls back to "outline" for unknown levels. -->
<script lang="ts">
	import { Badge } from "../badge/index.js";

	// Badge variant for each log level. Matches the mapping used in LogRow and LogFilters.
	const LEVEL_VARIANTS: Record<
		string,
		"secondary" | "destructive" | "outline" | "default" | "warning"
	> = {
		Debug: "outline",
		Info: "default",
		Warn: "warning",
		Error: "destructive",
		Perf: "secondary",
	};

	let {
		level,
	}: {
		/** Log level string from the event. */
		level: string;
	} = $props();

	const variant = $derived(LEVEL_VARIANTS[level] ?? "outline");
</script>

<!-- size="log" applies fixed 42px width, centered, uppercase, monospace — matches log column layout. -->
<Badge {variant} size="log">
	{level}
</Badge>

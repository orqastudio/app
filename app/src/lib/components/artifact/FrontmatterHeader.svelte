<!-- Renders structured frontmatter fields for a governance artifact: title, description, status, priority, dates, links, chips, and gate questions. -->
<script lang="ts">
	import { Icon, CardRoot, CardContent, HStack, Box, Heading, Badge, Stack } from "@orqastudio/svelte-components/pure";
	import { ArtifactLink } from "@orqastudio/svelte-components/connected";
	import GateQuestions from "./GateQuestions.svelte";
	import { StatusIndicator } from "@orqastudio/svelte-components/connected";
	import { getCapabilityLabel } from "$lib/utils/tool-display";
	import { logger } from "@orqastudio/sdk";
	import {
		SKIP_FIELDS,
		DATE_FIELDS,
		LINK_FIELDS,
		CHIP_FIELDS,
		BOOLEAN_FIELDS,
		FIELD_ORDER,
		priorityClass,
		priorityLabel,
	} from "$lib/config/frontmatter-config";

	const log = logger("frontmatter");

	let {
		metadata,
		artifactType,
	}: {
		metadata: Record<string, unknown>;
		artifactType: string;
	} = $props();

	/**
	 * Format an ISO date string to a readable date.
	 * @param value - Raw date value from frontmatter
	 * @returns Formatted date string or null for invalid values
	 */
	function formatDate(value: unknown): string | null {
		if (value === null || value === undefined || value === "" || value === "null") return null;
		try {
			const d = new Date(String(value));
			if (isNaN(d.getTime())) return null;
			return d.toLocaleDateString(undefined, {
				year: "numeric",
				month: "short",
				day: "numeric",
			});
		} catch (err) {
			log.debug("Failed to parse date in formatDate", { value, err });
			return null;
		}
	}

	/**
	 * Returns true if a value is non-empty.
	 * @param value - Value to check
	 * @returns True if non-null, non-empty
	 */
	function isPresent(value: unknown): boolean {
		if (value === null || value === undefined) return false;
		if (value === "" || value === "null") return false;
		if (Array.isArray(value) && value.length === 0) return false;
		return true;
	}

	/**
	 * @param value - Value to coerce to string array
	 * @returns Array of strings
	 */
	function asArray(value: unknown): string[] {
		if (Array.isArray(value)) return value.map(String);
		if (typeof value === "string") return [value];
		return [String(value)];
	}

	/** Classify a field key into its render type. */
	type FieldType = "date" | "link" | "chip" | "boolean" | "generic";

	/**
	 * @param key - Frontmatter field key
	 * @returns Render type for the field
	 */
	function fieldType(key: string): FieldType {
		if (DATE_FIELDS.has(key)) return "date";
		if (LINK_FIELDS.has(key)) return "link";
		if (BOOLEAN_FIELDS.has(key)) return "boolean";
		if (CHIP_FIELDS.has(key)) return "chip";
		return "generic";
	}

	/**
	 * Humanize a kebab-case field key for display.
	 * @param key - Kebab-case field key
	 * @returns Human-readable label
	 */
	function humanizeKey(key: string): string {
		return key
			.replace(/-/g, " ")
			.replace(/_/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	// --- Derived header values (always rendered first) ---
	const id = $derived(metadata["id"] as string | undefined);
	const title = $derived(metadata["title"] as string | undefined);
	const description = $derived(metadata["description"] as string | undefined);
	const status = $derived(metadata["status"] as string | undefined);
	const priority = $derived(
		isPresent(metadata["priority"]) ? String(metadata["priority"]) : undefined,
	);

	/** Scoring dimensions as key-value pairs for inline display. */
	const scoringEntries = $derived.by(() => {
		const raw = metadata["scoring"];
		if (raw === null || raw === undefined || typeof raw !== "object" || Array.isArray(raw)) return [];
		return Object.entries(raw as Record<string, unknown>).filter(
			([, v]) => v !== null && v !== undefined,
		);
	});

	/**
	 * Short date format for the header chip.
	 * @param value - Raw date value
	 * @returns Short formatted date or null
	 */
	function shortDate(value: unknown): string | null {
		if (value === null || value === undefined || value === "" || value === "null") return null;
		try {
			const d = new Date(String(value));
			if (isNaN(d.getTime())) return null;
			return d.toLocaleDateString(undefined, { month: "short", day: "numeric" });
		} catch (err) {
			log.debug("Failed to parse date in shortDate", { value, err });
			return null;
		}
	}

	const createdShort = $derived(shortDate(metadata["created"]));
	const updatedShort = $derived(shortDate(metadata["updated"]));
	const dateChip = $derived(
		createdShort && updatedShort && createdShort !== updatedShort
			? `${createdShort} → ${updatedShort}`
			: createdShort ?? updatedShort,
	);

	/** Gate — supports both a single string (milestones) and an array (pillars). */
	const gateQuestions = $derived(
		isPresent(metadata["gate"]) ? asArray(metadata["gate"]).filter(Boolean) : [],
	);

	/** Capabilities with human-friendly names for display. */
	const appTools = $derived(
		isPresent(metadata["capabilities"])
			? asArray(metadata["capabilities"]).map(getCapabilityLabel)
			: [],
	);

	/**
	 * The ordered body entries from the metadata object, skipping:
	 * - Fixed header fields (SKIP_FIELDS)
	 * - Progress fields (rendered as a combined row)
	 * - Gate field (rendered separately at the end)
	 * - Entries without a present value
	 */
	const bodyEntries = $derived.by(() => {
		const filtered = Object.entries(metadata).filter(([key, value]) => {
			if (SKIP_FIELDS.has(key)) return false;
			if (key === "gate") return false;
			if (!isPresent(value)) return false;
			return true;
		});
		return filtered.sort(([a], [b]) => {
			const ai = FIELD_ORDER.indexOf(a);
			const bi = FIELD_ORDER.indexOf(b);
			// Both in order list: sort by position
			if (ai !== -1 && bi !== -1) return ai - bi;
			// Only one in order list: it comes first
			if (ai !== -1) return -1;
			if (bi !== -1) return 1;
			// Neither: preserve original order (stable sort)
			return 0;
		});
	});

	/** True when the card has content below the header row. */
	const hasBody = $derived(bodyEntries.length > 0 || appTools.length > 0 || gateQuestions.length > 0 || scoringEntries.length > 0);
</script>

<!-- Title -->
{#if title}
	<Heading level={1}>{title}</Heading>
{/if}

<!-- Description + metadata card share a Stack gap={4} so no marginTop or mb-* spacers are needed. -->
<Stack gap={4}>
{#if description}
	<!-- leading-relaxed is not in Text variants, kept as p. -->
	<p class="text-sm leading-relaxed text-muted-foreground">{description}</p>
{/if}

<!-- Metadata card -->
<CardRoot gap={0}>
	<CardContent>
		<!-- ID + Status/Priority row — only rendered when at least one value is present -->
		{#if id || (status && isPresent(status)) || priority || dateChip}
			<HStack justify="between" gap={3} align={hasBody ? "start" : "center"}>
				<Stack gap={0.5}>
					{#if id}
						<!-- tracking-widest and uppercase not in Text variants, kept as p. -->
						<p class="font-mono text-xs font-semibold uppercase tracking-widest text-muted-foreground">
							{artifactType} · {id}
						</p>
					{/if}
				</Stack>

				<HStack gap={2} flex={0}>
					{#if createdShort}
						<span class="text-muted-foreground"><Badge variant="secondary">
							<Icon name="calendar-plus" size="xs" />{createdShort}
						</Badge></span>
					{/if}
					{#if updatedShort && updatedShort !== createdShort}
						<span class="text-muted-foreground"><Badge variant="secondary">
							<Icon name="calendar-check" size="xs" />{updatedShort}
						</Badge></span>
					{/if}
					{#if priority}
						<span class={priorityClass(priority)}><Badge variant="outline">
							{priorityLabel(priority)}
						</Badge></span>
					{/if}
					{#if status && isPresent(status)}
						<StatusIndicator {status} mode="badge" />
					{/if}
				</HStack>
			</HStack>
		{/if}

		<!-- Scoring dimensions (shown near priority when present) -->
		{#if priority && scoringEntries.length > 0}
			<HStack align="baseline" gap={2}>
				<!-- w-[7rem] is an arbitrary width not in Box/Stack props, kept as span. -->
				<span class="w-[7rem] shrink-0 text-xs font-medium text-muted-foreground">
					<HStack gap={1}>
						<Icon name="scale" size="xs" />Scoring
					</HStack>
				</span>
				<Box flex={1} minWidth={0}><HStack wrap gap={1}>
					{#each scoringEntries as [key, val] (key)}
						<Badge variant="secondary">
							<span class="font-normal"><span class="text-muted-foreground">{humanizeKey(key)}:</span> {String(val)}</span>
						</Badge>
					{/each}
				</HStack></Box>
			</HStack>
		{/if}

		<!-- Dynamic body — YAML source order, type-dispatched -->
		{#each bodyEntries as [key, value] (key)}
			{@const type = fieldType(key)}
			{#if type === "date"}
				{@const formatted = formatDate(value)}
				{#if formatted}
					<HStack align="baseline" gap={2}>
						<!-- w-[7rem] is an arbitrary width not in Box/Stack props, kept as span. -->
						<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
							{humanizeKey(key)}
						</span>
						<span class="text-xs text-foreground">{formatted}</span>
					</HStack>
				{/if}

			{:else if type === "link"}
				{@const vals = asArray(value).filter(Boolean)}
				{#if vals.length > 0}
					<HStack align="baseline" gap={2}>
						<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
							{humanizeKey(key)}
						</span>
						<Box flex={1} minWidth={0}><HStack wrap gap={1}>
							{#each vals as val, i (i)}
								<ArtifactLink id={val.trim()} />
							{/each}
						</HStack></Box>
					</HStack>
				{/if}

			{:else if type === "chip"}
				{@const items = asArray(value).filter(Boolean)}
				{#if items.length > 0}
					<HStack align="baseline" gap={2}>
						<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
							{humanizeKey(key)}
						</span>
						<Box flex={1} minWidth={0}><HStack wrap gap={1}>
							{#each items as item, i (i)}
								<span class="capitalize"><Badge variant="secondary">{item}</Badge></span>
							{/each}
						</HStack></Box>
					</HStack>
				{/if}

			{:else if type === "boolean"}
				<HStack gap={2}>
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					{#if value}
						<Icon name="check" size="md" />
					{:else}
						<Icon name="x" size="md" />
					{/if}
				</HStack>

			{:else}
				<!-- generic -->
				<HStack align="baseline" gap={2}>
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					{#if Array.isArray(value)}
						<Box flex={1} minWidth={0}><HStack wrap gap={1}>
							{#each value as v, i (i)}
								<span class="capitalize"><Badge variant="secondary">{v}</Badge></span>
							{/each}
						</HStack></Box>
					{:else if typeof value === "object" && value !== null}
						<Box flex={1} minWidth={0}><HStack wrap gap={1}>
							{#each Object.entries(value as Record<string, unknown>) as [k, v], i (i)}
								<Badge variant="secondary">
									<span class="text-muted-foreground">{humanizeKey(k)}:</span> {String(v)}
								</Badge>
							{/each}
						</HStack></Box>
					{:else}
						<span class="min-w-0 flex-1 text-xs capitalize text-foreground">{String(value)}</span>
					{/if}
				</HStack>
			{/if}
		{/each}

		<!-- Capabilities (human-friendly names) -->
		{#if appTools.length > 0}
			<HStack align="baseline" gap={2}>
				<!-- w-[7rem] + capitalize not in Stack/Box primitives, kept as span. -->
				<span class="inline-flex w-[7rem] shrink-0 items-center gap-1 text-xs font-medium capitalize text-muted-foreground">
					<Icon name="wrench" size="xs" />Capabilities
				</span>
				<Box flex={1} minWidth={0}><HStack wrap gap={1}>
					{#each appTools as tool, i (i)}
						<Badge variant="secondary">{tool}</Badge>
					{/each}
				</HStack></Box>
			</HStack>
		{/if}

		<!-- Gate question(s) — always last -->
		<GateQuestions questions={gateQuestions} />
	</CardContent>
</CardRoot>
</Stack>

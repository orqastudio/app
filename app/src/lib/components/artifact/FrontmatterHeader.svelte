<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import ArtifactLink from "./ArtifactLink.svelte";
	import GateQuestions from "./GateQuestions.svelte";
	import { StatusIndicator } from "@orqastudio/svelte-components/connected";
	import { Badge } from "@orqastudio/svelte-components/pure";
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

	/** Format an ISO date string to a readable date; returns null for invalid/null values. */
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

/** Returns true if a value is non-empty (not null, undefined, empty string, or "null"). */
	function isPresent(value: unknown): boolean {
		if (value === null || value === undefined) return false;
		if (value === "" || value === "null") return false;
		if (Array.isArray(value) && value.length === 0) return false;
		return true;
	}

	function asArray(value: unknown): string[] {
		if (Array.isArray(value)) return value.map(String);
		if (typeof value === "string") return [value];
		return [String(value)];
	}

	/** Classify a field key into its render type. */
	type FieldType = "date" | "link" | "chip" | "boolean" | "generic";

	function fieldType(key: string): FieldType {
		if (DATE_FIELDS.has(key)) return "date";
		if (LINK_FIELDS.has(key)) return "link";
		if (BOOLEAN_FIELDS.has(key)) return "boolean";
		if (CHIP_FIELDS.has(key)) return "chip";
		return "generic";
	}

	/** Humanize a kebab-case field key for display. */
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

	/** Short date format for the header chip (e.g. "Jan 5"). */
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

	/** Capabilities (or legacy tools) with human-friendly names for display. */
	const appTools = $derived.by(() => {
		// Prefer capabilities field (current); fall back to tools (legacy)
		if (isPresent(metadata["capabilities"])) {
			return asArray(metadata["capabilities"]).map(getCapabilityLabel);
		}
		if (isPresent(metadata["tools"])) {
			return asArray(metadata["tools"])
				.filter((t) => !t.startsWith("mcp__"))
				.map(getCapabilityLabel);
		}
		return [];
	});

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
	<h1 class="mb-1 text-2xl font-bold leading-snug">{title}</h1>
{/if}

<!-- Description -->
{#if description}
	<p class="mb-4 text-sm leading-relaxed text-muted-foreground">{description}</p>
{:else if title}
	<div class="mb-4"></div>
{/if}

<!-- Metadata card -->
<div class="mb-4 space-y-3 rounded-lg border border-border bg-muted/30 px-4 py-3">
	<!-- ID + Status/Priority row — only rendered when at least one value is present -->
	{#if id || (status && isPresent(status)) || priority || dateChip}
		<div class="flex justify-between gap-3" class:items-center={!hasBody} class:items-start={hasBody}>
			<div class="space-y-0.5">
				{#if id}
					<p class="font-mono text-xs font-semibold uppercase tracking-widest text-muted-foreground">
						{artifactType} · {id}
					</p>
				{/if}
			</div>

			<div class="flex shrink-0 items-center gap-2">
				{#if createdShort}
					<Badge variant="secondary" class="text-muted-foreground">
						<Icon name="calendar-plus" size="xs" />{createdShort}
					</Badge>
				{/if}
				{#if updatedShort && updatedShort !== createdShort}
					<Badge variant="secondary" class="text-muted-foreground">
						<Icon name="calendar-check" size="xs" />{updatedShort}
					</Badge>
				{/if}
				{#if priority}
					<Badge variant="outline" class={priorityClass(priority)}>
						{priorityLabel(priority)}
					</Badge>
				{/if}
				{#if status && isPresent(status)}
					<StatusIndicator {status} mode="badge" />
				{/if}
			</div>
		</div>
	{/if}

	<!-- Scoring dimensions (shown near priority when present) -->
	{#if priority && scoringEntries.length > 0}
		<div class="flex items-baseline gap-2">
			<span class="w-[7rem] shrink-0 text-xs font-medium text-muted-foreground">
				<span class="inline-flex items-center gap-1">
					<Icon name="scale" size="xs" />Scoring
				</span>
			</span>
			<div class="flex min-w-0 flex-1 flex-wrap gap-1">
				{#each scoringEntries as [key, val] (key)}
					<Badge variant="secondary" class="font-normal">
						<span class="text-muted-foreground">{humanizeKey(key)}:</span> {String(val)}
					</Badge>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Dynamic body — YAML source order, type-dispatched -->
	{#each bodyEntries as [key, value] (key)}
		{@const type = fieldType(key)}
		{#if type === "date"}
			{@const formatted = formatDate(value)}
			{#if formatted}
				<div class="flex items-baseline gap-2">
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<span class="text-xs text-foreground">{formatted}</span>
				</div>
			{/if}

		{:else if type === "link"}
			{@const vals = asArray(value).filter(Boolean)}
			{#if vals.length > 0}
				<div class="flex items-baseline gap-2">
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each vals as val, i (i)}
							<ArtifactLink id={val.trim()} />
						{/each}
					</div>
				</div>
			{/if}

		{:else if type === "chip"}
			{@const items = asArray(value).filter(Boolean)}
			{#if items.length > 0}
				<div class="flex items-baseline gap-2">
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each items as item, i (i)}
							<Badge variant="secondary" class="capitalize">{item}</Badge>
						{/each}
					</div>
				</div>
			{/if}

		{:else if type === "boolean"}
			<div class="flex items-center gap-2">
				<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
					{humanizeKey(key)}
				</span>
				{#if value}
					<Icon name="check" size="md" />
				{:else}
					<Icon name="x" size="md" />
				{/if}
			</div>

		{:else}
			<!-- generic -->
			<div class="flex items-baseline gap-2">
				<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
					{humanizeKey(key)}
				</span>
				{#if Array.isArray(value)}
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each value as v, i (i)}
							<Badge variant="secondary" class="capitalize">{v}</Badge>
						{/each}
					</div>
				{:else if typeof value === "object" && value !== null}
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each Object.entries(value as Record<string, unknown>) as [k, v], i (i)}
							<Badge variant="secondary">
								<span class="text-muted-foreground">{humanizeKey(k)}:</span> {String(v)}
							</Badge>
						{/each}
					</div>
				{:else}
					<span class="min-w-0 flex-1 text-xs capitalize text-foreground">{String(value)}</span>
				{/if}
			</div>
		{/if}
	{/each}

	<!-- Capabilities (human-friendly names) -->
	{#if appTools.length > 0}
		<div class="flex items-baseline gap-2">
			<span class="inline-flex w-[7rem] shrink-0 items-center gap-1 text-xs font-medium capitalize text-muted-foreground">
				<Icon name="wrench" size="xs" />Capabilities
			</span>
			<div class="flex min-w-0 flex-1 flex-wrap gap-1">
				{#each appTools as tool, i (i)}
					<Badge variant="secondary">{tool}</Badge>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Gate question(s) — always last -->
	<GateQuestions questions={gateQuestions} />
</div>

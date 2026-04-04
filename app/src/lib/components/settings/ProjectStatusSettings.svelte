<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent, FormGroup } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Input } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { ConfirmDialog as ConfirmDeleteDialog } from "@orqastudio/svelte-components/pure";
	import { Switch } from "@orqastudio/svelte-components/pure";
	import type { ProjectSettings, StatusDefinition, StatusAutoRule } from "@orqastudio/types";

	interface Props {
		settings: ProjectSettings;
		onSave: (settings: ProjectSettings) => void;
	}

	const props: Props = $props();

	// localStatuses is a local edit buffer initialized from props.settings and mutated
	// by the update functions below. The $effect re-syncs when the prop changes
	// externally (e.g. undo or external save). $derived.by is inappropriate here
	// because the local state is intentionally mutated independently of the prop.
	// eslint-disable-next-line svelte/prefer-writable-derived
	let localStatuses = $state<StatusDefinition[]>([]);
	let deleteIndex = $state<number | null>(null);
	let confirmDeleteOpen = $state(false);

	// Drag state
	let dragIndex = $state<number | null>(null);
	let dragOverIndex = $state<number | null>(null);

	$effect(() => {
		localStatuses = (props.settings.statuses ?? []).map((s) => ({
			...s,
			transitions: [...(s.transitions ?? [])],
			auto_rules: (s.auto_rules ?? []).map((r) => ({ ...r })),
		}));
	});

	/**
	 * Constructs a ProjectSettings object from the current local statuses.
	 * @returns The merged ProjectSettings with updated status definitions.
	 */
	function buildSettings(): ProjectSettings {
		return {
			...props.settings,
			statuses: localStatuses.map((s) => ({ ...s })),
		};
	}

	/** Persists the current local statuses to the parent via onSave. */
	function save() {
		props.onSave(buildSettings());
	}

	/**
	 * Updates a single field on a status definition and saves.
	 * @param index - The index of the status in the local list.
	 * @param field - The StatusDefinition field to update.
	 * @param value - The new value for the field.
	 */
	function updateField(index: number, field: keyof StatusDefinition, value: string | boolean) {
		localStatuses = localStatuses.map((s, i) => (i === index ? { ...s, [field]: value } : s));
		save();
	}

	/**
	 * Adds or removes a transition target for a status, then saves.
	 * @param statusIndex - The index of the status to modify.
	 * @param targetKey - The status key to toggle in the transitions list.
	 */
	function toggleTransition(statusIndex: number, targetKey: string) {
		localStatuses = localStatuses.map((s, i) => {
			if (i !== statusIndex) return s;
			const current = s.transitions ?? [];
			const transitions = current.includes(targetKey)
				? current.filter((k) => k !== targetKey)
				: [...current, targetKey];
			return { ...s, transitions };
		});
		save();
	}

	/**
	 * Appends an empty auto-rule to a status definition.
	 * @param index - The index of the status to add the rule to.
	 */
	function addAutoRule(index: number) {
		localStatuses = localStatuses.map((s, i) => {
			if (i !== index) return s;
			const auto_rules = [...(s.auto_rules ?? []), { condition: "", target: "" }];
			return { ...s, auto_rules };
		});
	}

	/**
	 * Updates a single field on an auto-rule and saves.
	 * @param statusIndex - The index of the status that owns the rule.
	 * @param ruleIndex - The index of the rule within the status.
	 * @param field - The StatusAutoRule field to update.
	 * @param value - The new value for the field.
	 */
	function updateAutoRule(
		statusIndex: number,
		ruleIndex: number,
		field: keyof StatusAutoRule,
		value: string,
	) {
		localStatuses = localStatuses.map((s, i) => {
			if (i !== statusIndex) return s;
			const auto_rules = (s.auto_rules ?? []).map((r, ri) =>
				ri === ruleIndex ? { ...r, [field]: value } : r,
			);
			return { ...s, auto_rules };
		});
		save();
	}

	/**
	 * Removes an auto-rule from a status definition and saves.
	 * @param statusIndex - The index of the status that owns the rule.
	 * @param ruleIndex - The index of the rule to remove.
	 */
	function removeAutoRule(statusIndex: number, ruleIndex: number) {
		localStatuses = localStatuses.map((s, i) => {
			if (i !== statusIndex) return s;
			const auto_rules = (s.auto_rules ?? []).filter((_, ri) => ri !== ruleIndex);
			return { ...s, auto_rules };
		});
		save();
	}

	/** Appends a new blank status definition to the list and saves. */
	function addStatus() {
		const newStatus: StatusDefinition = {
			key: `status_${Date.now()}`,
			label: "New Status",
			icon: "circle",
			spin: false,
			transitions: [],
			auto_rules: [],
		};
		localStatuses = [...localStatuses, newStatus];
		save();
	}

	/**
	 * Marks a status for deletion and opens the confirmation dialog.
	 * @param index - The index of the status to delete.
	 */
	function requestDelete(index: number) {
		deleteIndex = index;
		confirmDeleteOpen = true;
	}

	/** Executes the pending deletion, removing the status and cleaning up references from other statuses. */
	function confirmDelete() {
		if (deleteIndex !== null) {
			const removed = localStatuses[deleteIndex]?.key;
			localStatuses = localStatuses
				.filter((_, i) => i !== deleteIndex)
				.map((s) => ({
					...s,
					transitions: (s.transitions ?? []).filter((k) => k !== removed),
					auto_rules: (s.auto_rules ?? []).filter((r) => r.target !== removed),
				}));
			deleteIndex = null;
			save();
		}
	}

	// Drag-and-drop reorder
	/**
	 * Records the dragged item's index at the start of a drag operation.
	 * @param index - The index of the status being dragged.
	 */
	function handleDragStart(index: number) {
		dragIndex = index;
	}

	/**
	 * Prevents default drag behavior and tracks the current drag-over index.
	 * @param e - The drag event to prevent default on.
	 * @param index - The index of the status being dragged over.
	 */
	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		dragOverIndex = index;
	}

	/**
	 * Reorders the statuses list by moving the dragged item to the drop target index.
	 * @param index - The index of the drop target status.
	 */
	function handleDrop(index: number) {
		if (dragIndex === null || dragIndex === index) {
			dragIndex = null;
			dragOverIndex = null;
			return;
		}
		const reordered = [...localStatuses];
		const [moved] = reordered.splice(dragIndex, 1);
		reordered.splice(index, 0, moved);
		localStatuses = reordered;
		dragIndex = null;
		dragOverIndex = null;
		save();
	}

	/** Clears drag state when the drag operation ends without a drop. */
	function handleDragEnd() {
		dragIndex = null;
		dragOverIndex = null;
	}
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Status Machine</CardTitle>
		<CardDescription>
			Define status values, icons, allowed transitions, and auto-progression rules. Drag to reorder.
		</CardDescription>
	</CardHeader>
	<CardContent>
		{#if localStatuses.length === 0}
			<span class="text-sm text-muted-foreground">No statuses defined. Add one below.</span>
		{:else}
			{#each localStatuses as status, index (status.key + index)}
				{@const isDragging = dragIndex === index}
				{@const isDragTarget = dragOverIndex === index && dragIndex !== null && dragIndex !== index}
				<div
					class="rounded-md border p-3 space-y-3 transition-opacity {isDragging ? 'opacity-40' : 'opacity-100'} {isDragTarget ? 'border-primary' : ''}"
					draggable="true"
					ondragstart={() => handleDragStart(index)}
					ondragover={(e) => handleDragOver(e, index)}
					ondrop={() => handleDrop(index)}
					ondragend={handleDragEnd}
					role="listitem"
				>
					<!-- Header row: drag handle, key, delete -->
					<div class="flex items-center gap-2">
						<Icon name="grip-vertical" size="md" />
						<span class="flex-1 font-mono text-xs font-semibold text-muted-foreground">{status.key}</span>
						<button
							class="flex h-7 items-center rounded px-2 text-muted-foreground hover:bg-accent hover:text-destructive"
							onclick={() => requestDelete(index)}
						>
							<Icon name="trash-2" size="sm" />
						</button>
					</div>

					<!-- Label + Icon + Spin -->
					<div class="grid grid-cols-[1fr_1fr_auto] gap-3 items-end">
						<FormGroup label="Label" for="s-label-{index}">
							<Input
								id="s-label-{index}"
								value={status.label}
								oninput={(e) => updateField(index, "label", e.currentTarget.value)}
								placeholder="Display label"
							/>
						</FormGroup>
						<FormGroup label="Icon" for="s-icon-{index}">
							<div class="flex items-center gap-2">
								<Input
									id="s-icon-{index}"
									value={status.icon}
									oninput={(e) => updateField(index, "icon", e.currentTarget.value)}
									placeholder="e.g. circle"
								/>
								<span class="shrink-0 text-base leading-none" aria-label="Icon preview">
									{#if status.icon}
										<span class="font-mono text-[10px] text-muted-foreground">{status.icon}</span>
									{/if}
								</span>
							</div>
						</FormGroup>
						<div class="flex items-center gap-1">
							<Switch
								checked={status.spin ?? false}
								size="sm"
								aria-label="Spin icon"
								onCheckedChange={(v) => updateField(index, "spin", v)}
							/>
							<span class="text-xs text-muted-foreground">Spin</span>
						</div>
					</div>

					<!-- Transitions -->
					<div class="space-y-1">
						<span class="text-xs text-muted-foreground">Allowed transitions</span>
						<div class="flex flex-wrap gap-1.5">
							{#each localStatuses.filter((s) => s.key !== status.key) as target (target.key)}
								{@const active = (status.transitions ?? []).includes(target.key)}
								<Button
									variant={active ? "default" : "outline"}
									size="sm"
									onclick={() => toggleTransition(index, target.key)}
								>
									{target.label || target.key}
								</Button>
							{/each}
							{#if localStatuses.filter((s) => s.key !== status.key).length === 0}
								<span class="text-xs text-muted-foreground">No other statuses yet</span>
							{/if}
						</div>
					</div>

					<!-- Auto rules -->
					<div class="space-y-1.5">
						<div class="flex items-center justify-between">
							<span class="text-xs text-muted-foreground">Auto-transition rules</span>
							<button
								class="flex items-center gap-1 rounded px-2 py-1 text-xs hover:bg-accent"
								onclick={() => addAutoRule(index)}
							>
								<Icon name="plus" size="xs" />
								Add rule
							</button>
						</div>
						{#if (status.auto_rules ?? []).length === 0}
							<span class="text-xs text-muted-foreground">No auto-transition rules.</span>
						{:else}
							<div class="space-y-1.5">
								{#each status.auto_rules ?? [] as rule, rIndex (rIndex)}
									<div class="flex items-center gap-2">
										<Input
											value={rule.condition}
											oninput={(e) => updateAutoRule(index, rIndex, "condition", e.currentTarget.value)}
											placeholder="condition"
										/>
										<span class="shrink-0 text-xs text-muted-foreground">→</span>
										<select
											class="flex h-7 flex-1 rounded-md border border-input bg-background px-2 py-0.5 text-xs ring-offset-background focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
											value={rule.target}
											onchange={(e) => updateAutoRule(index, rIndex, "target", e.currentTarget.value)}
										>
											<option value="">Select target</option>
											{#each localStatuses.filter((s) => s.key !== status.key) as target (target.key)}
												<option value={target.key}>{target.label || target.key}</option>
											{/each}
										</select>
										<button
											class="flex h-7 items-center rounded px-2 text-muted-foreground hover:bg-accent hover:text-destructive"
											onclick={() => removeAutoRule(index, rIndex)}
										>
											<Icon name="trash-2" size="sm" />
										</button>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>

				{#if index < localStatuses.length - 1}
					<Separator />
				{/if}
			{/each}
		{/if}

		<Button variant="outline" size="sm" onclick={addStatus}>
			<Icon name="plus" size="sm" />
			Add Status
		</Button>
	</CardContent>
</CardRoot>

<ConfirmDeleteDialog
	bind:open={confirmDeleteOpen}
	title="Delete status?"
	description="This removes the status and cleans up any transitions referencing it. Existing artifacts are not modified."
	onConfirm={confirmDelete}
/>

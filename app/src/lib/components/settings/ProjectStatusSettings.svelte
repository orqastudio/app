<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent, FormGroup } from "@orqastudio/svelte-components/pure";
	import { Button, HStack, Stack, Caption, SelectMenu } from "@orqastudio/svelte-components/pure";
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
			<Caption tone="muted">No statuses defined. Add one below.</Caption>
		{:else}
			{#each localStatuses as status, index (status.key + index)}
				{@const isDragging = dragIndex === index}
				{@const isDragTarget = dragOverIndex === index && dragIndex !== null && dragIndex !== index}
				<!-- Draggable container: native drag API requires raw div — not expressible via Box props -->
				<div
					class="rounded-md border p-3"
					draggable="true"
					ondragstart={() => handleDragStart(index)}
					ondragover={(e) => handleDragOver(e, index)}
					ondrop={() => handleDrop(index)}
					ondragend={handleDragEnd}
					role="listitem"
				>
					<Stack gap={3}>
						<!-- Header row: drag handle, key, delete -->
						<HStack gap={2}>
							<Icon name="grip-vertical" size="md" />
							<Caption variant="caption-mono" tone="muted">{status.key}</Caption>
							<Button
								variant="ghost"
								size="sm"
								onclick={() => requestDelete(index)}
							>
								<Icon name="trash-2" size="sm" />
							</Button>
						</HStack>

						<!-- Label + Icon + Spin -->
						<HStack gap={3} align="end">
							<FormGroup label="Label" for="s-label-{index}">
								<Input
									id="s-label-{index}"
									value={status.label}
									oninput={(e) => updateField(index, "label", e.currentTarget.value)}
									placeholder="Display label"
								/>
							</FormGroup>
							<FormGroup label="Icon" for="s-icon-{index}">
								<HStack gap={2}>
									<Input
										id="s-icon-{index}"
										value={status.icon}
										oninput={(e) => updateField(index, "icon", e.currentTarget.value)}
										placeholder="e.g. circle"
									/>
									{#if status.icon}
										<Caption variant="caption-mono" tone="muted">{status.icon}</Caption>
									{/if}
								</HStack>
							</FormGroup>
							<HStack gap={1}>
								<Switch
									checked={status.spin ?? false}
									size="sm"
									aria-label="Spin icon"
									onCheckedChange={(v) => updateField(index, "spin", v)}
								/>
								<Caption tone="muted">Spin</Caption>
							</HStack>
						</HStack>

						<!-- Transitions -->
						<Stack gap={1}>
							<Caption tone="muted">Allowed transitions</Caption>
							<HStack gap={1} wrap>
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
									<Caption tone="muted">No other statuses yet</Caption>
								{/if}
							</HStack>
						</Stack>

						<!-- Auto rules -->
						<Stack gap={1}>
							<HStack justify="between">
								<Caption tone="muted">Auto-transition rules</Caption>
								<Button
									variant="ghost"
									size="sm"
									onclick={() => addAutoRule(index)}
								>
									<Icon name="plus" size="xs" />
									Add rule
								</Button>
							</HStack>
							{#if (status.auto_rules ?? []).length === 0}
								<Caption tone="muted">No auto-transition rules.</Caption>
							{:else}
								<Stack gap={1}>
									{#each status.auto_rules ?? [] as rule, rIndex (rIndex)}
										<HStack gap={2}>
											<Input
												value={rule.condition}
												oninput={(e) => updateAutoRule(index, rIndex, "condition", e.currentTarget.value)}
												placeholder="condition"
											/>
											<Caption tone="muted">→</Caption>
											<SelectMenu
												items={[{ label: "Select target", value: "" }, ...localStatuses.filter((s) => s.key !== status.key).map((t) => ({ label: t.label || t.key, value: t.key }))]}
												selected={rule.target}
												onSelect={(v) => updateAutoRule(index, rIndex, "target", v)}
												triggerLabel={localStatuses.find((s) => s.key === rule.target)?.label || rule.target || "Select target"}
											/>
											<Button
												variant="ghost"
												size="sm"
												onclick={() => removeAutoRule(index, rIndex)}
											>
												<Icon name="trash-2" size="sm" />
											</Button>
										</HStack>
									{/each}
								</Stack>
							{/if}
						</Stack>
					</Stack>
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

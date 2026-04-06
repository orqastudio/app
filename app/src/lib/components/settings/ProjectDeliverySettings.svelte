<script lang="ts">
	import {
		Icon,
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
		FormGroup,
	} from "@orqastudio/svelte-components/pure";
	import {
		Button,
		Stack,
		HStack,
		Panel,
		Grid,
		Caption,
		SelectMenu,
	} from "@orqastudio/svelte-components/pure";
	import { Input } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { ConfirmDialog as ConfirmDeleteDialog } from "@orqastudio/svelte-components/pure";
	import type {
		ProjectSettings,
		DeliveryTypeConfig,
		DeliveryParentConfig,
	} from "@orqastudio/types";

	interface Props {
		settings: ProjectSettings;
		onSave: (settings: ProjectSettings) => void;
	}

	const props: Props = $props();

	// localTypes is a local edit buffer initialized from props.settings and mutated
	// by the update functions below. The $effect re-syncs when the prop changes
	// externally (e.g. undo or external save). $derived.by is inappropriate here
	// because the local state is intentionally mutated independently of the prop.
	// eslint-disable-next-line svelte/prefer-writable-derived
	let localTypes = $state<DeliveryTypeConfig[]>([]);
	let deleteIndex = $state<number | null>(null);
	let confirmDeleteOpen = $state(false);

	$effect(() => {
		localTypes = (props.settings.delivery?.types ?? []).map((t) => ({ ...t }));
	});

	/**
	 * Assemble the full project settings object from the current local delivery type buffer.
	 * @returns A new ProjectSettings value with the local delivery type list applied.
	 */
	function buildSettings(): ProjectSettings {
		return {
			...props.settings,
			delivery: {
				...props.settings.delivery,
				types: localTypes.map((t) => ({ ...t })),
			},
		};
	}

	/** Save the current local delivery type buffer to project settings via the onSave prop. */
	function save() {
		props.onSave(buildSettings());
	}

	/**
	 * Update a single string field on the delivery type at the given index and persist.
	 * @param index - The zero-based index of the delivery type to update in the local buffer.
	 * @param field - The key of the DeliveryTypeConfig field to update.
	 * @param value - The new string value to set for the field.
	 */
	function updateType(index: number, field: keyof DeliveryTypeConfig, value: string) {
		localTypes = localTypes.map((t, i) => (i === index ? { ...t, [field]: value } : t));
		save();
	}

	/**
	 * Update the parent artifact type for the delivery type at the given index and persist.
	 * @param index - The zero-based index of the delivery type to update.
	 * @param parentType - The new parent type key, or an empty string to remove the parent config.
	 */
	function updateParentType(index: number, parentType: string) {
		localTypes = localTypes.map((t, i) => {
			if (i !== index) return t;
			if (!parentType) {
				return { key: t.key, label: t.label, path: t.path };
			}
			const parent: DeliveryParentConfig = {
				type: parentType,
				relationship: t.parent?.relationship ?? "",
			};
			return { ...t, parent };
		});
		save();
	}

	/**
	 * Update the parent relationship type for the delivery type at the given index and persist.
	 * @param index - The zero-based index of the delivery type to update.
	 * @param parentRelationship - The relationship key linking this type to its parent (e.g. "delivers").
	 */
	function updateParentRelationship(index: number, parentRelationship: string) {
		localTypes = localTypes.map((t, i) => {
			if (i !== index) return t;
			const parent: DeliveryParentConfig = {
				type: t.parent?.type ?? "",
				relationship: parentRelationship,
			};
			return { ...t, parent };
		});
		save();
	}

	/**
	 * Update the gate field for the delivery type at the given index and persist.
	 * @param index - The zero-based index of the delivery type to update.
	 * @param gateField - The YAML field name used as a workflow gate, or an empty string to remove.
	 */
	function updateGateField(index: number, gateField: string) {
		localTypes = localTypes.map((t, i) => {
			if (i !== index) return t;
			return { ...t, gate_field: gateField || null };
		});
		save();
	}

	/**
	 *
	 */
	function addType() {
		const newType: DeliveryTypeConfig = {
			key: `type_${Date.now()}`,
			label: "New Type",
			path: ".orqa/delivery/new-type",
		};
		localTypes = [...localTypes, newType];
		save();
	}

	/**
	 * Open the delete confirmation dialog for the delivery type at the given index.
	 * @param index - The zero-based index of the delivery type the user wants to remove.
	 */
	function requestDelete(index: number) {
		deleteIndex = index;
		confirmDeleteOpen = true;
	}

	/** Confirm deletion of the pending delivery type and persist the updated list. */
	function confirmDelete() {
		if (deleteIndex !== null) {
			localTypes = localTypes.filter((_, i) => i !== deleteIndex);
			deleteIndex = null;
			save();
		}
	}

	const typeKeyOptions = $derived(
		localTypes.map((t) => ({ value: t.key, label: t.label || t.key })),
	);
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Delivery Pipeline</CardTitle>
		<CardDescription>Define the delivery types and hierarchy for this project</CardDescription>
	</CardHeader>
	<CardContent>
		{#if localTypes.length === 0}
			<Caption tone="muted">No delivery types defined. Add one below.</Caption>
		{:else}
			{#each localTypes as type, index (type.key + index)}
				<Panel padding="normal" rounded="md" border="all">
					<Stack gap={3}>
						<HStack justify="between">
							<Caption variant="caption-mono" tone="muted">{type.key}</Caption>
							<Button variant="ghost" size="sm" onclick={() => requestDelete(index)}>
								<Icon name="trash-2" size="sm" />
							</Button>
						</HStack>

						<Grid cols={2} gap={3}>
							<FormGroup label="Label" for="label-{index}">
								<Input
									id="label-{index}"
									value={type.label}
									oninput={(e) => updateType(index, "label", e.currentTarget.value)}
									placeholder="Display label"
								/>
							</FormGroup>
							<FormGroup label="Path" for="path-{index}">
								<Input
									id="path-{index}"
									value={type.path}
									oninput={(e) => updateType(index, "path", e.currentTarget.value)}
									placeholder=".orqa/delivery/..."
								/>
							</FormGroup>
						</Grid>

						<Grid cols={2} gap={3}>
							<FormGroup label="Parent type" for="parent-type-{index}">
								<SelectMenu
									items={[
										{ label: "None", value: "" },
										...typeKeyOptions.filter((o) => o.value !== type.key),
									]}
									selected={type.parent?.type ?? ""}
									onSelect={(v) => updateParentType(index, v)}
									triggerLabel={typeKeyOptions.find((o) => o.value === (type.parent?.type ?? ""))
										?.label ?? "None"}
								/>
							</FormGroup>
							<FormGroup label="Parent relationship" for="parent-rel-{index}">
								<Input
									id="parent-rel-{index}"
									value={type.parent?.relationship ?? ""}
									oninput={(e) => updateParentRelationship(index, e.currentTarget.value)}
									disabled={!type.parent?.type}
									placeholder="e.g. delivers"
								/>
							</FormGroup>
						</Grid>

						<FormGroup label="Gate field (optional)" for="gate-{index}">
							<Input
								id="gate-{index}"
								value={type.gate_field ?? ""}
								oninput={(e) => updateGateField(index, e.currentTarget.value)}
								placeholder="e.g. gate"
							/>
						</FormGroup>
					</Stack>
				</Panel>

				{#if index < localTypes.length - 1}
					<Separator />
				{/if}
			{/each}
		{/if}

		<Button variant="outline" size="sm" onclick={addType}>
			<Icon name="plus" size="sm" />
			Add Delivery Type
		</Button>
	</CardContent>
</CardRoot>

<ConfirmDeleteDialog
	bind:open={confirmDeleteOpen}
	title="Delete delivery type?"
	description="This removes the type from the pipeline configuration. Existing artifacts on disk are not affected."
	onConfirm={confirmDelete}
/>

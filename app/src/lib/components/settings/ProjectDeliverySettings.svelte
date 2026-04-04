<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent, FormGroup } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Input } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { ConfirmDialog as ConfirmDeleteDialog } from "@orqastudio/svelte-components/pure";
	import type { ProjectSettings, DeliveryTypeConfig, DeliveryParentConfig } from "@orqastudio/types";

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

	function buildSettings(): ProjectSettings {
		return {
			...props.settings,
			delivery: {
				...props.settings.delivery,
				types: localTypes.map((t) => ({ ...t })),
			},
		};
	}

	function save() {
		props.onSave(buildSettings());
	}

	function updateType(index: number, field: keyof DeliveryTypeConfig, value: string) {
		localTypes = localTypes.map((t, i) => (i === index ? { ...t, [field]: value } : t));
		save();
	}

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

	function updateGateField(index: number, gateField: string) {
		localTypes = localTypes.map((t, i) => {
			if (i !== index) return t;
			return { ...t, gate_field: gateField || null };
		});
		save();
	}

	function addType() {
		const newType: DeliveryTypeConfig = {
			key: `type_${Date.now()}`,
			label: "New Type",
			path: ".orqa/delivery/new-type",
		};
		localTypes = [...localTypes, newType];
		save();
	}

	function requestDelete(index: number) {
		deleteIndex = index;
		confirmDeleteOpen = true;
	}

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
			<span class="text-sm text-muted-foreground">No delivery types defined. Add one below.</span>
		{:else}
			{#each localTypes as type, index (type.key + index)}
				<div class="rounded-md border p-3 space-y-3">
					<div class="flex items-center justify-between">
						<span class="font-mono text-xs font-semibold text-muted-foreground">{type.key}</span>
						<button
							class="flex h-7 items-center rounded px-2 text-muted-foreground hover:bg-accent hover:text-destructive"
							onclick={() => requestDelete(index)}
						>
							<Icon name="trash-2" size="sm" />
						</button>
					</div>

					<div class="grid grid-cols-2 gap-3">
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
					</div>

					<div class="grid grid-cols-2 gap-3">
						<FormGroup label="Parent type" for="parent-type-{index}">
							<select
								id="parent-type-{index}"
								class="flex h-7 w-full rounded-md border border-input bg-background px-2 py-0.5 text-xs ring-offset-background focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
								value={type.parent?.type ?? ""}
								onchange={(e) => updateParentType(index, e.currentTarget.value)}
							>
								<option value="">None</option>
								{#each typeKeyOptions.filter((o) => o.value !== type.key) as opt (opt.value)}
									<option value={opt.value}>{opt.label}</option>
								{/each}
							</select>
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
					</div>

					<FormGroup label="Gate field (optional)" for="gate-{index}">
						<Input
							id="gate-{index}"
							value={type.gate_field ?? ""}
							oninput={(e) => updateGateField(index, e.currentTarget.value)}
							placeholder="e.g. gate"
						/>
					</FormGroup>
				</div>

				{#if index < localTypes.length - 1}
					<Separator />
				{/if}
			{/each}
		{/if}

		<button class="flex w-full items-center justify-center gap-1 rounded border border-border px-3 py-1.5 text-sm hover:bg-accent" onclick={addType}>
			<Icon name="plus" size="sm" />
			Add Delivery Type
		</button>
	</CardContent>
</CardRoot>

<ConfirmDeleteDialog
	bind:open={confirmDeleteOpen}
	title="Delete delivery type?"
	description="This removes the type from the pipeline configuration. Existing artifacts on disk are not affected."
	onConfirm={confirmDelete}
/>

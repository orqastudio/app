<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
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
	<CardContent class="space-y-4">
		{#if localTypes.length === 0}
			<p class="text-sm text-muted-foreground">No delivery types defined. Add one below.</p>
		{:else}
			{#each localTypes as type, index (type.key + index)}
				<div class="rounded-md border p-3 space-y-3">
					<div class="flex items-center justify-between">
						<span class="font-mono text-xs font-semibold text-muted-foreground">{type.key}</span>
						<Button
							variant="ghost"
							size="sm"
							class="h-7 px-2 text-muted-foreground hover:text-destructive"
							onclick={() => requestDelete(index)}
						>
							<Icon name="trash-2" size="sm" />
						</Button>
					</div>

					<div class="grid grid-cols-2 gap-3">
						<div class="space-y-1">
							<label class="text-xs font-medium text-muted-foreground" for="label-{index}">
								Label
							</label>
							<Input
								id="label-{index}"
								value={type.label}
								oninput={(e) => updateType(index, "label", e.currentTarget.value)}
								class="h-7 text-xs"
								placeholder="Display label"
							/>
						</div>
						<div class="space-y-1">
							<label class="text-xs font-medium text-muted-foreground" for="path-{index}">
								Path
							</label>
							<Input
								id="path-{index}"
								value={type.path}
								oninput={(e) => updateType(index, "path", e.currentTarget.value)}
								class="h-7 font-mono text-xs"
								placeholder=".orqa/delivery/..."
							/>
						</div>
					</div>

					<div class="grid grid-cols-2 gap-3">
						<div class="space-y-1">
							<label class="text-xs font-medium text-muted-foreground" for="parent-type-{index}">
								Parent type
							</label>
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
						</div>
						<div class="space-y-1">
							<label class="text-xs font-medium text-muted-foreground" for="parent-rel-{index}">
								Parent relationship
							</label>
							<Input
								id="parent-rel-{index}"
								value={type.parent?.relationship ?? ""}
								oninput={(e) => updateParentRelationship(index, e.currentTarget.value)}
								disabled={!type.parent?.type}
								class="h-7 font-mono text-xs"
								placeholder="e.g. delivers"
							/>
						</div>
					</div>

					<div class="space-y-1">
						<label class="text-xs font-medium text-muted-foreground" for="gate-{index}">
							Gate field <span class="text-muted-foreground/60">(optional)</span>
						</label>
						<Input
							id="gate-{index}"
							value={type.gate_field ?? ""}
							oninput={(e) => updateGateField(index, e.currentTarget.value)}
							class="h-7 font-mono text-xs"
							placeholder="e.g. gate"
						/>
					</div>
				</div>

				{#if index < localTypes.length - 1}
					<Separator />
				{/if}
			{/each}
		{/if}

		<Button variant="outline" size="sm" onclick={addType} class="w-full">
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

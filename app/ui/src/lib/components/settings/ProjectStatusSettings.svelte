<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, CardAction } from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Input } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { ConfirmDialog as ConfirmDeleteDialog } from "@orqastudio/svelte-components/pure";
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

	function buildSettings(): ProjectSettings {
		return {
			...props.settings,
			statuses: localStatuses.map((s) => ({ ...s })),
		};
	}

	function save() {
		props.onSave(buildSettings());
	}

	function updateField(index: number, field: keyof StatusDefinition, value: string | boolean) {
		localStatuses = localStatuses.map((s, i) => (i === index ? { ...s, [field]: value } : s));
		save();
	}

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

	function addAutoRule(index: number) {
		localStatuses = localStatuses.map((s, i) => {
			if (i !== index) return s;
			const auto_rules = [...(s.auto_rules ?? []), { condition: "", target: "" }];
			return { ...s, auto_rules };
		});
	}

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

	function removeAutoRule(statusIndex: number, ruleIndex: number) {
		localStatuses = localStatuses.map((s, i) => {
			if (i !== statusIndex) return s;
			const auto_rules = (s.auto_rules ?? []).filter((_, ri) => ri !== ruleIndex);
			return { ...s, auto_rules };
		});
		save();
	}

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

	function requestDelete(index: number) {
		deleteIndex = index;
		confirmDeleteOpen = true;
	}

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
	function handleDragStart(index: number) {
		dragIndex = index;
	}

	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		dragOverIndex = index;
	}

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
	<CardContent class="space-y-4">
		{#if localStatuses.length === 0}
			<p class="text-sm text-muted-foreground">No statuses defined. Add one below.</p>
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
						<Button
							variant="ghost"
							size="sm"
							class="h-7 px-2 text-muted-foreground hover:text-destructive"
							onclick={() => requestDelete(index)}
						>
							<Icon name="trash-2" size="sm" />
						</Button>
					</div>

					<!-- Label + Icon + Spin -->
					<div class="grid grid-cols-[1fr_1fr_auto] gap-3 items-end">
						<div class="space-y-1">
							<label class="text-xs font-medium text-muted-foreground" for="s-label-{index}">
								Label
							</label>
							<Input
								id="s-label-{index}"
								value={status.label}
								oninput={(e) => updateField(index, "label", e.currentTarget.value)}
								class="h-7 text-xs"
								placeholder="Display label"
							/>
						</div>
						<div class="space-y-1">
							<label class="text-xs font-medium text-muted-foreground" for="s-icon-{index}">
								Icon
							</label>
							<div class="flex items-center gap-2">
								<Input
									id="s-icon-{index}"
									value={status.icon}
									oninput={(e) => updateField(index, "icon", e.currentTarget.value)}
									class="h-7 font-mono text-xs"
									placeholder="e.g. circle"
								/>
								<span class="shrink-0 text-base leading-none" title="Icon preview (name only)">
									{#if status.icon}
										<span class="font-mono text-[10px] text-muted-foreground">{status.icon}</span>
									{/if}
								</span>
							</div>
						</div>
						<div class="flex items-center gap-1.5 pb-0.5">
							<button
								class="relative inline-flex h-5 w-9 shrink-0 items-center rounded-full transition-colors {status.spin ? 'bg-primary' : 'bg-muted-foreground/30'}"
								onclick={() => updateField(index, "spin", !status.spin)}
								role="switch"
								aria-checked={status.spin ?? false}
								aria-label="Spin icon"
							>
								<span
									class="inline-block h-4 w-4 transform rounded-full bg-background shadow-sm transition-transform {status.spin ? 'translate-x-4' : 'translate-x-0.5'}"
								></span>
							</button>
							<span class="text-xs text-muted-foreground">Spin</span>
						</div>
					</div>

					<!-- Transitions -->
					<div class="space-y-1">
						<span class="text-xs font-medium text-muted-foreground">Allowed transitions</span>
						<div class="flex flex-wrap gap-1.5">
							{#each localStatuses.filter((s) => s.key !== status.key) as target (target.key)}
								{@const active = (status.transitions ?? []).includes(target.key)}
								<button
									class="rounded border px-2 py-0.5 text-xs transition-colors {active
										? 'border-primary bg-primary text-primary-foreground'
										: 'border-border bg-background text-muted-foreground hover:bg-accent/50'}"
									onclick={() => toggleTransition(index, target.key)}
								>
									{target.label || target.key}
								</button>
							{/each}
							{#if localStatuses.filter((s) => s.key !== status.key).length === 0}
								<span class="text-xs text-muted-foreground">No other statuses yet</span>
							{/if}
						</div>
					</div>

					<!-- Auto rules -->
					<div class="space-y-1.5">
						<div class="flex items-center justify-between">
							<span class="text-xs font-medium text-muted-foreground">Auto-transition rules</span>
							<Button
								variant="ghost"
								size="sm"
								class="h-6 px-2 text-xs"
								onclick={() => addAutoRule(index)}
							>
								<Icon name="plus" size="xs" />
								Add rule
							</Button>
						</div>
						{#if (status.auto_rules ?? []).length === 0}
							<p class="text-xs text-muted-foreground">No auto-transition rules.</p>
						{:else}
							<div class="space-y-1.5">
								{#each status.auto_rules ?? [] as rule, rIndex (rIndex)}
									<div class="flex items-center gap-2">
										<Input
											value={rule.condition}
											oninput={(e) => updateAutoRule(index, rIndex, "condition", e.currentTarget.value)}
											class="h-7 flex-1 font-mono text-xs"
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
										<Button
											variant="ghost"
											size="sm"
											class="h-7 px-1.5 text-muted-foreground hover:text-destructive"
											onclick={() => removeAutoRule(index, rIndex)}
										>
											<Icon name="trash-2" size="sm" />
										</Button>
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

		<Button variant="outline" size="sm" onclick={addStatus} class="w-full">
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

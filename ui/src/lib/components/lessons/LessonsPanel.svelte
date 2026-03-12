<script lang="ts">
	import { onMount } from "svelte";
	import LessonList from "./LessonList.svelte";
	import LessonViewer from "./LessonViewer.svelte";
	import { lessonStore } from "$lib/stores/lessons.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import type { Lesson } from "$lib/types/lessons";

	let selectedLesson = $state<Lesson | null>(null);

	const projectPath = $derived(projectStore.projectPath);

	onMount(() => {
		if (projectPath) {
			lessonStore.loadLessons(projectPath);
		}
	});

	function handleSelect(lesson: Lesson) {
		selectedLesson = lesson;
	}

	async function handleIncrementRecurrence(id: string) {
		if (!projectPath) return;
		await lessonStore.incrementRecurrence(projectPath, id);
		// Refresh the selected lesson state from the updated store
		const updated = lessonStore.lessons.find((l) => l.id === id);
		if (updated) {
			selectedLesson = updated;
		}
	}

	function handleRetry() {
		if (projectPath) {
			lessonStore.loadLessons(projectPath);
		}
	}
</script>

<div class="flex h-full">
	<!-- Lesson list sidebar (240px) -->
	<div class="w-60 shrink-0 overflow-hidden border-r border-border">
		<LessonList
			lessons={lessonStore.lessons}
			loading={lessonStore.loading}
			error={lessonStore.error}
			selectedId={selectedLesson?.id ?? null}
			onSelect={handleSelect}
			onRetry={handleRetry}
		/>
	</div>

	<!-- Lesson viewer -->
	<div class="min-w-0 flex-1 overflow-hidden">
		{#if selectedLesson}
			<LessonViewer lesson={selectedLesson} onIncrementRecurrence={handleIncrementRecurrence} />
		{:else}
			<div class="flex h-full items-center justify-center text-sm text-muted-foreground">
				Select a lesson to view it
			</div>
		{/if}
	</div>
</div>

import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import type { Lesson } from "@orqastudio/types";

/**
 * Reactive store managing lessons loaded from the backend, including creation and recurrence tracking.
 */
export class LessonStore {
	lessons = $state<Lesson[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	/**
	 * Lessons with recurrence >= 2 and status "active" — ready for promotion.
	 * @returns The list of lessons eligible for promotion to rules.
	 */
	get promotionCandidates(): Lesson[] {
		return this.lessons.filter((l) => l.recurrence >= 2 && l.status === "active");
	}

	/**
	 * Loads all lessons for the given project from the backend and updates reactive state.
	 * @param projectPath - Absolute path to the project root.
	 */
	async loadLessons(projectPath: string): Promise<void> {
		this.loading = true;
		this.error = null;
		try {
			this.lessons = await invoke<Lesson[]>("lessons_list", { projectPath });
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	/**
	 * Creates a new lesson in the backend and appends it to the local lessons list.
	 * @param projectPath - Absolute path to the project root.
	 * @param title - Short title summarising the lesson learned.
	 * @param category - Category tag used to group related lessons.
	 * @param body - Full Markdown body describing the lesson in detail.
	 */
	async createLesson(
		projectPath: string,
		title: string,
		category: string,
		body: string,
	): Promise<void> {
		this.error = null;
		try {
			const lesson = await invoke<Lesson>("lessons_create", {
				projectPath,
				title,
				category,
				body,
			});
			this.lessons = [...this.lessons, lesson].sort((a, b) => a.id.localeCompare(b.id));
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	/**
	 * Increments the recurrence counter on a lesson, signalling it has been observed again.
	 * @param projectPath - Absolute path to the project root.
	 * @param id - The unique identifier of the lesson to update.
	 */
	async incrementRecurrence(projectPath: string, id: string): Promise<void> {
		this.error = null;
		try {
			const updated = await invoke<Lesson>("lesson_increment_recurrence", { projectPath, id });
			this.lessons = this.lessons.map((l) => (l.id === id ? updated : l));
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}
}

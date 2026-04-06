import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import type { Lesson } from "@orqastudio/types";

/**
 *
 */
export class LessonStore {
	lessons = $state<Lesson[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);

	/** Lessons with recurrence >= 2 and status "active" — ready for promotion. */
	get promotionCandidates(): Lesson[] {
		return this.lessons.filter((l) => l.recurrence >= 2 && l.status === "active");
	}

	/**
	 *
	 * @param projectPath
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
	 *
	 * @param projectPath
	 * @param title
	 * @param category
	 * @param body
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
	 *
	 * @param projectPath
	 * @param id
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

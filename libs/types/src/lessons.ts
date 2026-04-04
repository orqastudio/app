export type LessonStatus = "active" | "promoted" | "resolved";
export type LessonCategory = "process" | "coding" | "architecture";

export interface Lesson {
	readonly id: string;
	readonly title: string;
	readonly category: string;
	readonly recurrence: number;
	readonly status: LessonStatus;
	readonly promoted_to: string | null;
	readonly created: string;
	readonly updated: string;
	readonly body: string;
	readonly file_path: string;
}

export interface NewLesson {
	readonly title: string;
	readonly category: LessonCategory;
	readonly body: string;
}

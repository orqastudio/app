// Lesson pipeline stage configuration for LessonVelocityWidget.
// Defines the ordered stages a lesson moves through, with display labels and
// dot colours matching the PipelineStages visual component.

export interface LessonStageConfig {
	key: string;
	label: string;
	dotColorClass: string;
}

// Ordered stage definitions — active through promoted mirrors the lesson lifecycle.
export const LESSON_STAGES: readonly LessonStageConfig[] = [
	{ key: "active",    label: "Active",    dotColorClass: "bg-blue-500"   },
	{ key: "recurring", label: "Recurring", dotColorClass: "bg-amber-500"  },
	{ key: "promoted",  label: "Promoted",  dotColorClass: "bg-purple-500" },
] as const;

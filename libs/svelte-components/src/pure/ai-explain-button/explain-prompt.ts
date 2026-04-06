// Builds AI explanation prompts from log event context. Used by the AiExplainButton
// component to produce a structured prompt string that can be sent to an AI assistant.

import type { StackFrame } from "../stack-frame-list/StackFrameList.svelte";

/** Minimal event shape required to build an explain prompt. */
export interface ExplainEvent {
	message: string;
	level: string;
	source: string;
	stack_frames?: StackFrame[];
	correlation_id?: string;
	metadata?: unknown;
}

/**
 * Build an AI explanation prompt from event context.
 * Formats a structured prompt with the error message, level, source, stack trace
 * (top 5 frames), and metadata context for submission to an AI assistant.
 * @param event - The event to explain.
 * @returns A formatted prompt string for the AI.
 */
export function buildExplainPrompt(event: ExplainEvent): string {
	const lines: string[] = [
		`Explain this error from the OrqaStudio ${event.source} component:`,
		"",
		`**Error:** ${event.message}`,
		`**Level:** ${event.level}`,
	];

	const frames = event.stack_frames?.slice(0, 5) ?? [];
	if (frames.length > 0) {
		lines.push("", "**Stack trace (top 5 frames):**");
		frames.forEach((frame, i) => {
			const location = frame.line != null ? `${frame.file}:${frame.line}` : frame.file;
			const fn = frame.function ?? frame.raw ?? "(anonymous)";
			lines.push(`${i + 1}. ${fn} at ${location}`);
		});
	}

	if (event.metadata !== undefined && event.metadata !== null) {
		lines.push("", `**Context:** ${JSON.stringify(event.metadata)}`);
	}

	lines.push("", "What is the likely root cause and how should it be fixed?");

	return lines.join("\n");
}

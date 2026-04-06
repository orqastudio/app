import type { Meta, StoryObj } from "@storybook/svelte";
import AiExplainButton from "./AiExplainButton.svelte";

const meta = {
	title: "Pure/AiExplainButton",
	component: AiExplainButton,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

/** Mock event with full context including stack frames and metadata. */
const fullEvent = {
	message: "Graph traversal failed: circular dependency detected in workflow",
	level: "Error",
	source: "Daemon",
	stack_frames: [
		{ file: "src/core/graph-engine.ts", line: 142, col: 18, function: "GraphEngine.traverse" },
		{
			file: "src/plugins/workflow-plugin.ts",
			line: 87,
			col: 5,
			function: "WorkflowPlugin.execute",
		},
		{ file: "src/runtime/scheduler.ts", line: 223, col: 12, function: "Scheduler.runTask" },
		{ file: "src/runtime/event-loop.ts", line: 55, col: 3, function: "EventLoop.tick" },
		{ file: "src/main.ts", line: 12, col: 1, function: "bootstrap" },
	],
	correlation_id: "corr-abc123",
	metadata: { workflow_id: "wf-001", plugin: "software" },
};

/** Button with a full event — ready to build and emit a prompt. */
export const WithEvent: Story = {
	args: {
		event: fullEvent,
	},
};

/** Button with no stack frames — prompt omits the stack section. */
export const NoStackFrames: Story = {
	args: {
		event: {
			message: "Connection timeout",
			level: "Warn",
			source: "Connector",
		},
	},
};

/** Button with no event selected — disabled state. */
export const NoEvent: Story = {
	args: {
		event: null,
	},
};

/** Button explicitly disabled via the disabled prop. */
export const ExplicitlyDisabled: Story = {
	args: {
		event: fullEvent,
		disabled: true,
	},
};

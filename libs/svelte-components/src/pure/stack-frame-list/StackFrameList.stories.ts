import type { Meta, StoryObj } from "@storybook/svelte";
import StackFrameList from "./StackFrameList.svelte";

const meta = {
	title: "Pure/StackFrameList",
	component: StackFrameList,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

/** Fully resolved 5-frame stack trace from a typical application error. */
export const FiveFrames: Story = {
	args: {
		frames: [
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
	},
};

/** Single top-level frame — common when a throw happens at the entry point. */
export const SingleFrame: Story = {
	args: {
		frames: [{ file: "src/auth/session.ts", line: 34, col: 9, function: "Session.validate" }],
	},
};

/** Empty frames array renders the empty-state caption. */
export const NoFrames: Story = {
	args: {
		frames: [],
	},
};

/** Frame that contains only a raw string and no structured fields. */
export const RawStringOnly: Story = {
	args: {
		frames: [
			{
				file: "unknown",
				raw: "at Object.<anonymous> (webpack:///./src/index.js:1:0)",
			},
		],
	},
};

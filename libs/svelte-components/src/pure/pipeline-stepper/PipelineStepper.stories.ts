import type { Meta, StoryObj } from "@storybook/svelte";
import PipelineStepper from "./PipelineStepper.svelte";

const meta: Meta<typeof PipelineStepper> = {
	title: "Pure/PipelineStepper",
	component: PipelineStepper,
	tags: ["autodocs"],
	argTypes: {
		status: { control: "text" },
		transitioning: { control: "boolean" },
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

const stages = [
	{ key: "draft", label: "Draft" },
	{ key: "in-progress", label: "In Progress" },
	{ key: "review", label: "Review" },
	{ key: "done", label: "Done" },
];

export const Active: Story = {
	args: {
		stages,
		status: "in-progress",
		reachableKeys: ["review"],
	},
};

export const Complete: Story = {
	args: {
		stages,
		status: "done",
		reachableKeys: [],
	},
};

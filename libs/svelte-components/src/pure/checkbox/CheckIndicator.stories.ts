import type { Meta, StoryObj } from "@storybook/svelte";
import CheckIndicator from "./CheckIndicator.svelte";

const meta = {
	title: "Pure/CheckIndicator",
	component: CheckIndicator,
	tags: ["autodocs"],
	argTypes: {
		checked: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Unchecked: Story = {
	args: { checked: false },
};

export const Checked: Story = {
	args: { checked: true },
};

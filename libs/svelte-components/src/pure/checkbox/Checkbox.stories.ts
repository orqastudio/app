import type { Meta, StoryObj } from "@storybook/svelte";
import Checkbox from "./checkbox.svelte";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const meta: Meta<any> = {
	title: "Pure/Checkbox",
	component: Checkbox,
	tags: ["autodocs"],
	argTypes: {
		checked: { control: "boolean" },
		disabled: { control: "boolean" },
	},
};

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		checked: false,
		disabled: false,
	},
};

export const Checked: Story = {
	args: {
		checked: true,
		disabled: false,
	},
};

export const Disabled: Story = {
	args: {
		checked: false,
		disabled: true,
	},
};

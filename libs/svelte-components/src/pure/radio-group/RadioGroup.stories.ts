import type { Meta, StoryObj } from "@storybook/svelte";
import RadioGroup from "./radio-group.svelte";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const meta: Meta<any> = {
	title: "Pure/RadioGroup",
	component: RadioGroup,
	tags: ["autodocs"],
	argTypes: {
		value: { control: "text" },
	},
};

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		value: "option1",
	},
};

export const Disabled: Story = {
	args: {
		value: "option1",
		disabled: true,
	},
};

import type { Meta, StoryObj } from "@storybook/svelte";
import Switch from "./switch.svelte";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const meta: Meta<any> = {
	title: "Pure/Switch",
	component: Switch,
	tags: ["autodocs"],
	argTypes: {
		checked: { control: "boolean" },
		disabled: { control: "boolean" },
		size: {
			control: "select",
			options: ["default", "sm"],
		},
	},
};

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		checked: false,
		disabled: false,
		size: "default",
	},
};

export const Checked: Story = {
	args: {
		checked: true,
		disabled: false,
		size: "default",
	},
};

export const Disabled: Story = {
	args: {
		checked: false,
		disabled: true,
		size: "default",
	},
};

export const Small: Story = {
	args: {
		checked: false,
		disabled: false,
		size: "sm",
	},
};

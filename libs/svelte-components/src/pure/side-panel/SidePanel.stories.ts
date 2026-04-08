import type { Meta, StoryObj } from "@storybook/svelte";
import SidePanel from "./SidePanel.svelte";

const meta: Meta<typeof SidePanel> = {
	title: "Pure/SidePanel",
	component: SidePanel,
	tags: ["autodocs"],
	argTypes: {
		width: {
			control: "select",
			options: ["xs", "sm", "md", "lg"],
		},
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		width: "md",
	},
};

export const Narrow: Story = {
	args: {
		width: "xs",
	},
};

export const Wide: Story = {
	args: {
		width: "lg",
	},
};

import type { Meta, StoryObj } from "@storybook/svelte";
import Sidebar from "./Sidebar.svelte";

const meta: Meta<typeof Sidebar> = {
	title: "Pure/Sidebar",
	component: Sidebar,
	tags: ["autodocs"],
	argTypes: {
		width: { control: "select", options: ["sm", "md", "lg", "xl"] },
		border: { control: "select", options: ["right", "left", "none"] },
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: { width: "md", border: "right" },
};

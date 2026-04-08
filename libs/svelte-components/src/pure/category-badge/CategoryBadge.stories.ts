import type { Meta, StoryObj } from "@storybook/svelte";
import CategoryBadge from "./CategoryBadge.svelte";

const meta: Meta<typeof CategoryBadge> = {
	title: "Pure/CategoryBadge",
	component: CategoryBadge,
	tags: ["autodocs"],
	argTypes: {
		category: { control: "text" },
		color: { control: "color" },
	},
};

export default meta;
type Story = StoryObj<typeof meta>;

export const WithColor: Story = {
	args: { category: "process", color: "#3b82f6" },
};

export const WithoutColor: Story = {
	args: { category: "tooling" },
};

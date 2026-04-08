import type { Meta, StoryObj } from "@storybook/svelte";
import CollapsibleGroupHeader from "./CollapsibleGroupHeader.svelte";

const meta = {
	title: "Pure/Collapsible/CollapsibleGroupHeader",
	component: CollapsibleGroupHeader,
	tags: ["autodocs"],
} satisfies Meta<typeof CollapsibleGroupHeader>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: { label: "Tasks" },
};

export const WithCount: Story = {
	args: { label: "Tasks", count: 12 },
};

export const WithDepth: Story = {
	args: { label: "Nested Group", count: 5, depth: 2 },
};

export const LongLabel: Story = {
	args: { label: "Architecture Decisions", count: 34 },
};

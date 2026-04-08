import type { Meta, StoryObj } from "@storybook/svelte";
import TreeCollapsibleTrigger from "./TreeCollapsibleTrigger.svelte";

const meta = {
	title: "Pure/TreeCollapsibleTrigger",
	component: TreeCollapsibleTrigger,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Depth0: Story = { args: { depth: 0 } };
export const Depth1: Story = { args: { depth: 1 } };
export const Depth2: Story = { args: { depth: 2 } };
export const Depth4: Story = { args: { depth: 4 } };

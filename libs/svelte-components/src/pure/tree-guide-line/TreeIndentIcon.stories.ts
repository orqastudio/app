import type { Meta, StoryObj } from "@storybook/svelte";
import TreeIndentIcon from "./TreeIndentIcon.svelte";

const meta = {
	title: "Pure/TreeIndentIcon",
	component: TreeIndentIcon,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Depth0: Story = { args: { name: "corner-down-right", depth: 0 } };
export const Depth1: Story = { args: { name: "corner-down-right", depth: 1 } };
export const Depth2: Story = { args: { name: "corner-down-right", depth: 2 } };
export const GitBranch: Story = { args: { name: "git-branch", depth: 1 } };

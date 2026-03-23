import type { Meta, StoryObj } from "@storybook/svelte";
import NavSubPanel from "./NavSubPanel.svelte";

const meta = {
	title: "Connected/NavSubPanel",
	component: NavSubPanel,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { title: "Delivery", width: 200 },
};

export const Wide: Story = {
	args: { title: "Project Settings", width: 280 },
};

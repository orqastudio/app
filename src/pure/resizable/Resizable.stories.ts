import type { Meta, StoryObj } from "@storybook/svelte";
import Resizable from "./SimpleResizable.svelte";

const meta = {
	title: "Pure/Resizable",
	component: Resizable,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const HorizontalSplit: Story = {
	args: {
		direction: "horizontal",
		mainSize: 70,
		sideSize: 30,
	},
};

export const VerticalSplit: Story = {
	args: {
		direction: "vertical",
		mainSize: 60,
		sideSize: 40,
	},
};

export const EqualSplit: Story = {
	args: {
		direction: "horizontal",
		mainSize: 50,
		sideSize: 50,
		mainMinSize: 20,
		sideMinSize: 20,
	},
};

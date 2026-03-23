import type { Meta, StoryObj } from "@storybook/svelte";
import ActivityBar from "./ActivityBar.svelte";

const meta = {
	title: "Connected/ActivityBar",
	component: ActivityBar,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		topItems: [],
		bottomItems: [],
	},
};

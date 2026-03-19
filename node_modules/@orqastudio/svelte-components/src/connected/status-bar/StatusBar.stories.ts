import type { Meta, StoryObj } from "@storybook/svelte";
import StatusBar from "./StatusBar.svelte";

const meta = {
	title: "Connected/StatusBar",
	component: StatusBar,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: {} };

import type { Meta, StoryObj } from "@storybook/svelte";
import Toolbar from "./Toolbar.svelte";

const meta = {
	title: "Connected/Toolbar",
	component: Toolbar,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: {} };

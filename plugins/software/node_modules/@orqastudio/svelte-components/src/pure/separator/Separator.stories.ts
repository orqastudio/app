import type { Meta, StoryObj } from "@storybook/svelte";
import { Separator } from "./index.js";

const meta = {
	title: "Pure/Separator",
	component: Separator,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Horizontal: Story = { args: {} };

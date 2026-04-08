import type { Meta, StoryObj } from "@storybook/svelte";
import StatCard from "./StatCard.svelte";

const meta = {
	title: "Pure/StatCard",
	component: StatCard,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {},
};

import type { Meta, StoryObj } from "@storybook/svelte";
import Table from "./Table.svelte";

const meta = {
	title: "Pure/Table",
	component: Table,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {};

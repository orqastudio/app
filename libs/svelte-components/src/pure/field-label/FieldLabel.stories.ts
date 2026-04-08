import type { Meta, StoryObj } from "@storybook/svelte";
import FieldLabel from "./FieldLabel.svelte";

const meta: Meta<typeof FieldLabel> = {
	title: "Pure/FieldLabel",
	component: FieldLabel,
	tags: ["autodocs"],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

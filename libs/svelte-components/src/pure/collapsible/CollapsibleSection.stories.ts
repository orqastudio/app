import type { Meta, StoryObj } from "@storybook/svelte";
import CollapsibleSection from "./CollapsibleSection.svelte";

const meta = {
	title: "Pure/Collapsible/CollapsibleSection",
	component: CollapsibleSection,
	tags: ["autodocs"],
} satisfies Meta<typeof CollapsibleSection>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {},
};

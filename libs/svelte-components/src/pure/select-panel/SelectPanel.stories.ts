import type { Meta, StoryObj } from "@storybook/svelte";
import SelectPanel from "./SelectPanel.svelte";

const meta: Meta<typeof SelectPanel> = {
	title: "Pure/SelectPanel",
	component: SelectPanel,
	tags: ["autodocs"],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = { args: { role: "listbox", "aria-label": "Options" } };

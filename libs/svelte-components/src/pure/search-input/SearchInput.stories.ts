import type { Meta, StoryObj } from "@storybook/svelte";
import SearchInput from "./SearchInput.svelte";

const meta = {
	title: "Pure/SearchInput",
	component: SearchInput,
	tags: ["autodocs"],
	argTypes: {
		size: { control: "select", options: ["sm", "xs"] },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { placeholder: "Search artifacts..." } };
export const ExtraSmall: Story = { args: { size: "xs", placeholder: "Filter..." } };

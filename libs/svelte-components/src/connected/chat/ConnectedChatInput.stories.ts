import type { Meta, StoryObj } from "@storybook/svelte";
import ConnectedChatInput from "./ConnectedChatInput.svelte";

const meta = {
	title: "Connected/ConnectedChatInput",
	component: ConnectedChatInput,
	tags: ["autodocs"],
	argTypes: {
		placeholder: { control: "text" },
		disabled: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { placeholder: "Type a message..." },
};

export const Disabled: Story = {
	args: { placeholder: "Connecting...", disabled: true },
};

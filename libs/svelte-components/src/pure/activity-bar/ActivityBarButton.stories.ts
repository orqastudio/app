import type { Meta, StoryObj } from "@storybook/svelte";
import ActivityBarButton from "./ActivityBarButton.svelte";

const meta = {
	title: "Pure/ActivityBar/ActivityBarButton",
	component: ActivityBarButton,
	tags: ["autodocs"],
} satisfies Meta<typeof ActivityBarButton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: { icon: "file-text", label: "Artifacts", active: false },
};

export const Active: Story = {
	args: { icon: "file-text", label: "Artifacts", active: true },
};

export const ChatInactive: Story = {
	args: { icon: "message-square", label: "Chat", active: false },
};

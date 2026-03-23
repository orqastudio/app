import type { Meta, StoryObj } from "@storybook/svelte";
import AppShell from "./AppShell.svelte";

const meta = {
	title: "Connected/AppShell",
	component: AppShell,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {
		showNavPanel: true,
		showChatPanel: true,
		mainPanelSize: 70,
		chatPanelSize: 30,
	},
};

export const NoChatPanel: Story = {
	args: {
		showNavPanel: true,
		showChatPanel: false,
	},
};

export const NoNavPanel: Story = {
	args: {
		showNavPanel: false,
		showChatPanel: true,
	},
};

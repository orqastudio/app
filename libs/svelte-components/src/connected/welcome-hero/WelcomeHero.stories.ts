import type { Meta, StoryObj } from "@storybook/svelte";
import WelcomeHero from "./WelcomeHero.svelte";

const meta = {
	title: "Connected/WelcomeHero",
	component: WelcomeHero,
	tags: ["autodocs"],
} satisfies Meta<WelcomeHero>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		title: "Welcome to OrqaStudio",
		subtitle: "Open a project to get started",
	},
};

export const WithLogo: Story = {
	args: {
		title: "OrqaDev",
		subtitle: "Developer tools for OrqaStudio",
		logoSrc:
			"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Ccircle cx='12' cy='12' r='10' fill='%23666'/%3E%3C/svg%3E",
		logoAlt: "OrqaDev",
	},
};

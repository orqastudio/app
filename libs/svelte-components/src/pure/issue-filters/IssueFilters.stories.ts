// IssueFilters stories — demonstrates default state, active filters, and search query.
import type { Meta, StoryObj } from "@storybook/svelte";
import IssueFilters from "./IssueFilters.svelte";

const meta = {
	title: "Pure/IssueFilters",
	component: IssueFilters,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

/** Default state: sorted by last_seen descending, no active filters. */
export const Default: Story = {
	args: {
		sortBy: "last_seen",
		sortDir: "desc",
		filterLevel: undefined,
		filterComponent: undefined,
		searchQuery: "",
		components: ["AuthService", "UserStore", "ApiGateway", "DatabasePool"],
	},
};

/** Active filters: Error level selected, component filtered, sorted by count ascending. */
export const WithActiveFilters: Story = {
	args: {
		sortBy: "count",
		sortDir: "asc",
		filterLevel: "Error",
		filterComponent: "AuthService",
		searchQuery: "",
		components: ["AuthService", "UserStore", "ApiGateway", "DatabasePool"],
	},
};

/** Search query entered with no other filters active. */
export const WithSearchQuery: Story = {
	args: {
		sortBy: "last_seen",
		sortDir: "desc",
		filterLevel: undefined,
		filterComponent: undefined,
		searchQuery: "connection refused",
		components: ["AuthService", "UserStore"],
	},
};

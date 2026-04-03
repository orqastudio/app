// Tests for MetricCell — label, value, trend display.
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import MetricCell from "../../src/pure/metric-cell/MetricCell.svelte";

describe("MetricCell", () => {
	it("renders the label", () => {
		const { getByText } = render(MetricCell, { props: { label: "Open Tasks", value: 12 } });
		expect(getByText("Open Tasks")).toBeInTheDocument();
	});

	it("renders the numeric value", () => {
		const { getByText } = render(MetricCell, { props: { label: "Open Tasks", value: 12 } });
		expect(getByText("12")).toBeInTheDocument();
	});

	it("renders a string value", () => {
		const { getByText } = render(MetricCell, { props: { label: "Status", value: "Active" } });
		expect(getByText("Active")).toBeInTheDocument();
	});

	it("does not render trend when trend prop is omitted", () => {
		const { queryByText } = render(MetricCell, { props: { label: "L", value: 1 } });
		// No + or - sign from trendArrow/formatTrend
		expect(queryByText(/[+\-]\d+%/)).toBeNull();
	});

	it("renders a positive trend with + sign", () => {
		const { getByText } = render(MetricCell, {
			props: { label: "L", value: 1, trend: 10 },
		});
		expect(getByText(/\+10%/)).toBeInTheDocument();
	});

	it("renders a negative trend with - sign", () => {
		const { getByText } = render(MetricCell, {
			props: { label: "L", value: 1, trend: -5 },
		});
		expect(getByText(/-5%/)).toBeInTheDocument();
	});
});

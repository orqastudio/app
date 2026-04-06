/**
 * Sparkline path generation utilities.
 * Used by the Sparkline component and exported for custom SVG rendering.
 */

/**
 * Generate an SVG path string from a series of numeric values.
 * Normalises values to fit within the given width/height.
 * @param values - Array of numeric data points to plot.
 * @param width - Total SVG width in pixels.
 * @param height - Total SVG height in pixels.
 * @param options - Optional rendering configuration.
 * @param options.padding - Vertical padding in pixels (default 4).
 * @param options.fixedMin - Fixed minimum value; overrides computed min.
 * @param options.fixedMax - Fixed maximum value; overrides computed max.
 * @returns SVG path data string starting with M, or empty string if fewer than 2 values.
 */
export function sparklinePath(
	values: readonly number[],
	width: number,
	height: number,
	options?: { readonly padding?: number; readonly fixedMin?: number; readonly fixedMax?: number },
): string {
	if (values.length < 2) return "";
	const padding = options?.padding ?? 4;
	const min = options?.fixedMin ?? 0;
	const max = options?.fixedMax ?? Math.max(...values, 1);
	const range = max - min || 1;
	const usableHeight = height - padding * 2;
	const stepX = width / (values.length - 1);
	const points = values.map(
		(v, i) => `${i * stepX},${padding + usableHeight - ((v - min) / range) * usableHeight}`,
	);
	return `M${points.join(" L")}`;
}

/**
 * Calculate trend percentage between two values.
 * Returns null if insufficient data.
 * @param current - The current metric value.
 * @param previous - The previous metric value to compare against.
 * @returns Percentage change rounded to nearest integer, or null when previous is zero and current is non-zero.
 */
export function trendPercent(current: number, previous: number): number | null {
	if (previous === 0) {
		if (current === 0) return 0;
		return 100;
	}
	return Math.round(((current - previous) / previous) * 100);
}

/**
 * Format a trend percentage with sign.
 * @param pct - The trend percentage to format, or null for no data.
 * @returns Formatted string like "+12%" or "-5%", empty string when null or zero.
 */
export function formatTrend(pct: number | null): string {
	if (pct === null) return "";
	if (pct === 0) return "0%";
	const sign = pct > 0 ? "+" : "";
	return `${sign}${pct}%`;
}

/**
 * Get a trend arrow character.
 * @param pct - The trend percentage to represent, or null for no data.
 * @returns An up or down arrow character, or empty string when null or zero.
 */
export function trendArrow(pct: number | null): string {
	if (pct === null || pct === 0) return "";
	return pct > 0 ? "\u2191" : "\u2193";
}

/**
 * Determine trend colour class.
 * @param pct - Trend percentage value, or null for no data.
 * @param lowerIsBetter - If true, negative trend = good (green). Default true.
 * @returns Tailwind text color class name appropriate for the trend direction.
 */
export function trendColorClass(pct: number | null, lowerIsBetter = true): string {
	if (pct === null || pct === 0) return "text-muted-foreground";
	if (lowerIsBetter) {
		return pct < 0 ? "text-success" : "text-destructive";
	}
	return pct > 0 ? "text-success" : "text-destructive";
}

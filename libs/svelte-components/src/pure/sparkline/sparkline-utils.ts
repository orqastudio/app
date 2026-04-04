/**
 * Sparkline path generation utilities.
 * Used by the Sparkline component and exported for custom SVG rendering.
 */

/**
 * Generate an SVG path string from a series of numeric values.
 * Normalises values to fit within the given width/height.
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
 */
export function trendPercent(current: number, previous: number): number | null {
	if (previous === 0) {
		if (current === 0) return 0;
		return 100;
	}
	return Math.round(((current - previous) / previous) * 100);
}

/** Format a trend percentage with sign. */
export function formatTrend(pct: number | null): string {
	if (pct === null) return "";
	if (pct === 0) return "0%";
	const sign = pct > 0 ? "+" : "";
	return `${sign}${pct}%`;
}

/** Get a trend arrow character. */
export function trendArrow(pct: number | null): string {
	if (pct === null || pct === 0) return "";
	return pct > 0 ? "\u2191" : "\u2193";
}

/**
 * Determine trend colour class.
 * @param pct - trend percentage
 * @param lowerIsBetter - if true, negative trend = good (green). Default true.
 */
export function trendColorClass(pct: number | null, lowerIsBetter = true): string {
	if (pct === null || pct === 0) return "text-muted-foreground";
	if (lowerIsBetter) {
		return pct < 0 ? "text-success" : "text-destructive";
	}
	return pct > 0 ? "text-success" : "text-destructive";
}

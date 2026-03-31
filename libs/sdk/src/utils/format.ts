/**
 * Number formatting utilities for consistent display across the app.
 *
 * All decimal output is limited to 2 decimal places by default,
 * with trailing zeros stripped for clean display.
 */

/**
 * Format a number to at most `decimals` decimal places, stripping trailing zeros.
 * @param n - The number to format.
 * @param decimals - Maximum decimal places (default 2).
 * @returns Formatted string (e.g., 5.79 → "5.79", 3.0 → "3", 100.00 → "100").
 */
export function fmt(n: number, decimals = 2): string {
	return parseFloat(n.toFixed(decimals)).toString();
}

/**
 * Format a 0–1 ratio as a percentage string with at most 2 decimal places.
 * @param ratio - A number between 0 and 1.
 * @returns Percentage string without the % symbol (e.g., 0.916 → "91.6").
 */
export function pct(ratio: number): string {
	return fmt(ratio * 100);
}

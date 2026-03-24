/**
 * Hash-based router for the explorer panel.
 *
 * Encodes the current view state in window.location.hash so that:
 * - Back/forward browser buttons work
 * - Hot reload preserves the current view
 * - Deep linking to specific artifacts is possible
 *
 * The chat panel sits outside the router — only the explorer content routes.
 *
 * Route format:
 *   #/project                              → Project dashboard
 *   #/artifacts/:activity                  → Artifact list for an activity
 *   #/artifacts/:activity/:path            → Specific artifact viewer
 *   #/plugin/:pluginName/:viewKey          → Plugin view
 *   #/settings                             → Settings
 *   #/graph                                → Full graph view
 *   #/setup                                → Setup wizard
 *   #/                                     → Default (chat/welcome)
 */

// Navigation functions — injected by the app during initialization.
// The SDK can't import $app/navigation directly (it's a standalone library).
// The app calls injectNavigation() with SvelteKit's pushState/replaceState.
let _pushState: (url: string, state: Record<string, unknown>) => void = (url) => history.pushState(null, "", url);
let _replaceState: (url: string, state: Record<string, unknown>) => void = (url) => history.replaceState(null, "", url);

/** Inject SvelteKit navigation functions. Call once from the app's root layout. */
export function injectNavigation(
	pushState: (url: string, state: Record<string, unknown>) => void,
	replaceState: (url: string, state: Record<string, unknown>) => void,
): void {
	_pushState = pushState;
	_replaceState = replaceState;
}

// ── Navigation API for plugins and SDK consumers ────────────────────────────

/** Navigate to an artifact by path (e.g., ".orqa/process/rules/RULE-abc.md"). */
export function navigateToArtifact(artifactPath: string, activity?: string): void {
	pushRoute({ type: "artifact", activity: activity ?? "explorer", artifactPath });
}

/** Navigate to a plugin view. */
export function navigateToPluginView(pluginName: string, viewKey: string): void {
	pushRoute({ type: "plugin", pluginName, viewKey });
}

/** Navigate to an activity panel (e.g., "roadmap", "lessons", "settings"). */
export function navigateToActivity(activity: string): void {
	pushRoute({ type: "artifacts", activity });
}

/** Navigate to the project dashboard. */
export function navigateToProject(): void {
	pushRoute({ type: "project" });
}

/** Navigate to the artifact graph view. */
export function navigateToGraph(): void {
	pushRoute({ type: "graph" });
}

/** Navigate to settings. */
export function navigateToSettings(): void {
	pushRoute({ type: "settings" });
}

export interface ParsedRoute {
	type: "project" | "artifacts" | "artifact" | "plugin" | "settings" | "graph" | "setup" | "default";
	activity?: string;
	artifactPath?: string;
	pluginName?: string;
	viewKey?: string;
}

/**
 * Parse a hash string into a structured route.
 */
export function parseHash(hash: string): ParsedRoute {
	// Remove leading # and /
	const path = hash.replace(/^#\/?/, "");
	if (!path) return { type: "default" };

	const segments = path.split("/");

	if (segments[0] === "project") {
		return { type: "project" };
	}

	if (segments[0] === "settings") {
		return { type: "settings" };
	}

	if (segments[0] === "graph") {
		return { type: "graph" };
	}

	if (segments[0] === "setup") {
		return { type: "setup" };
	}

	if (segments[0] === "plugin" && segments.length >= 3) {
		// #/plugin/@orqastudio/plugin-software-project/roadmap
		// Plugin names can contain / (scoped packages), so join all middle segments
		const viewKey = segments[segments.length - 1];
		const pluginName = segments.slice(1, -1).join("/");
		return { type: "plugin", pluginName, viewKey };
	}

	if (segments[0] === "artifacts") {
		if (segments.length === 1) {
			return { type: "artifacts" };
		}
		// Check if the second segment looks like a file path (contains / or .)
		const rest = segments.slice(1).join("/");
		if (rest.includes(".") || rest.includes("\\")) {
			// It's an artifact path: #/artifacts/docs/.orqa/delivery/epics/EPIC-001.md
			const firstSlash = rest.indexOf("/");
			if (firstSlash > 0) {
				return {
					type: "artifact",
					activity: rest.substring(0, firstSlash),
					artifactPath: rest.substring(firstSlash + 1),
				};
			}
		}
		// It's an activity: #/artifacts/docs or #/artifacts/ideas
		return { type: "artifacts", activity: rest };
	}

	// Fallback — treat as an activity name for backwards compat
	return { type: "artifacts", activity: segments[0] };
}

/**
 * Build a hash string from route parameters.
 */
export function buildHash(route: ParsedRoute): string {
	switch (route.type) {
		case "project":
			return "#/project";
		case "settings":
			return "#/settings";
		case "graph":
			return "#/graph";
		case "setup":
			return "#/setup";
		case "plugin":
			return `#/plugin/${route.pluginName}/${route.viewKey}`;
		case "artifact":
			return `#/artifacts/${route.activity}/${route.artifactPath}`;
		case "artifacts":
			return route.activity ? `#/artifacts/${route.activity}` : "#/artifacts";
		default:
			return "#/";
	}
}

/**
 * Push a route to the browser history.
 *
 * Uses SvelteKit's pushState to avoid conflicts with the SvelteKit router.
 * This also avoids triggering the hashchange listener (which would cause a
 * loop when called from syncToHash → pushRoute → hashchange → applyRoute).
 */
export function pushRoute(route: ParsedRoute): void {
	const hash = buildHash(route);
	if (window.location.hash !== hash) {
		_pushState(hash, {});
	}
}

/**
 * Replace the current route without adding a history entry.
 */
export function replaceRoute(route: ParsedRoute): void {
	const hash = buildHash(route);
	if (window.location.hash !== hash) {
		_replaceState(hash, {});
	}
}

/**
 * Get the current route from the URL hash.
 */
export function currentRoute(): ParsedRoute {
	return parseHash(window.location.hash);
}

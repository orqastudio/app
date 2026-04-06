/**
 * App-level GraphVisualiser singleton.
 *
 * Created once, wired to the SDK's graph data via onRefresh().
 * Components import { graphViz } to access visualization elements.
 */

import { GraphVisualiser } from "@orqastudio/graph-visualiser";
import { getStores } from "@orqastudio/sdk";

let _instance: GraphVisualiser | null = null;

/**
 * Initialize the graph visualiser and wire it to the SDK's graph refresh.
 * Call once from the root layout after initializeStores().
 */
export function initializeGraphViz(): GraphVisualiser {
	if (_instance) return _instance;

	const { artifactGraphSDK } = getStores();
	_instance = new GraphVisualiser();

	// Sync on every graph refresh (return value is unlisten fn — kept alive by closure)
	artifactGraphSDK.onRefresh(() => {
		_instance!.update(artifactGraphSDK.graph);
	});

	// Initial sync if graph is already loaded
	if (artifactGraphSDK.graph.size > 0) {
		_instance.update(artifactGraphSDK.graph);
	}

	return _instance;
}

/** Access the graph visualiser. Throws if not initialized. */
export function getGraphViz(): GraphVisualiser {
	if (!_instance) {
		throw new Error(
			"[OrqaStudio] GraphVisualiser not initialized. Call initializeGraphViz() first.",
		);
	}
	return _instance;
}

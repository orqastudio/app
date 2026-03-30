/**
 * Roadmap view entry point.
 *
 * Exports a mount function that the app's PluginViewContainer calls
 * to render the roadmap into a container element. Returns a cleanup
 * function that unmounts the component.
 */

import { mount as svelteMount, unmount as svelteUnmount } from "svelte";
import RoadmapView from "./RoadmapView.svelte";

export function mount(container: HTMLElement): () => void {
	const component = svelteMount(RoadmapView, { target: container });
	return () => svelteUnmount(component);
}

export default RoadmapView;

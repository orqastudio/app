/**
 * Roadmap view entry point.
 *
 * Exports a mount function that the app's PluginViewContainer calls
 * to render the roadmap into a container element. Returns a cleanup
 * function that unmounts the component.
 */
import RoadmapView from "./RoadmapView.svelte";
/**
 * Mounts the RoadmapView Svelte component into the given container element.
 * @param container - The DOM element to mount the roadmap view into.
 * @returns A cleanup function that unmounts the component when called.
 */
export declare function mount(container: HTMLElement): () => void;
export default RoadmapView;

/**
 * MockChannel — minimal Channel mock matching \@tauri-apps/api/core Channel.
 *
 * Captures the `onmessage` callback and exposes an `emit()` helper
 * so tests can simulate backend events without a running Tauri app.
 */
export class MockChannel<T> {
	onmessage: ((event: T) => void) | null = null;

	/**
	 * Simulate an event from the backend. Calls onmessage if set.
	 * @param event
	 */
	emit(event: T): void {
		if (this.onmessage) {
			this.onmessage(event);
		}
	}
}

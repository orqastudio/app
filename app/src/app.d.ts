declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

// Svelte internal module used only in tests for effect_root access.
declare module "svelte/internal/client";

export {};

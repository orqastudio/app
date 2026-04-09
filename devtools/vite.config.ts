// Vite configuration for OrqaDev — SvelteKit + Tauri + Tailwind.
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { getPort } from "@orqastudio/constants";

// Port is read from infrastructure/ports.json via @orqastudio/constants.
// The canonical value is 10140 (base 10100 + offset 40).
// tauri.conf.json devUrl must match — validated by `orqa check ports`.
const DEVTOOLS_PORT = getPort("devtools");

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	build: { sourcemap: true },
	clearScreen: false,
	optimizeDeps: {
		exclude: ["@orqastudio/types", "@orqastudio/sdk", "@orqastudio/svelte-components"],
	},
	server: {
		port: DEVTOOLS_PORT,
		strictPort: true,
		allowedHosts: true,
		fs: {
			allow: ["src", ".svelte-kit", "node_modules", "../libs"],
		},
	},
});

// Vite configuration for OrqaDev — SvelteKit + Tauri + Tailwind.
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  clearScreen: false,
  optimizeDeps: {
    exclude: [
      '@orqastudio/types',
      '@orqastudio/sdk',
      '@orqastudio/svelte-components',
    ]
  },
  server: {
    port: 10421,
    strictPort: true,
    allowedHosts: true,
    fs: {
      allow: ['src', '.svelte-kit', 'node_modules', '../libs']
    },
  },
});

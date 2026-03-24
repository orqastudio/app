import { sveltekit } from '@sveltejs/kit/vite';
import { svelteTesting } from '@testing-library/svelte/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [tailwindcss(), sveltekit(), svelteTesting()],
  clearScreen: false,
  optimizeDeps: {
    // Don't pre-bundle linked @orqastudio packages — read from dist directly
    // so that library watcher rebuilds are picked up immediately by Vite HMR.
    exclude: [
      '@orqastudio/types',
      '@orqastudio/sdk',
      '@orqastudio/svelte-components',
      '@orqastudio/graph-visualiser',
    ]
  },
  server: {
    port: 10420,
    strictPort: true,
    allowedHosts: true,
    fs: {
      allow: ['src', '.svelte-kit', 'node_modules', '../../libs', '../../plugins', '../../connectors']
    },
    watch: {
      ignored: ['**/.orqa/**']
    }
  },
  test: {
    environment: 'jsdom',
    setupFiles: ['src/lib/components/shared/__tests__/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'text-summary'],
      include: ['src/**/*.{ts,svelte}'],
      exclude: ['**/*.test.ts', '**/node_modules/**']
      // Target: 80% coverage. Not enforced yet — thresholds will be enabled
      // once coverage reaches the target. See RULE-029.
    }
  }
});

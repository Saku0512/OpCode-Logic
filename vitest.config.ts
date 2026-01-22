import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [
    // sveltekit() の代わりに、テスト用に svelte プラグインを直接使用して preprocess を無効化
    svelte({
      compilerOptions: {
        dev: true,
      },
      // svelte.config.js の preprocess を上書きして無効化
      preprocess: [],
    }),
  ],
  resolve: {
    alias: {
      $lib: '/src/lib',
    },
  },
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
    environment: 'jsdom',
    setupFiles: ['./src/test/setup.ts'],
    globals: true,
    server: {
      deps: {
        inline: ['@sveltejs/vite-plugin-svelte'],
      },
    },
  },
});

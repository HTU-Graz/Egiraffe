import devtools from 'solid-devtools/vite';
import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
  plugins: [devtools(), solidPlugin()],
  server: {
    port: 3000,
    proxy: {
      '/api': 'http://127.0.0.42:42002',
    },
  },
  build: {
    target: 'esnext',
  },
});

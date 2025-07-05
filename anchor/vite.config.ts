import { nodePolyfills } from 'vite-plugin-node-polyfills'
import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'
import viteTsconfigPaths from 'vite-tsconfig-paths'
import { resolve } from 'node:path'
import dts from 'vite-plugin-dts'

// https://vite.dev/config/
export default defineConfig({
  optimizeDeps: {
    include: ['buffer', 'process'],
  },
  plugins: [
    dts({ insertTypesEntry: true }),
    nodePolyfills({}),
    tailwindcss(),
    viteTsconfigPaths({
      root: resolve(__dirname),
    }),
  ],
  build: {
    outDir: '../dist',
    sourcemap: true,
    lib: {
      entry: 'src/index.ts',
      name: 'bls-node-registration',
      fileName: 'index',
    },
    rollupOptions: {
      external: ['fs', 'path'],
    },
  },
  root: resolve(__dirname),
})

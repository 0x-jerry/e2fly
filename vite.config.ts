import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { join } from 'path'
import { VitePluginElectronBuilder } from './vite-plugin'

const r = (...path: string[]) => join(__dirname, ...path)

// https://vitejs.dev/config/
export default defineConfig({
  base: './',
  root: r('src-render'),
  plugins: [
    vue(),
    VitePluginElectronBuilder({
      root: process.cwd(),
      tsconfig: './tsconfig.main.json',
    }),
  ],
  resolve: {
    alias: {
      '@render': r('src-render'),
      '@main': r('src-main'),
    },
  },
  build: {
    outDir: r('dist/render'),
    emptyOutDir: true,
  },
  test: {
    globals: true,
  },
})

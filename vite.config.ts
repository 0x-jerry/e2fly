/// <reference types="vitest/config" />

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { join } from 'path'
import { VitePluginElectronBuilder } from './vite-plugin'

const r = (...path: string[]) => join(__dirname, ...path)

// https://vitejs.dev/config/
export default defineConfig({
  base: './',
  root: r('src/render'),
  plugins: [
    vue(),
    VitePluginElectronBuilder({
      root: process.cwd(),
      tsconfig: './tsconfig.main.json',
      entry: [
        {
          input: r('src/preload/index.ts'),
          output: r('dist/preload/index.js'),
        },
        {
          input: r('src/main/index.ts'),
          output: r('dist/main/index.js'),
          electron: true,
        },
      ],
    }),
  ],
  resolve: {
    alias: {
      '@': r('src'),
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

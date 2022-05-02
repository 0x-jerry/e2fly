/// <reference types="vitest/config" />
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { join } from 'path'
import { VitePluginElectronBuilder } from './vite-plugin'
import Pages from 'vite-plugin-pages'
import Components from 'unplugin-vue-components/vite'
import AutoImport from 'unplugin-auto-import/vite'
import Icons from 'unplugin-icons/vite'
import IconsResolver from 'unplugin-icons/resolver'
import Unocss from 'unocss/vite'
import { presetAttributify, presetWind } from 'unocss'
import transformerDirective from '@unocss/transformer-directives'
import { VueKitResolver } from '@0x-jerry/vue-kit/resolver'

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

    Icons(),

    // https://github.com/unocss/unocss
    Unocss({
      presets: [presetAttributify(), presetWind()],
      transformers: [transformerDirective()],
    }),

    // https://github.com/antfu/unplugin-auto-import
    AutoImport({
      dts: 'auto-imports.d.ts',
      imports: ['vue', 'vue-router', '@vueuse/core'],
    }),

    // https://github.com/antfu/vite-plugin-components
    Components({
      dts: 'auto-components.d.ts',
      dirs: ['components'],
      resolvers: [VueKitResolver(), IconsResolver()],
    }),

    // https://github.com/hannoeru/vite-plugin-pages
    Pages({
      dirs: ['pages'],
      exclude: ['**/components/*.vue'],
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

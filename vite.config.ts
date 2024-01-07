import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { join } from 'path'
import Pages from 'vite-plugin-pages'
import Components from 'unplugin-vue-components/vite'
import AutoImport from 'unplugin-auto-import/vite'
import Icons from 'unplugin-icons/vite'
import IconsResolver from 'unplugin-icons/resolver'
import Unocss from 'unocss/vite'
import vueI18n from '@intlify/unplugin-vue-i18n/vite'

const r = (...path: string[]) => join(__dirname, ...path)

// https://vitejs.dev/config/
export default defineConfig({
  base: './',
  root: '.',
  server: {
    watch: {
      ignored: ['**/test-conf/**/*', '**/src-tauri/**/*'],
    },
  },
  plugins: [
    vue(),
    vueI18n({
      include: r('src/locales/**'),
    }),

    Icons({}),

    // https://github.com/unocss/unocss
    Unocss(),

    // https://github.com/antfu/unplugin-auto-import
    AutoImport({
      dts: 'types/auto-imports.d.ts',
      imports: ['vue', 'vue-router', '@vueuse/core'],
    }),

    // https://github.com/antfu/vite-plugin-components
    Components({
      dts: 'types/auto-components.d.ts',
      resolvers: [IconsResolver()],
    }),

    // https://github.com/hannoeru/vite-plugin-pages
    Pages({
      exclude: ['**/components/*.vue'],
    }),
  ],
  resolve: {
    alias: {
      '@': r('src'),
    },
  },
  build: {
    emptyOutDir: true,
  },
})

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { join } from 'path'
import VueRouter from 'unplugin-vue-router/vite'
import Components from 'unplugin-vue-components/vite'
import { PrimeVueResolver } from 'unplugin-vue-components/resolvers'
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
    // https://github.com/posva/unplugin-vue-router
    VueRouter({
      dts: 'types/typed-router.d.ts',
      exclude: ['**/components/*.vue'],
    }),

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
      resolvers: [
        IconsResolver(),
        PrimeVueResolver({
          importStyle: false,
        }),
      ],
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

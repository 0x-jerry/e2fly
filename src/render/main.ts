import generatedRoutes from 'virtual:generated-pages'
import { createRouter, createWebHashHistory } from 'vue-router'
import { createApp } from 'vue'
import App from './App.vue'
import { createI18n } from 'vue-i18n'
import messages from '@intlify/vite-plugin-vue-i18n/messages'

import '@0x-jerry/vue-kit/dist/style.css'
import 'normalize.css'
import 'uno.css'

const app = createApp(App)

const routes = generatedRoutes

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

app.use(router)

const i18n = createI18n({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'en',
  globalInjection: true,
  messages,
})

app.use(i18n)

app.mount('#app')

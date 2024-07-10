import generatedRoutes from 'virtual:generated-pages'
import { createRouter, createWebHashHistory } from 'vue-router'
import { createApp } from 'vue'
import { i18n } from './i18n'
import App from './App.vue'
import PrimeVue, { type PrimeVueConfiguration } from 'primevue/config'
import ToastService from 'primevue/toastservice'
import Aura from '@primevue/themes/aura'
import './logic/win'

import './style'
import 'uno.css'

const app = createApp(App)

const routes = generatedRoutes

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      prefix: 'p',
      darkModeSelector: 'system',
      cssLayer: true,
    },
  },
  ripple: true,
} as PrimeVueConfiguration)

app.use(ToastService)

app.use(router)

app.use(i18n)

app.mount('#app')

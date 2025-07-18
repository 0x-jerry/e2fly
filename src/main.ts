import Aura from '@primeuix/themes/aura'
import PrimeVue, { type PrimeVueConfiguration } from 'primevue/config'
import ToastService from 'primevue/toastservice'
import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import { routes } from 'vue-router/auto-routes'
import App from './App.vue'
import { i18n } from './i18n'

import './style'
import 'uno.css'

const app = createApp(App)

function extendRoutes() {
  for (const route of routes) {
    if (route.path === '/') {
      route.children?.push({
        path: '/',
        redirect: '/server',
      })
    }
  }

  return routes
}

const router = createRouter({
  history: createWebHashHistory(),
  routes: extendRoutes(),
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

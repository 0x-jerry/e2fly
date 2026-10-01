import * as log from '@tauri-apps/plugin-log'
import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import { handleHotUpdate, routes } from 'vue-router/auto-routes'
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

if (import.meta.hot) {
  handleHotUpdate(router)
}

app.use(router)

app.use(i18n)

app.mount('#app')

window.addEventListener('unhandledrejection', async (evt) => {
  await log.error(String(evt.reason))
})

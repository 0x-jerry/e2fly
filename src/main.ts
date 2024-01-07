import generatedRoutes from 'virtual:generated-pages'
import { createRouter, createWebHashHistory } from 'vue-router'
import { createApp } from 'vue'
import { i18n } from './i18n'
import App from './App.vue'
import './logic/win'

import 'normalize.css'
import 'uno.css'

const app = createApp(App)

const routes = generatedRoutes

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

app.use(router)

app.use(i18n)

app.mount('#app')

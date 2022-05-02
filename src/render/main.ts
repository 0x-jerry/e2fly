import generatedRoutes from 'virtual:generated-pages'
import { createRouter, createWebHashHistory } from 'vue-router'
import { createApp } from 'vue'
import App from './App.vue'

import '@0x-jerry/vue-kit/dist/style.css'
// import '@unocss/reset/tailwind.css'
import 'uno.css'

const app = createApp(App)

const routes = generatedRoutes

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

app.use(router)

app.mount('#app')

import messages from '@intlify/unplugin-vue-i18n/messages'
import { createI18n } from 'vue-i18n'

export const i18n = createI18n({
  legacy: false,
  locale: navigator.language,
  fallbackLocale: 'en',
  globalInjection: true,
  messages,
})

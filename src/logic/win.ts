import { store } from '@/store'
import { appWindow } from '@tauri-apps/api/window'

appWindow.listen('tauri://blur', () => {
  if (store.config.app.autoHideWhenBlur) {
    appWindow.hide()
  }
})

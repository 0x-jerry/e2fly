import { store } from '@/store'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
const appWindow = getCurrentWebviewWindow()

appWindow.listen('tauri://blur', () => {
  if (store.config.app.autoHideWhenBlur) {
    appWindow.hide()
  }
})

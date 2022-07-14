import { appWindow } from '@tauri-apps/api/window'

appWindow.listen('tauri://blur', () => {
  appWindow.hide()
})

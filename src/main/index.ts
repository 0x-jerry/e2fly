import { app } from 'electron'
import { isDev } from './config'
import './rpc'
import { createWindow, restoreOrCreateWindow } from './mainWindow'
import { initServices } from './service'
import { createTray } from './tray'

process.env['ELECTRON_DISABLE_SECURITY_WARNINGS'] = 'true'

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

app.on('activate', async () => {
  try {
    await restoreOrCreateWindow()
  } catch (error) {
    app.quit()
  }
})

app.on('ready', async () => {
  try {
    createTray()
    await initServices()
    await createWindow()
  } catch (error) {
    app.quit()
  }
})

if (isDev) {
  if (process.platform === 'win32') {
    process.on('message', (data) => {
      if (data === 'graceful-exit') {
        app.quit()
      }
    })
  } else {
    process.on('SIGTERM', () => {
      app.quit()
    })
  }
}

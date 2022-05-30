import { app } from 'electron'
import { isDev } from './config'
import './rpc'
import { createWindow, restoreOrCreateWindow } from './mainWindow'
import { initServices, services } from './service'
import { createTray } from './tray'
import { isMac } from './utils'

if (isDev) {
  require('source-map-support').install()
}

process.env['ELECTRON_DISABLE_SECURITY_WARNINGS'] = 'true'

const gotTheLock = app.requestSingleInstanceLock()

if (!gotTheLock) {
  app.exit()
} else {
  createInstance()
}

function createInstance() {
  if (isMac()) {
    app.dock.hide()
  }

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

  app.on('before-quit', () => {
    services.v2fly?.stop()
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
}

import { app, BrowserWindow } from 'electron'
import { join } from 'path'
import { isDev } from './config'

export async function createWindow() {
  const win = new BrowserWindow({
    width: 1000,
    height: 640,
    minHeight: 640,
    minWidth: 1000,
    autoHideMenuBar: true,
    center: true,
    webPreferences: {
      devTools: isDev,
    },
  })

  win.maximize()

  const URL = isDev
    ? process.env.DEV_SERVER_URL!
    : `file://${join(app.getAppPath(), 'dist/render/index.html')}`

  win.loadURL(URL)

  if (isDev) {
    win.webContents.openDevTools()
  } else {
    win.removeMenu()
  }

  win.on('closed', () => {
    win.destroy()
  })

  return win
}

export async function restoreOrCreateWindow() {
  let window = BrowserWindow.getAllWindows().find((w) => !w.isDestroyed())

  if (window === undefined) {
    window = await createWindow()
  }

  if (window.isMinimized()) {
    window.restore()
  }

  window.focus()
}

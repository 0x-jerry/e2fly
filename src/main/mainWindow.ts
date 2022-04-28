import { BrowserWindow } from 'electron'
import { isDev } from './config'
import { getResourcePath } from './utils'

export async function createWindow() {
  const win = new BrowserWindow({
    width: 1000,
    height: 640,
    minHeight: 640,
    minWidth: 1000,
    autoHideMenuBar: true,
    center: true,
    webPreferences: {
      preload: getResourcePath('dist/preload/index.js'),
    },
  })

  win.maximize()

  const URL = isDev
    ? process.env.DEV_SERVER_URL!
    : 'file://' + getResourcePath('dist/render/index.html')

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

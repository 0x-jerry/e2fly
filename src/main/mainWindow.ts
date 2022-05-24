import { BrowserWindow } from 'electron'
import { isDev } from './config'
import { getResourcePath } from './utils'

export async function createWindow() {
  const win = new BrowserWindow({
    width: 350,
    height: 640,
    resizable: false,
    center: true,
    show: isDev,
    autoHideMenuBar: true,
    frame: false,
    webPreferences: {
      preload: getResourcePath('dist/preload/index.js'),
    },
  })

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

  if (!isDev) {
    win.on('blur', () => win.hide())
  }

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

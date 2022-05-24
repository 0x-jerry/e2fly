import { app, Tray, Menu, nativeImage } from 'electron'
import { getResourcePath, isMac } from './utils'

let tray: Tray

export function createTray() {
  const iconPath = isMac() ? 'dist/assets/menu.png' : 'dist/assets/favicon/favicon-16x16.png'

  const icon = nativeImage.createFromPath(getResourcePath(iconPath))
  tray = new Tray(icon)

  const contextMenu = Menu.buildFromTemplate([
    { label: 'Item1', type: 'radio' },
    { label: 'Item2', type: 'radio' },
    { label: 'Item3', type: 'radio', checked: true },
    { label: 'Item4', type: 'radio' },
  ])

  tray.setContextMenu(contextMenu)

  tray.setToolTip('This is my application')
}

import { Tray, nativeImage, BrowserWindow, Menu, app } from 'electron'
import { wins } from './mainWindow'
import { getResourcePath, isWin } from './utils'

let tray: Tray

export function createTray() {
  const iconPath = 'dist/assets/logoTemplate.png'

  const icon = nativeImage.createFromPath(getResourcePath(iconPath))
  tray = new Tray(icon)
  tray.setIgnoreDoubleClickEvents(true)

  const menu = Menu.buildFromTemplate([
    {
      label: 'Quit E2Fly',
      role: 'quit',
      accelerator: 'CommandOrControl+Q',
    },
  ])

  tray.on('right-click', () => {
    tray.popUpContextMenu(menu)
  })

  tray.on('click', async (_e, bounds) => {
    BrowserWindow.getAllWindows().forEach((n) => {
      if (n.isDestroyed()) return

      toggle(n)
    })

    function toggle(win: BrowserWindow) {
      if (win.isVisible()) {
        win.hide()
        return
      }

      if (!isWin()) {
        const width = win.getSize()[0] / 2
        win.setPosition(bounds.x - width, bounds.y)
      }

      win.show()
      win.moveTop()
    }
  })
}

export function initAppMenu() {
  const menu = Menu.buildFromTemplate([
    {
      label: 'E2fly',
      submenu: [
        {
          label: 'Close',
          accelerator: 'CommandOrControl+W',
          click() {
            wins.float?.hide()
          },
        },
        {
          label: 'Quit',
          role: 'quit',
        },
      ],
    },
  ])

  Menu.setApplicationMenu(menu)
}

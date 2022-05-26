import { Tray, nativeImage, BrowserWindow, Menu, app } from 'electron'
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
      type: 'normal',
      click() {
        app.exit()
      },
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

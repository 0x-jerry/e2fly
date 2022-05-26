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

      show(n)
    })

    function show(n: BrowserWindow) {
      if (!isWin()) {
        const width = n.getSize()[0] / 2
        n.setPosition(bounds.x - width, bounds.y)
      }

      n.show()
      n.moveTop()
    }
  })
}

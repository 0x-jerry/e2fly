import { createRPC, uuid } from '@0x-jerry/utils'
import { methods } from './methods'
import { BrowserWindow, ipcMain } from 'electron'
import { logger } from '../utils'
import type { RendererThreadMethods } from '@/render/rpc/methods'
import { isDev } from '../config'

const rpcID = uuid(8)
process.env.E2FLY_RPC_PROTOCOL_ID = rpcID

const rpc = createRPC<RendererThreadMethods>(methods, {
  id: rpcID,
  verbose: isDev,
  send(data) {
    BrowserWindow.getAllWindows()
      .filter((n) => !n.isDestroyed())
      .forEach((win) => {
        win.webContents.send('rpc', data)
      })
  },
})

export const rpcMainProxy = rpc.proxy

ipcMain.on('rpc', (e, data) => {
  logger.log('receive rpc msg:', data)

  rpc.receive({
    ...data,
    send: (data) => e.sender.send('rpc', data),
  })
})

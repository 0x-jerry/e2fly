import { createRPC, uuid } from '@0x-jerry/utils'
import { methods } from './methods'
import { ipcMain } from 'electron'
import { logger } from '../utils'

const rpcID = uuid(8)
process.env.E2FLY_RPC_PROTOCOL_ID = rpcID

const rpc = createRPC(methods, { id: rpcID })

ipcMain.on('rpc', (e, data) => {
  logger.log('receive rpc msg:', data)

  rpc.receive({
    ...data,
    send: (data) => e.sender.send('rpc', data),
  })
})

import { RPCRequestCtx } from '@0x-jerry/utils'
import { contextBridge, ipcRenderer } from 'electron'

const expose = {
  rpc: {
    id: process.env.E2FLY_RPC_PROTOCOL_ID,
    send: (data: any) => ipcRenderer.send('rpc', data),
    receiver: (resolve: (msg: RPCRequestCtx) => any) => {
      ipcRenderer.on('rpc', (e, data) => {
        resolve({ ...data, send: (data) => ipcRenderer.send('rpc', data) })
      })
    },
  },
}

contextBridge.exposeInMainWorld('_e2fly_', expose)

declare global {
  export interface Window {
    _e2fly_: typeof expose
  }
}

import { contextBridge } from 'electron'

contextBridge.exposeInMainWorld('_rpc_', {
  addr: process.env.DEV_WS_ADDR,
})

import { contextBridge } from 'electron'

const expose = {
  rpc: {
    addr: process.env.E2FLY_WS_ADDR,
    id: process.env.E2FLY_RPC_PROTOCOL_ID,
  },
}

contextBridge.exposeInMainWorld('_e2fly_', expose)

declare global {
  export interface Window {
    _e2fly_: typeof expose
  }
}

import { createRPC } from '@0x-jerry/utils'
import type { MainThreadMethods } from '@/main/rpc/methods'
import { methods } from './methods'

const ws = new WebSocket(window._rpc_.addr)

export const rpc = createRPC<MainThreadMethods>(methods, {
  receive(resolve) {
    ws.addEventListener('message', (e) => resolve(JSON.parse(e.data)))
  },
  send(data) {
    ws.send(JSON.stringify(data))
  },
})

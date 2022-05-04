import { createRPC } from '@0x-jerry/utils'
import type { MainThreadMethods } from '@/main/rpc/methods'
import { methods } from './methods'
import { e2fly } from '../bridge'
import { isDev } from '../config'

// const ws = new WebSocket(e2fly.rpc.addr!)

export const rpc = createRPC<MainThreadMethods>(methods, {
  verbose: isDev,
  id: e2fly.rpc.id!,
  send: (data) => e2fly.rpc.send(data),
})

export const rpcProxy = rpc.proxy

e2fly.rpc.receiver(rpc.receive)

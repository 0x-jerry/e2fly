import { createRPC, uuid } from '@0x-jerry/utils'
import { WebSocketServer } from 'ws'
import { methods } from './methods'
import getPort from 'get-port'

const rpcID = uuid(8)
process.env.E2FLY_RPC_PROTOCOL_ID = rpcID

getPort({ port: 54637 }).then((port) => {
  process.env.E2FLY_WS_ADDR = 'ws://localhost:' + port

  const wss = new WebSocketServer({ port })

  wss.on('connection', (ws) => {
    const rpc = createRPC(methods, { id: rpcID })

    ws.on('message', (data) => {
      rpc.receive({
        ...JSON.parse(data.toString()),
        send: (d) => ws.send(JSON.stringify(d)),
      })
    })
  })
})

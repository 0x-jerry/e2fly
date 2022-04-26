import { createRPC } from '@0x-jerry/utils'
import { WebSocketServer } from 'ws'
import { methods } from './methods'
import getPort from 'get-port'

getPort({ port: 54637 }).then((port) => {
  process.env.DEV_WS_ADDR = 'ws://localhost:' + port

  const wss = new WebSocketServer({ port })

  wss.on('connection', function connection(ws) {
    createRPC(methods, {
      receive(resolve) {
        ws.on('message', function message(data) {
          resolve(JSON.parse(data.toString('utf8')))
        })
      },
      send(data) {
        ws.send(JSON.stringify(data))
      },
    })
  })
})

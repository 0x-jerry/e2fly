import { IV2Ray } from '@0x-jerry/v2ray-schema'
import { ChildProcess, spawn } from 'child_process'
import { join } from 'path'
import { rpcMainProxy } from '../../rpc/server'
import { configDir, ConfigService } from '../config'
import fs from 'fs-extra'
import { E2FlyConfig } from '../../config'
import { getV2rayConfig } from './gen-conf'
import { throttle } from 'lodash'
import { RPCTimeoutError } from '@0x-jerry/utils'

const v2flyConfPath = join(configDir, 'v2fly.conf.json')

export class V2flyService {
  progress?: ChildProcess

  constructor(public confService: ConfigService) {
    if (confService.config.activeOutboundId) {
      this.start(confService.config.activeOutboundId)
    }
  }

  get isEnabled() {
    return !this.progress?.killed
  }

  get v2fly() {
    return this.confService.config.v2fly
  }

  logs: string[] = []
  logHandler?: any

  sendLogs = throttle(async () => {
    const logs = this.logs.splice(0)

    try {
      await rpcMainProxy.v2flyLog(logs)
    } catch (error) {
      if (!(error instanceof RPCTimeoutError)) {
        console.warn('log error:', error)
      }

      this.logs.push(...logs)
    }
  }, 100)

  async start(enableId: string) {
    this.stop()

    const conf = await generateV2flyConfig(this.confService.config, enableId)
    await fs.writeFile(v2flyConfPath, JSON.stringify(conf))

    const bin = this.v2fly.bin

    if (!(await fs.pathExists(bin))) {
      throw new Error('v2fly path not found: ' + bin)
    }

    this.progress = spawn(bin, ['-c', v2flyConfPath])

    this.progress.stdout?.on('data', (message) => {
      const str = message.toString()
      this.log(str)
    })

    this.progress.stderr?.on('data', (message) => {
      const str = message.toString()
      this.log(str)
    })
  }

  log(str: string) {
    this.logs.unshift(str)
    this.sendLogs()
  }

  stop() {
    this.progress?.kill()
  }
}

async function generateV2flyConfig(config: E2FlyConfig, enableOutboundId: string): Promise<IV2Ray> {
  const outbound = config.outbound.find((n) => n.id === enableOutboundId)

  if (!outbound) {
    throw new Error('Not found id: ' + enableOutboundId)
  }

  const conf = getV2rayConfig(config, outbound.config)

  return conf
}

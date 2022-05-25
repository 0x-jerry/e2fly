import { IV2Ray } from '@0x-jerry/v2ray-schema'
import { ChildProcess, spawn } from 'child_process'
import { join } from 'path'
import { rpcMainProxy } from '../../rpc/server'
import { configDir, ConfigService } from '../config'
import fs from 'fs-extra'
import { E2FlyConfig } from '../../config'
import { getV2rayConfig } from './gen-conf'

const v2flyConfPath = join(configDir, 'v2fly.conf.json')

export class V2flyService {
  progress?: ChildProcess

  constructor(public confService: ConfigService) {}

  get isEnabled() {
    return !this.progress?.killed
  }

  get v2fly() {
    return this.confService.config.v2fly
  }

  async start(enableId: string) {
    this.stop()

    const conf = await generateV2flyConfig(this.confService.config, enableId)
    await fs.writeFile(v2flyConfPath, JSON.stringify(conf))

    const bin = this.v2fly.bin

    this.progress = spawn(bin, ['-c', v2flyConfPath], {
      env: process.env,
    })

    this.progress.stdout?.on('data', (message) => {
      const str = message.toString()
      rpcMainProxy.v2flyLog(str)
    })

    this.progress.stderr?.on('data', (message) => {
      const str = message.toString()
      rpcMainProxy.v2flyLog(str)
    })
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

  const conf = getV2rayConfig(
    {
      proxy: {
        http: {
          host: config.v2fly.http.address,
          port: config.v2fly.http.port,
        },
        socks: {
          host: config.v2fly.socks.address,
          port: config.v2fly.socks.port,
        },
      },
    },
    outbound.config
  )

  return conf
}

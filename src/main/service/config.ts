import { homedir } from 'os'
import { join } from 'path'
import { E2FlyConfig } from '../config'
import fs from 'fs-extra'
import { deepMerge } from '@0x-jerry/utils'

export const configDir = join(homedir(), '.e2fly')
export const configPath = join(configDir, 'config.json')

async function createDefaultConfig() {
  const conf: E2FlyConfig = {
    proxy: {
      system: false,
      bypassCN: true,
      pac: false,
    },
    activeOutboundId: '',
    v2fly: {
      bin: 'v2ray',
      http: {
        enabled: true,
        address: '127.0.0.1',
        port: 6667,
      },
      socks: {
        enabled: true,
        address: '127.0.0.1',
        port: 6666,
      },
      stream: {
        udp: true,
        tcp: true,
      },
    },
    outbound: [],
  }

  return conf
}

export class ConfigService {
  config!: E2FlyConfig

  async init() {
    this.config = await this.read()
  }

  async save(config: E2FlyConfig = this.config) {
    this.config = config

    await fs.ensureDir(configDir)

    await fs.writeFile(configPath, JSON.stringify(config, null, 2))
  }

  async read(): Promise<E2FlyConfig> {
    const conf = await createDefaultConfig()

    if (!(await fs.pathExists(configPath))) {
      return conf
    }

    try {
      const confTxt = await fs.readFile(configPath, { encoding: 'utf-8' })

      return deepMerge(conf, JSON.parse(confTxt))
    } catch (error) {
      //
    }

    return conf
  }
}

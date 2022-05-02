import { homedir } from 'os'
import { join } from 'path'
import { E2FlyConfig } from '../config'
import fs from 'fs-extra'

export const configDir = join(homedir(), '.e2fly')
export const configPath = join(configDir, 'config.json')

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
    const defaultConfig: E2FlyConfig = {
      proxy: {},
      v2fly: {
        bin: '',
        inbound: [],
      },
      outbound: [],
    }

    if (!(await fs.pathExists(configPath))) {
      return defaultConfig
    }

    try {
      const confTxt = await fs.readFile(configPath, { encoding: 'utf-8' })
      return JSON.parse(confTxt)
    } catch (error) {
      //
    }

    return defaultConfig
  }
}

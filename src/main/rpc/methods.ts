import { E2FlyConfig } from '../config'
import { services } from '../service'

export type MainThreadMethods = typeof methods

export const methods = {
  async getConfig() {
    return services.config.read()
  },

  async saveConfig(config: E2FlyConfig) {
    await services.config.save(config)
  },
}

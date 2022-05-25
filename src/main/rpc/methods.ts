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

  async startV2fly(id: string) {
    await services.v2fly.start(id)
  },

  stopV2fly() {
    services.v2fly.stop()
  },

  started() {
    services.v2fly.confService
  },

  isEnabled() {
    return services.v2fly.isEnabled
  },
}

import { AppConfig } from '../config'

/**
 * @todo implement methods
 */
export const rpcProxy = {
  async getConfig() {
    return {}
  },

  async saveConfig(conf: AppConfig) {},
  async startV2fly(id: string) {},
  async stopV2fly() {}
}

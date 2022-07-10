import { invoke } from '@tauri-apps/api'
import { AppConfig } from '../config'

/**
 * @todo implement methods
 */
export const rpcProxy = {
  async getConfig(): Promise<AppConfig> {
    return invoke('read_conf')
  },

  async saveConfig(conf: AppConfig) {
    return invoke('save_config', conf)
  },
  async startV2fly(id: string) {},
  async stopV2fly() {}
}

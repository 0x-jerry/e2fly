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
    return invoke('save_conf', { conf })
  },
  async startV2fly(id: string) {
    return invoke('start_v2ray')
  },
  async stopV2fly() {
    return invoke('stop_v2ray')
  }
}

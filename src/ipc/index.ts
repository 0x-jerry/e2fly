import { getV2rayConfig } from '@/logic/v2fly'
import { store } from '@/store'
import { invoke } from '@tauri-apps/api'
import { AppConfig } from '../config'

/**
 * @todo implement methods
 */
export const ipc = {
  async getConfig(): Promise<AppConfig> {
    return invoke('read_conf')
  },

  async saveConfig(conf: AppConfig) {
    return invoke('save_conf', { conf })
  },
  async startV2fly(id: string) {
    const outbound = store.config.outbound.find((n) => n.id === id)?.config

    if (!outbound) return

    const v2rayConf = await getV2rayConfig(store.config, JSON.parse(outbound))

    await invoke('save_v2ray_conf', { content: JSON.stringify(v2rayConf) })

    const err: string = await invoke('start_v2ray')

    if (err) {
      // toast.warning(err)
    }

    return err
  },
  async stopV2fly() {
    return invoke('stop_v2ray')
  },
  async getV2flyLogs(): Promise<string[]> {
    return invoke('get_v2ray_log')
  },
}

export async function isEnabledAutostart(): Promise<boolean> {
  return await invoke('plugin:autostart|is_enabled')
}

export async function enableAutostart(): Promise<void> {
  await invoke('plugin:autostart|enable')
}

export async function disableAutostart(): Promise<void> {
  await invoke('plugin:autostart|disable')
}

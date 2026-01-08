import { sleep } from '@0x-jerry/utils'
import { invoke } from '@tauri-apps/api/core'
import { getV2rayConfig, type TunConfig } from '@/logic/v2fly'
import { store } from '@/store'
import type { AppConfig } from '../config'

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

    const tunConf = await generateTunConfig()

    const v2rayConf = await getV2rayConfig(
      store.config,
      JSON.parse(outbound),
      tunConf,
    )

    await invoke('save_v2ray_conf', { content: JSON.stringify(v2rayConf) })

    const err: string = await invoke('start_v2ray')

    if (err) {
      return err
    }

    await sleep(1500)
    const logs = await ipc.getV2flyLogs()
    const isStartFailed = logs.at(0)?.startsWith('Failed to start')
    if (isStartFailed) {
      return logs.at(0)
    }

    async function generateTunConfig() {
      const enabledTun = await ipc.isEnabledTunMode()
      if (!enabledTun) {
        return
      }

      const inet = await ipc.getDefaultInterfaceName()
      const conf: TunConfig = {
        interfaceName: inet,
      }

      return conf
    }
  },
  async stopV2fly() {
    return invoke('stop_v2ray')
  },
  async getV2flyLogs(): Promise<string[]> {
    return invoke('get_v2ray_log')
  },
  async updateDatFile(): Promise<void> {
    return invoke('update_xray_dat_data')
  },
  async openLogsFolder(): Promise<void> {
    await invoke('open_logs_folder')
  },
  async getDefaultInterfaceName(): Promise<string> {
    return invoke('get_default_interface_name')
  },
  async toggleTunMode(enable: boolean): Promise<void> {
    if (enable) {
      await invoke('enable_tun_mode')
    } else {
      await invoke('disable_tun_mode')
    }
    await restartV2fly()
  },
  async isEnabledTunMode(): Promise<boolean> {
    return invoke('is_tun_mode_enabled')
  },
}

async function restartV2fly() {
  const conf = await ipc.getConfig()
  if (!conf.active.enabled) {
    return
  }

  await ipc.stopV2fly()
  await ipc.startV2fly(conf.active.outboundId)
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

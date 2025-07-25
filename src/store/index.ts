import type { AppConfig } from '../config'
import { ipc } from '../ipc'

export const store = reactive({
  config: {} as AppConfig,
})

export const actions = {
  async startV2fly(id: string) {
    const err = await ipc.startV2fly(id)
    if (err) {
      return err
    }

    store.config.active.outboundId = id
    store.config.active.enabled = true
    await ipc.saveConfig(toRaw(store.config))
  },
  async saveConfig() {
    await ipc.saveConfig(toRaw(store.config))
  },
  async stopV2fly() {
    await ipc.stopV2fly()

    store.config.active.enabled = false
    await ipc.saveConfig(toRaw(store.config))
  },
}

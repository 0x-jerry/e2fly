import { AppConfig } from '../config'
import { ipc } from '../ipc'

export const store = reactive({
  config: {} as AppConfig,
  logs: [] as { id: string; content: string }[],
  enabled: false
})

export const actions = {
  async startV2fly(id: string) {
    store.config.active.outboundId = id

    await ipc.saveConfig(toRaw(store.config))
    await ipc.startV2fly(id)
    store.enabled = true
  },
  async stopV2fly() {
    await ipc.stopV2fly()
    store.enabled = false
  }
}

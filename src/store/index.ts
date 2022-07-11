import { AppConfig } from '../config'
import { ipc } from '../ipc'

export const store = reactive({
  config: {} as AppConfig,
  logs: [] as { id: string; content: string }[]
})

export const actions = {
  async startV2fly(id: string) {
    await ipc.startV2fly(id)

    store.config.active.outboundId = id
    store.config.active.enabled = true
    await ipc.saveConfig(toRaw(store.config))
  },
  async stopV2fly() {
    await ipc.stopV2fly()

    store.config.active.enabled = false
    await ipc.saveConfig(toRaw(store.config))
  }
}

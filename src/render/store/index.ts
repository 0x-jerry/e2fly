import type { E2FlyConfig } from '@/main/config'
import { rpcProxy } from '../rpc'

export const store = reactive({
  config: {} as E2FlyConfig,
  logs: [] as { id: string; content: string }[],
  enabled: false,
})

export const actions = {
  async startV2fly(id: string) {
    store.config.activeOutboundId = id

    await rpcProxy.saveConfig(toRaw(store.config))
    await rpcProxy.startV2fly(id)
    store.enabled = true
  },
  async stopV2fly() {
    await rpcProxy.stopV2fly()
    store.enabled = false
  },
}

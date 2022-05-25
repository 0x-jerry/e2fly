import type { E2FlyConfig } from '@/main/config'
import { rpcProxy } from '../rpc'

export const store = reactive({
  config: {} as E2FlyConfig,
  logs: [] as string[],
  enabled: false,
})

export const actions = {
  async startV2fly(id: string) {
    store.config.activeOutboundId = id
    store.enabled = true

    await rpcProxy.saveConfig(toRaw(store.config))
    await rpcProxy.startV2fly(id)
  },
  async stopV2fly() {
    store.enabled = false
    await rpcProxy.stopV2fly()
  },
}

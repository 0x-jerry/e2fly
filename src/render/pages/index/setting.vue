<script lang="ts" setup>
import { rpcProxy } from '@/render/rpc'
import { store } from '@/render/store'

const e2fly = reactive(structuredClone(toRaw(store.config.v2fly)))

const data = reactive({
  proxy: store.config.proxy,
})

async function saveConfig() {
  store.config.v2fly = structuredClone(toRaw(e2fly))
  store.config.proxy = structuredClone(toRaw(data.proxy))

  const conf = toRaw(store.config)
  await rpcProxy.saveConfig(conf)

  if (store.enabled) {
    await rpcProxy.startV2fly(conf.activeOutboundId)
  }
}
</script>

<template>
  <k-col class="px-3 py-2" gap="0.5rem">
    <div flex="~">
      <k-checkbox v-model="data.proxy.system" class="flex-1 justify-start">System Proxy</k-checkbox>
    </div>
    <div flex="~">
      <k-checkbox v-model="e2fly.stream.tcp" class="flex-1 justify-start">TCP</k-checkbox>
      <k-checkbox v-model="e2fly.stream.udp" class="flex-1 justify-start">UDP</k-checkbox>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <k-checkbox v-model="e2fly.http.enabled"></k-checkbox>
      <div w="6em" text="right">Http Port：</div>
      <div flex="1">
        <k-input class="w-full" v-model.number="e2fly.http.port" block></k-input>
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <k-checkbox v-model="e2fly.socks.enabled"></k-checkbox>
      <div w="6em" text="right">Socks Port：</div>
      <div flex="1">
        <k-input v-model.number="e2fly.socks.port" class="w-full" block></k-input>
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <div w="7em" text="right">V2Ray Bin：</div>
      <div flex="1">
        <k-input v-model.number="e2fly.bin" class="w-full" block></k-input>
      </div>
    </div>
    <k-row>
      <k-button @click="saveConfig" class="w-full" block> {{ $t('page.setting.save') }}</k-button>
    </k-row>
  </k-col>
</template>

<style></style>

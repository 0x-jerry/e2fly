<script lang="ts" setup>
import { rpcProxy } from '@/render/rpc'
import { store } from '@/render/store'
import { logger } from '@/render/utils'

const data = reactive(structuredClone(toRaw(store.config.v2fly)))

async function saveConfig() {
  store.config.v2fly = structuredClone(toRaw(data))
  const conf = toRaw(store.config)
  logger.log('save config:', conf)
  await rpcProxy.saveConfig(conf)

  if (store.enabled) {
    await rpcProxy.startV2fly(conf.activeOutboundId)
  }
}
</script>

<template>
  <k-col class="px-3 py-2" gap="0.5rem">
    <div flex="~">
      <k-checkbox v-model="data.stream.tcp" class="flex-1 justify-start">TCP</k-checkbox>
      <k-checkbox v-model="data.stream.udp" class="flex-1 justify-start">UDP</k-checkbox>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <k-checkbox v-model="data.http.enabled"></k-checkbox>
      <div w="6em" text="right">Http Port：</div>
      <div flex="1">
        <k-input class="w-full" v-model.number="data.http.port" block></k-input>
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <k-checkbox v-model="data.socks.enabled"></k-checkbox>
      <div w="6em" text="right">Socks Port：</div>
      <div flex="1">
        <k-input v-model.number="data.socks.port" class="w-full" block></k-input>
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <div w="7em" text="right">V2Ray Bin：</div>
      <div flex="1">
        <k-input v-model.number="data.bin" class="w-full" block></k-input>
      </div>
    </div>
    <k-row>
      <k-button @click="saveConfig" class="w-full" block> {{ $t('page.setting.save') }}</k-button>
    </k-row>
  </k-col>
</template>

<style></style>

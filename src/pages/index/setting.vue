<script lang="ts" setup>
import { ipc } from '@/ipc'
import { store } from '@/store'

const e2fly = reactive(structuredClone(toRaw(store.config.v2fly)))

const data = reactive({
  proxy: store.config.proxy
})

async function saveConfig() {
  store.config.v2fly = structuredClone(toRaw(e2fly))
  store.config.proxy = structuredClone(toRaw(data.proxy))

  const conf = toRaw(store.config)
  await ipc.saveConfig(conf)

  if (conf.active.enabled) {
    await ipc.startV2fly(conf.active.outboundId)
  } else {
    await ipc.stopV2fly()
  }
}
</script>

<template>
  <k-col class="px-3 py-2" gap="0.5rem">
    <div flex="~">
      <k-checkbox v-model="data.proxy.system" class="flex-1 justify-start">
        {{ $t('page.setting.system-proxy') }}
      </k-checkbox>
    </div>
    <div flex="~">
      <k-checkbox v-model="e2fly.routes.bypassCN" class="flex-1 justify-start">
        {{ $t('page.setting.bypassCN') }}
      </k-checkbox>
    </div>
    <div flex="~">
      <k-checkbox v-model="e2fly.routes.blockAds" class="flex-1 justify-start">
        {{ $t('page.setting.blockAds') }}
      </k-checkbox>
    </div>
    <div flex="~">
      <k-checkbox v-model="e2fly.stream.tcp" class="flex-1 justify-start"
        >TCP</k-checkbox
      >
      <k-checkbox v-model="e2fly.stream.udp" class="flex-1 justify-start"
        >UDP</k-checkbox
      >
    </div>
    <div class="items-center gap-x-1" flex="~">
      <k-checkbox v-model="e2fly.http.enabled"></k-checkbox>
      <div w="6em" text="right">Http {{ $t('page.setting.port') }}：</div>
      <div flex="1">
        <k-input
          class="w-full"
          v-model.number="e2fly.http.port"
          block
        ></k-input>
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <k-checkbox v-model="e2fly.socks.enabled"></k-checkbox>
      <div w="6em" text="right">Socks {{ $t('page.setting.port') }}：</div>
      <div flex="1">
        <k-input
          v-model.number="e2fly.socks.port"
          class="w-full"
          block
        ></k-input>
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <div w="7em" text="right">{{ $t('page.setting.v2ray-bin') }}：</div>
      <div flex="1">
        <k-input v-model.number="e2fly.bin" class="w-full" block></k-input>
      </div>
    </div>
    <k-row>
      <k-button @click="saveConfig" class="w-full" block>
        {{ $t('page.setting.save') }}</k-button
      >
    </k-row>
  </k-col>
</template>

<style></style>

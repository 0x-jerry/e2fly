<script lang="ts" setup>
import { AppConfig } from '@/config'
import { ipc } from '@/ipc'
import { store } from '@/store'

const appConf = reactive<AppConfig>(structuredClone(toRaw(store.config)))

async function saveConfig() {
  store.config = structuredClone(toRaw(appConf))

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
  <div class="px-3 py-2" gap="0.5rem" flex="~ col">
    <div flex="~">
      <Checkbox v-model="appConf.proxy.system" class="flex-1 justify-start">
        {{ $t('page.setting.system-proxy') }}
      </Checkbox>
    </div>
    <div flex="~">
      <Checkbox v-model="appConf.proxy.lan" class="flex-1 justify-start">
        {{ $t('page.setting.proxy--with-lan') }}
      </Checkbox>
    </div>
    <div flex="~">
      <Checkbox v-model="appConf.app.autoHideWhenBlur" class="flex-1 justify-start">
        {{ $t('page.setting.auto-hide-when-blur') }}
      </Checkbox>
    </div>
    <div flex="~">
      <Checkbox v-model="appConf.app.autoStartup" class="flex-1 justify-start">
        {{ $t('page.setting.auto-startup') }}
      </Checkbox>
    </div>
    <div flex="~">
      <Checkbox v-model="appConf.v2fly.routes.bypassCN" class="flex-1 justify-start">
        {{ $t('page.setting.bypassCN') }}
      </Checkbox>
    </div>
    <div flex="~">
      <Checkbox v-model="appConf.v2fly.routes.blockAds" class="flex-1 justify-start">
        {{ $t('page.setting.blockAds') }}
      </Checkbox>
    </div>
    <div flex="~">
      <Checkbox v-model="appConf.v2fly.stream.tcp" class="flex-1 justify-start">TCP</Checkbox>
      <Checkbox v-model="appConf.v2fly.stream.udp" class="flex-1 justify-start">UDP</Checkbox>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <Checkbox v-model="appConf.v2fly.http.enabled"></Checkbox>
      <div w="6em" text="right">Http {{ $t('page.setting.port') }}：</div>
      <div flex="1">
        <input class="w-full" v-model.number="appConf.v2fly.http.port" block />
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <Checkbox v-model="appConf.v2fly.socks.enabled"></Checkbox>
      <div w="6em" text="right">Socks {{ $t('page.setting.port') }}：</div>
      <div flex="1">
        <input v-model.number="appConf.v2fly.socks.port" class="w-full" block />
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <div w="7em" text="right">{{ $t('page.setting.v2ray-bin') }}：</div>
      <div flex="1">
        <input v-model.number="appConf.v2fly.bin" class="w-full" block />
      </div>
    </div>
    <div>
      <Button @click="saveConfig" class="w-full block">{{ $t('page.setting.save') }}</Button>
    </div>
  </div>
</template>

<style></style>

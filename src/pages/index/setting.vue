<script lang="ts" setup>
import { AppConfig } from '@/config'
import { disableAutostart, enableAutostart, ipc, isEnabledAutostart } from '@/ipc'
import { store } from '@/store'
import { useLoading } from '@0x-jerry/vue-kit'
import { useToast } from 'primevue/usetoast'

const appConf = reactive<AppConfig>(structuredClone(toRaw(store.config)))

const toast = useToast()

const saveConfig = useLoading(async () => {
  store.config = structuredClone(toRaw(appConf))

  const conf = toRaw(store.config)
  await ipc.saveConfig(conf)

  if (conf.active.enabled) {
    const err = await ipc.startV2fly(conf.active.outboundId)
    if (err) {
      toast.add({
        severity: 'error',
        summary: 'Error',
        detail: err,
        life: 5000,
      })
    }
  } else {
    await ipc.stopV2fly()
  }

  if (conf.app.autoStartup !== (await isEnabledAutostart())) {
    if (conf.app.autoStartup) {
      await enableAutostart()
    } else {
      await disableAutostart()
    }
  }
})

const isModified = computed(() => {
  return JSON.stringify(appConf) === JSON.stringify(store.config)
})
</script>

<template>
  <div class="px-3 py-2" gap="0.5rem" flex="~ col">
    <div flex="~">
      <BinaryCheckbox v-model="appConf.proxy.system" class="flex-1 justify-start">
        {{ $t('page.setting.system-proxy') }}
      </BinaryCheckbox>
    </div>
    <div flex="~">
      <BinaryCheckbox v-model="appConf.proxy.lan" class="flex-1 justify-start">
        {{ $t('page.setting.proxy--with-lan') }}
      </BinaryCheckbox>
    </div>
    <div flex="~">
      <BinaryCheckbox v-model="appConf.app.autoHideWhenBlur" class="flex-1 justify-start">
        {{ $t('page.setting.auto-hide-when-blur') }}
      </BinaryCheckbox>
    </div>
    <div flex="~">
      <BinaryCheckbox v-model="appConf.app.autoStartup" class="flex-1 justify-start">
        {{ $t('page.setting.auto-startup') }}
      </BinaryCheckbox>
    </div>
    <div flex="~">
      <BinaryCheckbox v-model="appConf.v2fly.routes.bypassCN" class="flex-1 justify-start">
        {{ $t('page.setting.bypassCN') }}
      </BinaryCheckbox>
    </div>
    <div flex="~">
      <BinaryCheckbox v-model="appConf.v2fly.routes.blockAds" class="flex-1 justify-start">
        {{ $t('page.setting.blockAds') }}
      </BinaryCheckbox>
    </div>
    <div flex="~">
      <BinaryCheckbox v-model="appConf.v2fly.stream.tcp" class="flex-1 justify-start">
        TCP
      </BinaryCheckbox>
      <BinaryCheckbox v-model="appConf.v2fly.stream.udp" class="flex-1 justify-start">
        UDP
      </BinaryCheckbox>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <BinaryCheckbox v-model="appConf.v2fly.http.enabled"></BinaryCheckbox>
      <div w="6em" text="right">Http {{ $t('page.setting.port') }}：</div>
      <div flex="1">
        <InputText class="w-full" v-model.number="appConf.v2fly.http.port" block />
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <BinaryCheckbox v-model="appConf.v2fly.socks.enabled"></BinaryCheckbox>
      <div w="6em" text="right">Socks {{ $t('page.setting.port') }}：</div>
      <div flex="1">
        <InputText v-model.number="appConf.v2fly.socks.port" class="w-full" block />
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <div w="7.8em" text="right">{{ $t('page.setting.v2ray-bin') }}：</div>
      <div flex="1">
        <InputText v-model.number="appConf.v2fly.bin" class="w-full" block />
      </div>
    </div>
    <div>
      <Button
        @click="saveConfig"
        class="w-full"
        :disabled="isModified"
        :loading="saveConfig.isLoading"
        :label="$t('page.setting.save')"
      />
    </div>
  </div>
</template>

<style></style>

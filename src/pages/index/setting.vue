<script lang="ts" setup>
import { useLoading } from '@0x-jerry/vue-kit'
import { event } from '@tauri-apps/api'
import { MessagePlugin } from 'tdesign-vue-next'
import { useI18n } from 'vue-i18n'
import { AppConfig } from '@/config'
import { useConfigChangedEvent } from '@/events'
import {
  disableAutostart,
  enableAutostart,
  ipc,
  isEnabledAutostart,
} from '@/ipc'
import { store } from '@/store'

const { t } = useI18n()

interface DownloadProgressEventPayload {
  name: string
  total: number
  downloaded: number
}

const downloadProgressPayload = ref<DownloadProgressEventPayload>()

event.listen<DownloadProgressEventPayload>('download-progress', (event) => {
  if (event.payload.total === event.payload.downloaded) {
    downloadProgressPayload.value = undefined
  } else {
    downloadProgressPayload.value = event.payload
  }
})

const appConf = reactive<AppConfig>(structuredClone(toRaw(store.config)))

const isTunModeEnabled = ref(false)

onMounted(async () => {
  await updateTunModeStatus()
})

useConfigChangedEvent().on(() => {
  Object.assign(appConf, structuredClone(toRaw(store.config)))
})

const saveConfig = useLoading(async () => {
  store.config = structuredClone(toRaw(appConf))

  const conf = toRaw(store.config)
  await ipc.saveConfig(conf)

  if (conf.active.enabled) {
    const err = await ipc.startV2fly(conf.active.outboundId)
    if (err) {
      MessagePlugin.error({ content: err, duration: 5000 })
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

const updateDatFile = useLoading(async () => {
  try {
    await ipc.updateDatFile()
    MessagePlugin.success({
      content: t('page.setting.update-dat-success'),
      duration: 5000,
    })
  } catch (error) {
    MessagePlugin.error({ content: String(error), duration: 5000 })
  }
})

const btnText = computed(() => {
  const payload = downloadProgressPayload.value

  if (!payload) {
    return t('page.setting.update-dat')
  }

  return `${payload.name}: ${payload.downloaded} / ${payload.total}`
})

const toggleTunMode = useLoading(async () => {
  await ipc.toggleTunMode(!isTunModeEnabled.value)
  await updateTunModeStatus()
})

async function updateTunModeStatus() {
  isTunModeEnabled.value = await ipc.isEnabledTunMode()
}
</script>

<template>
  <div class="px-3 py-2" gap="0.5rem" flex="~ col">
    <div flex="~">
      <t-checkbox
        v-model="appConf.proxy.system"
        class="flex-1 justify-start"
      >
        {{ $t('page.setting.system-proxy') }}
      </t-checkbox>
    </div>
    <div flex="~">
      <t-checkbox v-model="appConf.proxy.lan" class="flex-1 justify-start">
        {{ $t('page.setting.proxy--with-lan') }}
      </t-checkbox>
    </div>
    <div flex="~">
      <t-checkbox
        v-model="appConf.app.autoStartup"
        class="flex-1 justify-start"
      >
        {{ $t('page.setting.auto-startup') }}
      </t-checkbox>
    </div>
    <div flex="~">
      <t-checkbox
        v-model="appConf.v2fly.routes.bypassCN"
        class="flex-1 justify-start"
      >
        {{ $t('page.setting.bypassCN') }}
      </t-checkbox>
    </div>
    <div flex="~">
      <t-checkbox
        v-model="appConf.v2fly.routes.blockAds"
        class="flex-1 justify-start"
      >
        {{ $t('page.setting.blockAds') }}
      </t-checkbox>
    </div>
    <div flex="~">
      <t-checkbox
        v-model="appConf.v2fly.stream.tcp"
        class="flex-1 justify-start"
      >
        TCP
      </t-checkbox>
      <t-checkbox
        v-model="appConf.v2fly.stream.udp"
        class="flex-1 justify-start"
      >
        UDP
      </t-checkbox>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <t-checkbox v-model="appConf.v2fly.http.enabled"></t-checkbox>
      <div w="6em" text="right">Http {{ $t('page.setting.port') }}：</div>
      <div flex="1">
        <t-input
          class="w-full"
          type="number"
          v-model="appConf.v2fly.http.port"
        />
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <t-checkbox v-model="appConf.v2fly.socks.enabled"></t-checkbox>
      <div w="6em" text="right">Socks {{ $t('page.setting.port') }}：</div>
      <div flex="1">
        <t-input
          class="w-full"
          type="number"
          v-model="appConf.v2fly.socks.port"
        />
      </div>
    </div>
    <div class="items-center gap-x-1" flex="~">
      <div w="7.8em" text="right">{{ $t('page.setting.v2ray-bin') }}：</div>
      <div flex="1">
        <t-input v-model="appConf.v2fly.bin" class="w-full" />
      </div>
    </div>
    <div>
      <t-button
        block
        @click="updateDatFile"
        :disabled="updateDatFile.isLoading || !!downloadProgressPayload"
        :loading="updateDatFile.isLoading || !!downloadProgressPayload"
      >
        {{ btnText }}
      </t-button>
    </div>
    <div>
      <t-button
        block
        @click="saveConfig"
        :disabled="isModified"
        :loading="saveConfig.isLoading"
      >
        {{ $t('page.setting.save') }}
      </t-button>
    </div>
    <div>
      <t-button
        block
        @click="toggleTunMode"
        :disabled="toggleTunMode.isLoading"
        :theme="isTunModeEnabled ? 'danger' : 'primary'"
        :loading="toggleTunMode.isLoading"
      >
        {{ isTunModeEnabled ? 'Disable TUN Mode' : 'Enable TUN Mode' }}
      </t-button>
    </div>
  </div>
</template>

<style></style>

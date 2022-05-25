<script lang="ts" setup>
import { E2FlyConfigOutbound } from '@/main/config'
import { getOutboundConfFromBase64 } from '@/render/logic/v2fly'
import { rpcProxy } from '@/render/rpc'
import { actions, store } from '@/render/store'
import { uuid } from '@0x-jerry/utils'
import { IV2RayOutbound } from '@0x-jerry/v2ray-schema'
import CircleIcon from '~icons/carbon/circle-filled'

const data = reactive({
  showConfig: false,
})

const v2flyConf = reactive({
  b64: '',
  mux: false,
  label: '',
})

function showConfigDrawer() {
  v2flyConf.label = 'conf' + (store.config.outbound.length + 1)
  data.showConfig = true
}

async function addConfig() {
  store.config.outbound.push({
    id: uuid(),
    label: v2flyConf.label,
    config: getOutboundConfFromBase64({
      b64: v2flyConf.b64,
      mux: v2flyConf.mux,
    }),
  })

  await rpcProxy.saveConfig(toRaw(store.config))
  data.showConfig = false
}

async function toggleV2fly() {
  if (store.enabled) {
    await actions.stopV2fly()
  } else {
    await actions.startV2fly(store.config.activeOutboundId)
  }
}

async function switchConfig(item: E2FlyConfigOutbound) {
  if (store.enabled && item.id === store.config.activeOutboundId) return

  await actions.startV2fly(item.id)
}

function getLabel(item: IV2RayOutbound) {
  const protocol = item.protocol

  if (protocol === 'vmess') {
    const address = item.settings?.vnext?.[0].address
    const port = item.settings?.vnext?.[0].port

    return `${protocol}://${address}:${port}`
  }
}

function isActiveOutboundConfig(item: E2FlyConfigOutbound) {
  return store.enabled && store.config.activeOutboundId === item.id
}
</script>

<template>
  <div>
    <div
      @click="toggleV2fly"
      p="y-1"
      class="connection-btn bg-red-400 text-white text-center cursor-pointer"
      bg="hover:red-500"
      :class="{ 'is-disabled': !store.enabled }"
    >
      {{ store.enabled ? '断开连接' : '重新连接' }}
    </div>
    <div
      class="cards"
      grid="~"
      style="grid-template-columns: repeat(auto-fit, minmax(200px, 1fr))"
      gap="x-2 y-1"
    >
      <div
        v-for="item in store.config.outbound"
        :key="item.id"
        class="v2fly-card"
        :class="{
          'is-active': isActiveOutboundConfig(item),
        }"
        @click="switchConfig(item)"
      >
        <div flex="1">{{ getLabel(item.config) }}</div>
        <div v-if="isActiveOutboundConfig(item)" class="text-xs items-center" flex="~">
          <CircleIcon />
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="less" scoped>
.add-card {
  border: 1px solid;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;

  @apply p-2;
  @apply border-gray-300 rounded;
  @apply hover:border-blue-500;
}

.cards {
  @apply my-1;
}

.v2fly-card {
  @apply px-3 py-1;
  @apply bg-gray-100 text-gray-700;
  @apply text-sm;
  @apply flex;
  cursor: pointer;

  &.is-selected {
    //
  }

  &.is-active {
    @apply text-green-600;
  }
}

.connection-btn {
  &.is-disabled {
    @apply bg-green-500;
  }
}
</style>

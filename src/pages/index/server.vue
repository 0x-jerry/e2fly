<script lang="ts" setup>
import { getOutboundConfFromBase64 } from '@/logic/v2fly'
import { ipc } from '@/ipc'
import { actions, store } from '@/store'
import { remove, uuid } from '@0x-jerry/utils'
import type { V4 } from '@0x-jerry/v2ray-schema'
import { Outbound } from '@/config'

const v2flyConf = reactive({
  b64: '',
  mux: false,
})

async function addConfig() {
  store.config.outbound.push({
    id: uuid(),
    label: 'default',
    config: JSON.stringify(
      getOutboundConfFromBase64({
        b64: v2flyConf.b64,
        mux: v2flyConf.mux,
      }),
    ),
  })

  await ipc.saveConfig(toRaw(store.config))

  v2flyConf.b64 = ''
}

async function toggleV2fly() {
  if (store.config.active.enabled) {
    await actions.stopV2fly()
  } else {
    await actions.startV2fly(store.config.active.outboundId)
  }
}

type E2FlyConfigOutbound = any

async function switchConfig(item: E2FlyConfigOutbound) {
  if (store.config.active.enabled && item.id === store.config.active.outboundId) return

  await actions.startV2fly(item.id)
}

function getLabel(itemConf: string) {
  const item: V4.outbounds.OutboundObject = JSON.parse(itemConf)
  const protocol = item.protocol

  if (protocol === 'vmess') {
    const address = item.settings?.vnext?.[0].address
    const port = item.settings?.vnext?.[0].port

    return `${protocol}://${address}:${port}`
  }
}

function isActiveOutboundConfig(item: E2FlyConfigOutbound) {
  return store.config.active.enabled && store.config.active.outboundId === item.id
}

function removeOutbound(item: E2FlyConfigOutbound) {
  remove(store.config.outbound, (n) => n.id === item.id)
  ipc.saveConfig(toRaw(store.config))
}

const preview = reactive({
  show: false,
  id: '',
  content: '',
})

function showConfig(item: Outbound) {
  preview.id = item.id
  preview.content = JSON.stringify(JSON.parse(item.config), null, 2)
  preview.show = true
}

async function saveCurrentConfig() {
  const resolved = store.config.outbound.find((n) => n.id === preview.id)
  if (!resolved) return

  resolved.config = preview.content

  await actions.saveConfig()
}
</script>

<template>
  <div class="flex flex-col">
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
        <div class="icons text-xs items-center" flex="~">
          <i-carbon-code class="icon" @click.stop="showConfig(item)" />
          <i-carbon-trash-can
            class="icon"
            v-if="!isActiveOutboundConfig(item)"
            @click.stop="removeOutbound(item)"
          />
          <i-carbon-circle-filled class="text-green-500" v-if="isActiveOutboundConfig(item)" />
        </div>
      </div>
    </div>

    <div
      @click="toggleV2fly"
      p="y-1"
      class="connection-btn bg-red-400 text-white text-center cursor-pointer"
      bg="hover:red-500"
      :class="{ 'is-disabled': !store.config.active.enabled }"
    >
      {{ store.config.active.enabled ? $t('page.server.disconnect') : $t('page.server.reconnect') }}
    </div>
    <textarea
      class="w-full border-gray-300 bg-gray-100 resize-y outline-none border-x-0 text-sm px-3"
      rows="6"
      :placeholder="$t('page.server.link-placeholder')"
      v-model="v2flyConf.b64"
    ></textarea>
    <div class="flex px-4">
      <k-checkbox v-model="v2flyConf.mux">Mux</k-checkbox>
    </div>
    <button class="w-full" block @click="addConfig">{{ $t('page.server.add') }}</button>
  </div>

  <k-drawer v-model="preview.show" width="300px" placement="left">
    <div class="flex flex-col h-full">
      <div class="border-(0 b solid gray-2) pb-1">
        <button @click="saveCurrentConfig">Save</button>
      </div>
      <MonacoEditor class="flex-1" v-model="preview.content"></MonacoEditor>
    </div>
  </k-drawer>
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
  // @apply my-1;
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
    //
  }

  .icon {
    opacity: 0;
    @apply transition transition-opacity;

    &:hover {
      @apply text-blue-500;
    }
  }

  &:hover {
    .icon {
      opacity: 1;
    }
  }
}

.connection-btn {
  &.is-disabled {
    @apply bg-green-500;
  }
}
</style>

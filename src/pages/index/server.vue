<script lang="ts" setup>
import { getOutboundConfFromBase64 } from '@/logic/v2fly'
import { ipc } from '@/ipc'
import { actions, store } from '@/store'
import { nanoid, remove } from '@0x-jerry/utils'
import type { V4 } from '@0x-jerry/v2ray-schema'
import { Outbound } from '@/config'
import { version } from '../../../package.json'
import { useToast } from 'primevue/usetoast'
import { useLoading } from '@0x-jerry/vue-kit'
import LoadingPanel from '@/components/LoadingPanel.vue'
import Drawer from 'primevue/drawer'

const toast = useToast()

const v2flyConf = reactive({
  b64: '',
  mux: false,
})

async function addConfig() {
  store.config.outbound.push({
    id: nanoid(),
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

const toggleV2fly = useLoading(async () => {
  if (store.config.active.enabled) {
    await actions.stopV2fly()
  } else {
    const err = await actions.startV2fly(store.config.active.outboundId)

    if (err) {
      toast.add({
        severity: 'error',
        summary: 'Error',
        detail: err,
        life: 5000,
      })
    }
  }
})

type E2FlyConfigOutbound = any

const switchConfig = useLoading(async (item: E2FlyConfigOutbound) => {
  if (store.config.active.enabled && item.id === store.config.active.outboundId) return

  const err = await actions.startV2fly(item.id)

  if (err) {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: err,
      life: 5000,
    })
  }
})

function getLabel(itemConf: string) {
  const item: V4.outbounds.OutboundObject = JSON.parse(itemConf)
  const protocol = item.protocol

  if (protocol === 'vmess' || protocol === 'vless') {
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

async function copyConfig(item: Outbound) {
  const conf = structuredClone(toRaw(item))
  conf.id = nanoid()

  store.config.outbound.push(conf)

  await ipc.saveConfig(toRaw(store.config))
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
    <div class="cards" grid="~" style="grid-template-columns: repeat(auto-fit, minmax(200px, 1fr))" gap="x-2 y-1">
      <LoadingPanel :loading="switchConfig.isLoading">
        <div v-for="item in store.config.outbound" @click="switchConfig(item)" :key="item.id" class="v2fly-item-btn"
          :class="{
            'is-active': isActiveOutboundConfig(item),
          }">
          <div flex="1">{{ getLabel(item.config) }}</div>
          <div class="flex gap-2 text-xs items-center" flex="~">
            <i-carbon-code class="icon" @click.stop="showConfig(item)" />
            <i-carbon-copy class="icon" @click.stop="copyConfig(item)" />
            <i-carbon-trash-can class="icon" v-if="!isActiveOutboundConfig(item)" @click.stop="removeOutbound(item)" />
            <i-carbon-circle-filled class="text-green-500" v-if="isActiveOutboundConfig(item)" />
          </div>
        </div>
      </LoadingPanel>
    </div>

    <Button @click="toggleV2fly" border="rounded-0" :loading="toggleV2fly.isLoading"
      :severity="!store.config.active.enabled ? 'primary' : 'danger'" :label="store.config.active.enabled ? $t('page.server.disconnect') : $t('page.server.reconnect')
        " />
    <textarea class="w-full border-gray-300 bg-gray-100 resize-y outline-none border-x-0 text-sm px-3" rows="6"
      :placeholder="$t('page.server.link-placeholder')" v-model="v2flyConf.b64"></textarea>
    <div class="px-4 my-2">
      <BinaryCheckbox v-model="v2flyConf.mux">Mux</BinaryCheckbox>
    </div>
    <Button severity="secondary" class="w-full rounded-0 block" @click="addConfig">
      {{ $t('page.server.add') }}
    </Button>
    <div class="mt-3" text="xs gray-3 center">version: v{{ version }}</div>
  </div>

  <Drawer v-model:visible="preview.show" position="full" :pt="{
    content: 'p-0',
    header: 'py-1',
  }">
    <template #header>
      <div class="flex gap-2 items-center">
        <h2 class="m-0">Edit Config</h2>
        <Button size="small" @click="saveCurrentConfig">Save</Button>
      </div>
    </template>
    <div class="flex flex-col h-full b-(0 t solid gray-3)">
      <MonacoEditor class="flex-1" v-model="preview.content"></MonacoEditor>
    </div>
  </Drawer>
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

.v2fly-item-btn {
  @apply px-3 py-2;
  @apply bg-gray-100 text-gray-700;
  @apply text-sm;
  @apply flex;
  outline: none;
  border: none;
  cursor: pointer;
  text-align: left;

  &.is-selected {
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

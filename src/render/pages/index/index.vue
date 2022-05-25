<script lang="ts" setup>
import { getOutboundConfFromBase64 } from '@/render/logic/v2fly'
import { rpcProxy } from '@/render/rpc'
import { actions, store } from '@/render/store'
import { uuid } from '@0x-jerry/utils'

const data = reactive({
  showConfig: false,
  activeId: store.config.activeOutboundId,
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
    await actions.startV2fly(data.activeId)
  }
}
</script>

<template>
  <div>
    <AppHeadAppend>
      <k-row>
        <k-button @click="showConfigDrawer">添加配置</k-button>
        <k-button @click="toggleV2fly">{{ store.enabled ? 'disable' : 'enable' }}</k-button>
      </k-row>
    </AppHeadAppend>
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
          'is-active': store.config.activeOutboundId === item.id,
          'is-selected': data.activeId === item.id,
        }"
        @click="data.activeId = item.id"
      >
        <div>
          {{ item.label }}
        </div>
        <div class="truncate">{{ item.id }}</div>
      </div>

      <!-- Empty div, as placeholder -->
      <div v-for="o in 3"></div>
    </div>

    <k-drawer v-model="data.showConfig" title="添加配置">
      <e2fly-config></e2fly-config>
    </k-drawer>
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

.v2fly-card {
  border: 1px solid;
  @apply p-5 border-gray-500;
  cursor: pointer;

  &.is-selected {
    @apply border-blue-300;
  }

  &.is-active {
    @apply border-blue-500;
  }
}
</style>

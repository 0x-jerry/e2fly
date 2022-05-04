<script lang="ts" setup>
import { getOutboundConfFromBase64 } from '@/render/logic/v2fly'
import { rpcProxy } from '@/render/rpc'
import { store } from '@/render/store'
import { uuid } from '@0x-jerry/utils'
import AppHeadAppend from '../components/AppHeadAppend.vue'
const data = reactive({
  showConfig: false,
  activeId: '',
})

function showConfigDrawer() {
  data.showConfig = true
}

const v2flyConf = reactive({
  b64: '',
  mux: false,
})

async function addConfig() {
  store.config.outbound.push({
    id: uuid(),
    label: 'test',
    config: getOutboundConfFromBase64({
      b64: v2flyConf.b64,
      mux: v2flyConf.mux,
    }),
  })

  await rpcProxy.saveConfig(toRaw(store.config))
}

async function startV2fly() {
  await rpcProxy.startV2fly(data.activeId)
}
</script>

<template>
  <div>
    <AppHeadAppend>
      <k-button @click="showConfigDrawer">添加配置</k-button>
      <k-button @click="startV2fly">启动</k-button>
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
        :class="{ active: data.activeId === item.id }"
        @click="data.activeId = item.id"
      >
        {{ item.label }} @ {{ item.id }}
      </div>
    </div>

    <k-drawer v-model="data.showConfig" title="添加配置">
      <k-col>
        <div>Import</div>
        <k-input v-model="v2flyConf.b64"></k-input>
        <!--  -->
        <k-checkbox v-model="v2flyConf.mux">Enable mux</k-checkbox>
        <!--  -->
        <k-button @click="addConfig">确认</k-button>
      </k-col>
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
  &.active {
    border: 1px solid;
    @apply border-blue-500;
  }
}
</style>

<script lang="ts" setup>
import AppHeadAppend from '../components/AppHeadAppend.vue'
const data = reactive({
  showConfig: false,
})

function showConfigDrawer() {
  data.showConfig = true
}
</script>

<template>
  <div>
    <AppHeadAppend>
      <k-button @click="showConfigDrawer">添加配置</k-button>
    </AppHeadAppend>
    <div
      class="cards"
      grid="~"
      style="grid-template-columns: repeat(auto-fit, minmax(200px, 1fr))"
      gap="x-2 y-1"
    >
      <!--  -->
    </div>

    <k-drawer v-model="data.showConfig" title="添加配置">
      <div>Hello</div>
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
  //
}
</style>

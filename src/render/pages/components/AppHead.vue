<script lang="ts" setup>
import { menus } from './menuConf'

const route = useRoute()

const activeMenu = computed(() => menus.find((n) => n.route === route.path))
</script>

<template>
  <div class="app-head">
    <div flex="~ 1">
      <h1 text="xl">
        {{ activeMenu?.text }}
      </h1>
    </div>
    <div id="app-head--append">
      <!--  -->
    </div>
  </div>
</template>

<style lang="less" scoped>
.app-head {
  height: 50px;
  border-width: 0 0 1px;
  border-style: solid;
  @apply bg-light-200 border-light-700;
  @apply px-3;
  @apply flex items-center;
}
</style>

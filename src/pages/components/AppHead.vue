<script lang="ts" setup>
import { MenuItem, menus } from './menuConf'

const router = useRouter()
const route = useRoute()

const activeMenu = computed(() => menus.find((n) => n.route === route.path))

function handleMenu(menu: MenuItem) {
  router.push(menu.route)
}
</script>

<template>
  <div class="app-head">
    <div flex="~ 1">
      <span>
        {{ activeMenu?.text }}
      </span>
    </div>
    <div class="flex gap-2">
      <div
        v-for="menu in menus"
        :key="menu.route"
        class="cursor-pointer"
        :class="{ 'text-blue-500': activeMenu?.route === menu.route }"
        @click="handleMenu(menu)"
      >
        <component :is="menu.icon"></component>
      </div>
    </div>
  </div>
</template>

<style lang="less" scoped>
.app-head {
  -webkit-app-region: drag;

  height: 40px;
  border-width: 0 0 1px;
  border-style: solid;
  @apply bg-light-200 border-light-700;
  @apply px-3;
  @apply flex items-center;
}
</style>

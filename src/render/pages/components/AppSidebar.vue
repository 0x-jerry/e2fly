<script setup lang="ts">
import { reactive } from 'vue'
import { menus, MenuItem } from './menuConf'

const data = reactive({
  expand: false,
})

const router = useRouter()
const route = useRoute()

function handleMenuClick(menu: MenuItem) {
  if (menu.route) {
    router.push(menu.route)
  }

  menu.onClick?.()
}

const isActive = (menu: MenuItem) => route.path === menu.route
</script>

<template>
  <div
    class="app-header"
    bg="light-500"
    flex="~ col"
    p="y-3 x-4"
    border="x-0 y-0 r solid light-800"
  >
    <div flex="1 ~ col" gap="y-3">
      <k-row
        class="menu-item items-center"
        :class="{ 'is-active': isActive(menu) }"
        v-for="menu in menus"
        @click="handleMenuClick(menu)"
        :title="menu.text"
      >
        <div class="menu-item--icon">
          <component :is="menu.icon"></component>
        </div>
        <div class="menu-item--text" v-if="data.expand">
          {{ menu.text }}
        </div>
      </k-row>
    </div>
    <!--  -->

    <k-row class="menu-item items-center" @click="data.expand = !data.expand">
      <div class="menu-item--icon">
        <i-ep-fold v-if="data.expand"></i-ep-fold>
        <i-ep-expand v-else></i-ep-expand>
      </div>
      <div class="menu-item--text" v-if="data.expand">收起</div>
    </k-row>
  </div>
</template>

<style lang="less" scoped>
.app-header {
  -webkit-app-region: drag;
}

.menu-item {
  @apply text-gray-500;
  @apply transition transition-colors;

  cursor: pointer;

  &.is-active,
  &:hover {
    @apply text-blue-500;
  }

  &--icon {
    @apply text-xl;
  }

  &--text {
    @apply mx-3;
  }
}
</style>

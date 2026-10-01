<script lang="ts" setup>
import { menus } from './menuConf'

const router = useRouter()
const route = useRoute()

const activeMenu = computed(() => menus.find((n) => n.route === route.path))

function handleMenu(value: string | number | boolean) {
  router.push(String(value))
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
      <t-radio-group
        :model-value="activeMenu?.route"
        theme="button"
        variant="default-filled"
        @change="handleMenu"
      >
        <t-radio-button
          v-for="menu in menus"
          :key="menu.route"
          :value="menu.route"
        >
          <component :is="menu.icon"></component>
        </t-radio-button>
      </t-radio-group>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.app-head {
  // -webkit-app-region: drag;
  background: #f1f5f9;

  --uno: pl-3;
  --uno: flex items-center;

  --uno: border-(0 b solid gray-3);
}

:deep(.t-radio-button__label) {
  display: inline-flex;
}
</style>

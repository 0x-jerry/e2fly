<script lang="ts" setup>
import { SelectButtonChangeEvent } from 'primevue/selectbutton'
import { menus } from './menuConf'

const router = useRouter()
const route = useRoute()

const activeMenu = computed(() => menus.find((n) => n.route === route.path))

function handleMenu(menu: SelectButtonChangeEvent) {
  const route = menu.value

  router.push(route)
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
      <SelectButton :model-value="activeMenu?.route" :options="menus" option-label="text" option-value="route" @change="handleMenu"
        #option="{ option }">
        <component :is="option.icon"></component>
      </SelectButton>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.app-head {
  // -webkit-app-region: drag;
  background: #f1f5f9;

  @apply pl-3;
  @apply flex items-center;

  @apply border-(0 b solid gray-3);
}
</style>

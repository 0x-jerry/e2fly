<script setup lang="ts">
import Toast from 'primevue/toast'
import { useConfigChangedEvent } from './events'
import { useEvent } from './hooks/useEvent'
import { ipc } from './ipc'
import { store } from './store'
import { logger } from './utils'

const initialized = ref(false)

async function init() {
  await reloadConfig()
  initialized.value = true
}

const configChangedEvent = useConfigChangedEvent()

async function reloadConfig() {
  const conf = await ipc.getConfig()
  logger.log('load config:', conf)

  store.config = conf

  configChangedEvent.emit()
}

useEvent('config-changed', () => reloadConfig())

init()
</script>

<template>
  <Toast position="bottom-center"></Toast>
  <router-view v-if="initialized"></router-view>
</template>

<style lang="scss">
hr {
  border-width: 0 0 1px;
  @apply border-light-900 !my-3;
}

html,
body {
  @apply font-sans;
}

html {
  font-size: 16px;
}

* {
  box-sizing: border-box;
}

.btn {
  @apply border-none px-2 py-1 bg-gray-2;
  cursor: pointer;

  &:hover {
    @apply bg-gray-3;
  }

  &:active {
    @apply bg-gray-2;
  }
}
</style>

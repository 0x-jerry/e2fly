<script setup lang="ts">
import { rpcProxy } from './rpc'
import { store } from './store'
import { logger } from './utils'

const initialized = ref(false)

async function init() {
  const conf = await rpcProxy.getConfig()
  logger.log('load config:', conf)

  store.config = conf
  initialized.value = true
}

init()
</script>

<template>
  <router-view v-if="initialized"></router-view>
</template>

<style lang="less">
hr {
  border-width: 0 0 1px;
  @apply border-light-900 !my-3;
}

html,
body {
  @apply font-sans;
}
</style>

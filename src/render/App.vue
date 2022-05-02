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
  @apply border-gray-200 !my-3;
}

div {
  @size: 8px;
  &::-webkit-scrollbar {
    width: @size;
    @apply bg-light-400;
  }

  &::-webkit-scrollbar-thumb {
    width: @size;
    @apply bg-light-800;
  }
}
</style>

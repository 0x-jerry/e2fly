<script lang="ts" setup>
import { ipc } from '@/ipc'
import { useInterval } from '@/hooks'
import { store } from '@/store'

async function getLogs() {
  const log = await ipc.getV2flyLogs()

  const logs = log
    .split(/\n/g)
    .filter((n) => !!n.trim())
    .map((n) => n + '\n')

  store.logs.unshift(
    ...logs
      .map((x) => ({
        id: x,
        content: x
      }))
      .reverse()
  )

  store.logs = store.logs.splice(0, 2000)
}

useInterval(() => getLogs(), 1000)
</script>

<template>
  <div class="log-page">
    <pre><code v-for="o in store.logs" :key="o.id" >{{o.content}}</code></pre>
  </div>
</template>

<style lang="less" scoped>
.log-page {
  @apply bg-light-300;
  height: 100%;
  margin: 0;
  @apply px-3;
  @apply text-gray-700;
  overflow: auto;
  @apply text-xs;
  line-height: 1.7em;
}

pre {
  margin: 0;
  padding: 0;
  width: fit-content;
  height: fit-content;
  user-select: text;
}

code {
  @apply font-mono;
  user-select: text;
}
</style>

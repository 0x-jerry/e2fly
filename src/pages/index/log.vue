<script lang="ts" setup>
import { useInterval } from '@/hooks'
import { ipc } from '@/ipc'

const state = reactive({
  logs: [] as string[],
})

async function getLogs() {
  const logs = await ipc.getV2flyLogs()

  state.logs = logs.map((line) => `${line}\n`)
}

useInterval(() => getLogs(), 1000)

async function openLogFolder() {
  await ipc.openLogsFolder()
}
</script>

<template>
  <div class="log-page flex flex-col">
    <div class="flex gap-1 border-(0 b solid gray-3) mb-1 justify-end">
      <Button border="rounded-0" size="small" severity="secondary" @click="openLogFolder">
        Open Log Folder
      </Button>
    </div>
    <div class="px-3 overflow-auto flex-1">
      <pre><code v-for="line in state.logs" :key="line" >{{ line }}</code></pre>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.log-page {
  @apply bg-light-300;
  height: 100%;
  margin: 0;
  @apply text-gray-700;
  @apply text-xs;
}

pre {
  margin: 0;
  padding: 0;
  width: fit-content;
  height: fit-content;
}

code {
  @apply font-mono;
}
</style>

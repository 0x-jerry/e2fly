<script lang="ts" setup>
import { ipc } from '@/ipc'
import { useInterval } from '@/hooks'
import { path, shell } from '@tauri-apps/api'
import { getLogConf } from '@/logic/v2fly'

interface LogLine {
  id: number
  content: string
}

const state = reactive({
  logs: [] as LogLine[],
})

async function getLogs() {
  const conf = await getLogConf()
  const logs = await ipc.getV2flyLogs(conf.access!)

  state.logs = logs.map((x, idx) => ({
    id: idx,
    content: x + '\n',
  }))
}

useInterval(() => getLogs(), 1000)

async function openLogFolder() {
  const logFolder = await path.appLogDir()
  await shell.open(logFolder)
}
</script>

<template>
  <div class="log-page flex flex-col">
    <div class="flex gap-1 border-(0 b solid gray-2) mb-1 justify-end">
      <button @click="openLogFolder">Open Log Folder</button>
    </div>
    <div class="px-3 overflow-auto flex-1">
      <pre><code v-for="o in state.logs" :key="o.id" >{{o.content}}</code></pre>
    </div>
  </div>
</template>

<style lang="less" scoped>
button {
  background: white;
  border: none;
  cursor: pointer;

  &.active {
    background: rgb(172, 214, 253);
  }
}
.log-page {
  @apply bg-light-300;
  height: 100%;
  margin: 0;
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

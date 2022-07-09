<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

async function startV2ray() {
  await invoke('start_v2ray')
}

async function log() {
  const r: any = await invoke('get_v2ray_log')
  console.log(r)
}
</script>

<template>
  <div>
    <button type="button" @click="startV2ray">start v2ray</button>
    <button type="button" @click="log">get log</button>
  </div>
</template>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>

<script lang="ts" setup>
import { rpcProxy } from '@/render/rpc'
import { store } from '@/render/store'
import { logger } from '@/render/utils'

async function saveConfig() {
  const conf = toRaw(store.config)
  logger.log('save config:', conf)
  await rpcProxy.saveConfig(conf)
}
</script>

<template>
  <k-col>
    <k-row tag="label">
      <div>V2fly 命令：</div>
      <k-input v-model="store.config.v2fly.bin"></k-input>
    </k-row>
    <k-row>
      <k-row tag="label">
        <div>Http Address：</div>
        <k-input v-model="store.config.v2fly.http.address"></k-input>
      </k-row>
      <k-row tag="label">
        <div>Http Port：</div>
        <k-input v-model.number="store.config.v2fly.http.port"></k-input>
      </k-row>
    </k-row>
    <k-row>
      <k-row tag="label">
        <div>Socks Address：</div>
        <k-input v-model="store.config.v2fly.socks.address"></k-input>
      </k-row>
      <k-row tag="label">
        <div>Socks Port：</div>
        <k-input v-model.number="store.config.v2fly.socks.port"></k-input>
      </k-row>
    </k-row>
    <k-row>
      <k-button @click="saveConfig"> 保存</k-button>
    </k-row>
  </k-col>
</template>

<style></style>

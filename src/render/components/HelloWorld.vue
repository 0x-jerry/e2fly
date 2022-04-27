<template>
  <div>
    <button @click="test">test</button>
  </div>
</template>

<script setup lang="ts">
import { rpcProxy } from '../rpc'

async function test() {
  const res = await rpcProxy.ping()
  console.log(res)
}
</script>

<style></style>

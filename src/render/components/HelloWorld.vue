<template>
  <div>
    <button @click="test">test</button>
  </div>
</template>

<script setup lang="ts">
import { rpc } from '../rpc'

async function test() {
  const res = await rpc.ping()
  console.log(res)
}
</script>

<style></style>

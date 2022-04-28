<template>
  <div>
    <button @click="test">test</button>
    <div>response: {{ data.response }}</div>
    <div>response date: {{ data.date }}</div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
import { rpcProxy } from '../rpc'

const data = reactive({
  response: '',
  date: '',
})

async function test() {
  const res = await rpcProxy.ping()
  data.date = new Date().toISOString()
  data.response = res
}
</script>

<style></style>

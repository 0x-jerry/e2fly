<script lang="ts" setup>
import { store } from '@/render/store'

//
</script>

<template>
  <div class="log-page">
    <pre><code v-for="o in store.logs" >{{o}}</code></pre>
  </div>
</template>

<style lang="less" scoped>
.log-page {
  @apply bg-light-300;
  height: 100%;
  margin: 0;
  @apply px-3;
  @apply text-gray-700;
  line-height: 1.5em;
  overflow: auto;
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

<script lang="ts" setup>
import AppHead from './components/AppHead.vue'
</script>

<template>
  <div flex="~ col" h="100vh">
    <app-head></app-head>
    <div flex="1" overflow="auto">
      <router-view></router-view>
    </div>
  </div>
</template>

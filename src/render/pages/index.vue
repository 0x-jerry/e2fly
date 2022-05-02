<script lang="ts" setup>
import AppHead from './components/AppHead.vue'
import AppSidebar from './components/AppSidebar.vue'

//
</script>

<template>
  <div flex="~" h="100vh">
    <app-sidebar></app-sidebar>
    <div flex="1 ~ col">
      <app-head></app-head>
      <div flex="1" overflow="auto" p="x-6 y-3">
        <router-view></router-view>
      </div>
    </div>
  </div>
</template>

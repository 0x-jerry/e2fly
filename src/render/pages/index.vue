<script lang="ts" setup>
import AppSidebar from './components/AppSidebar.vue'

//
</script>

<template>
  <div flex="~" h="100vh">
    <app-sidebar></app-sidebar>
    <div flex="1" overflow="auto" p="x-6 y-4">
      <router-view></router-view>
    </div>
  </div>
</template>

<style lang="less">
.k {
}
</style>

<script lang="ts" setup>
const visible = defineModel('visible')
</script>

<template>
  <div class="drawer-container" :class="{ visible }">
    <div class="drawer-bg" @click="visible = false"></div>
    <div class="drawer-content">
      <slot></slot>
    </div>
  </div>
</template>

<style lang="less" scoped>
.drawer-container {
  position: fixed;
  top: 0;
  bottom: 0;
  display: none;

  &.visible {
    display: block;
  }
}

.drawer-bg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: #000;
  opacity: 0.4;
}

.drawer-content {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100vw;
}
</style>

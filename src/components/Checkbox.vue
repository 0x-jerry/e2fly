<script lang="ts" setup>
const value = defineModel()
</script>

<template>
  <label>
    <input class="mr-1" type="checkbox" v-model="value" />
    <slot></slot>
  </label>
</template>

<style lang="less" scoped></style>

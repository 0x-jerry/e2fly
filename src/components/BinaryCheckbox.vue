<script lang="ts" setup>
const value = defineModel()
</script>

<template>
  <label class="flex items-center cursor-pointer">
    <Checkbox class="mr-1" binary v-model="value"></Checkbox>
    <slot></slot>
  </label>
</template>

<style lang="scss" scoped></style>

<script lang="ts" setup>
export interface LoadingProps {
  loading?: boolean
}

defineProps<LoadingProps>()
</script>

<template>
  <div class="relative">
    <slot></slot>
    <t-loading :loading="loading" class="spinner" />
  </div>
</template>

<style lang="scss" scoped>
.spinner {
  @apply absolute top-0 left-0 size-full flex items-center justify-center;
  @apply bg-gray-4 bg-op-40;
  z-index: 999;
}
</style>

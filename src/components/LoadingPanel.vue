<script lang="ts" setup>
import Spinner from 'primevue/progressspinner'

export interface LoadingProps {
  loading?: boolean
}

defineProps<LoadingProps>()
</script>

<template>
  <div class="relative">
    <slot></slot>
    <div
      v-if="loading"
      class="spinner absolute top-0 left-0 size-full flex items-center justify-center"
    >
      <Spinner class="h-4/5 max-h-8" strokeWidth="6" />
    </div>
  </div>
</template>

<style lang="less" scoped>
.spinner {
  @apply bg-gray-4 bg-op-40;
  z-index: 999;
}
</style>

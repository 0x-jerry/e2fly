<script lang="ts" setup>
const ensureTargetExists = ref(false)

// Ensure #app-head-append element exists.
nextTick().then(() => (ensureTargetExists.value = true))
</script>

<template>
  <teleport v-if="ensureTargetExists" to="#app-head--append">
    <slot></slot>
  </teleport>
</template>

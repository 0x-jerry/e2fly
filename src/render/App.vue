<script setup lang="ts"></script>

<template>
  <router-view></router-view>
</template>

<style lang="less">
hr {
  @apply border-gray-200 !my-3;
}

div {
  @size: 8px;
  &::-webkit-scrollbar {
    width: @size;
    @apply bg-light-400;
  }

  &::-webkit-scrollbar-thumb {
    width: @size;
    @apply bg-light-800;
  }
}
</style>

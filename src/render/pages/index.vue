<script lang="ts" setup>
//
</script>

<template>
  <div flex="~" h="100vh">
    <div w="60px" bg="light-500" flex="~ col" justify="end">
      <div>hello</div>
      <!--  -->
    </div>
    <div flex="1" overflow="auto" border="~ solid red">
      <router-view></router-view>
    </div>
  </div>
</template>

<style lang="less">
.k {
}
</style>

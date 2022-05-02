<script lang="ts" setup></script>

<template>
  <div>
    <div class="cards" grid="~" style="grid-template-columns: repeat(auto-fit, 200px)">
      <div class="add-card">
        <div>
          <i-ep:plus></i-ep:plus>
        </div>
        <div>添加配置</div>
        <!--  -->
      </div>
    </div>
  </div>
</template>

<style lang="less" scoped>
.add-card {
  border: 1px solid;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;

  @apply p-2;
  @apply border-gray-300 rounded;
  @apply hover:border-blue-500;
  //
}
</style>

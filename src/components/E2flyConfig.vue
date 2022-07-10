<script lang="ts" setup>
const data = reactive({
  type: 'import',
})
</script>

<template>
  <div class="e2fly-config">
    <k-row>
      <k-radio-group v-model="data.type">
        <k-radio value="import">导入</k-radio>
        <k-radio value="custom">自定义</k-radio>
      </k-radio-group>
    </k-row>
    <hr />
    <template v-if="data.type == 'import'">
      <k-row>
        <k-input class="w-full" placeholder="请输入链接"> </k-input>
      </k-row>
    </template>
    <template v-else>
      <k-row tag="label">
        <span>hello: </span>
        <k-select class="flex-1" block>
          <k-option value="1"> 12 </k-option>
        </k-select>
      </k-row>
    </template>
  </div>
</template>

<style lang="less"></style>

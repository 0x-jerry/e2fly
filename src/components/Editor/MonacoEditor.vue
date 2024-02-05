<script lang="ts" setup>
import { ref } from 'vue'
import { useMonaco } from './useMonaco'
import './userWorker'

const el = ref()

const props = defineProps<{
  modelValue: string
}>()

const emit = defineEmits(['update:modelValue'])

const ins = useMonaco(el, {
  language: 'json',
})

let _oldValue = ''

watchImmediate(
  () => props.modelValue,
  (v) => {
    if (_oldValue === v) {
      return
    }

    _oldValue = v

    ins.setValue(v)
  },
)

ins.onDidChangeModelContent(() => {
  const value = ins.getValue()

  if (value === _oldValue) {
    return
  }

  _oldValue = value

  emit('update:modelValue', value)
})
</script>

<template>
  <div class="editor-container" ref="el"></div>
</template>

<style lang="less" scoped>
.editor-container {
  width: 100%;
  height: 100%;
}
</style>

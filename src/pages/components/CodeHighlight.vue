<script lang="ts" setup>
import 'highlight.js/styles/github.css'
import hljs, { HighlightResult } from 'highlight.js'

const props = defineProps<{
  code: string
  lang: string
}>()

const html = ref<HighlightResult>()

watch(
  props,
  () => {
    html.value = hljs.highlight(props.code, { language: props.lang })
  },
  {
    deep: true,
    immediate: true,
  },
)
</script>

<template>
  <pre><code :class="`language-${props.lang}`" v-html="html?.value"></code></pre>
</template>

<style lang="scss"></style>

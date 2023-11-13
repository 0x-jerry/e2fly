import { editor } from 'monaco-editor'
import { Ref, onMounted, onUnmounted } from 'vue'

export function useMonaco(
  containerEl: Ref<HTMLElement>,
  opt: editor.IStandaloneEditorConstructionOptions,
) {
  const el = document.createElement('div')
  el.style.width = '100%'
  el.style.height = '100%'

  const ins = editor.create(el, {
    ...opt,
    automaticLayout: true,
  })

  onMounted(() => {
    containerEl.value?.append(el)
  })

  onUnmounted(() => {
    ins.dispose()
    el.remove()
  })

  return ins
}

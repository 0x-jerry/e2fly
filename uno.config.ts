import {
  defineConfig,
  presetAttributify,
  presetUno,
  transformerDirectives,
  transformerVariantGroup,
} from 'unocss'

// https://github.com/unocss/unocss
export default defineConfig({
  presets: [presetAttributify(), presetUno()],
  transformers: [transformerDirectives(), transformerVariantGroup()],
})

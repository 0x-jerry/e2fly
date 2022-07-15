import { defineConfig, presetAttributify, presetWind } from 'unocss'
import transformerDirective from '@unocss/transformer-directives'

// https://github.com/unocss/unocss
export default defineConfig({
  presets: [presetAttributify(), presetWind()],
  transformers: [transformerDirective()],
})

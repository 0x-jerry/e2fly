import { defineConfig, presetAttributify, presetWind, transformerDirectives } from 'unocss'

// https://github.com/unocss/unocss
export default defineConfig({
  presets: [presetAttributify(), presetWind()],
  transformers: [transformerDirectives()],
})

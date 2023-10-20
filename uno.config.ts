import {
  defineConfig,
  presetAttributify,
  presetWind,
  transformerDirectives,
  transformerVariantGroup,
} from 'unocss'

// https://github.com/unocss/unocss
export default defineConfig({
  presets: [presetAttributify(), presetWind()],
  transformers: [transformerDirectives(), transformerVariantGroup()],
})

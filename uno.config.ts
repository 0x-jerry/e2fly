import {
  defineConfig,
  presetAttributify,
  presetWind3,
  transformerDirectives,
  transformerVariantGroup,
} from 'unocss'

// https://github.com/unocss/unocss
export default defineConfig({
  presets: [presetWind3(), presetAttributify() ],
  transformers: [transformerDirectives(), transformerVariantGroup()],
})

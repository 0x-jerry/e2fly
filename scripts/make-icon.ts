import sharp from 'sharp'
import path from 'path'
import { fileURLToPath } from 'url'

const config = {
  resolutions: [1, 2, 3, 4, 5],
  baseSize: 16,
}

const dir = fileURLToPath(import.meta.url)
const r = (...args: string[]) => path.join(dir, '..', '..', ...args)

main()

async function main() {
  await resolve('assets/logo.png', 'src/assets')
  await resolve('assets/logoTemplate.png', 'src/assets')
}

async function resolve(input: string, output: string) {
  input = r(input)
  output = r(output)
  // ---
  const img = sharp(input)

  const name = path.parse(input).name
  const suffix = path.parse(input).ext

  for (const resolution of config.resolutions) {
    const size = resolution === 1 ? '' : `@${resolution}x`

    const outputPath = path.join(output, `${name}${size}${suffix}`)

    await img.resize(config.baseSize * resolution).toFile(outputPath)
    console.log('generate: ', outputPath)
  }
}
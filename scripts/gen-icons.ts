import { join } from 'path'
import sharp from 'sharp'
import { ensureDir, writeFile } from 'fs-extra'
import { createICNS, createICO } from 'png2icons'

interface ResizeOption {
  name: string
  size: string
}

type ResizeType = ResizeOption | string

interface GenIconPreset {
  output: {
    dir: string
  }
  resize: Array<ResizeType>
}

const set = [30, 44, 71, 89, 107, 142, 150, 284, 310]

const miniPreset: GenIconPreset = {
  output: {
    dir: join(__dirname, '../src-tauri/icons')
  },
  resize: [
    'icon.ico',
    'icon.icns',
    '32x32.png',
    '128x128.png',
    '128x128@2x.png',
    {
      name: 'icon.png',
      size: '512x512'
    },
    {
      name: 'StoreLogo.png',
      size: '50x50'
    },
    ...set.map((n) => ({
      name: `Square${n}x${n}Logo.png`,
      size: `${n}x${n}`
    }))
  ]
}

// ---------------
const logoFile = 'assets/logo.png'

resizeImage(logoFile, miniPreset)

async function resizeImage(file: string, preset: GenIconPreset) {
  const s = sharp(file)
  await ensureDir(preset.output.dir)

  const p = preset.resize.map(async (opt) => {
    const isStr = typeof opt === 'string'
    const size = parseSize(isStr ? opt : opt.size)

    const name = isStr ? opt : opt.name
    const output = join(preset.output.dir, name)

    if (name.endsWith('.ico')) {
      const buf = createICO(await s.png().toBuffer(), 0, 0, true)

      return writeFile(output, buf)
    }

    if (name.endsWith('.icns')) {
      const buf = createICNS(await s.toBuffer(), 5, 0)

      return writeFile(output, buf)
    }

    return s
      .resize(size.width, size.height, {
        fit: 'contain'
      })
      .png()
      .toFile(output)
  })

  await Promise.all(p)
}

function parseSize(str: string) {
  const sizeReg = /(\d+)x(\d+)(@\d+x)?/
  const [_, width, height, scaleStr = '@1x'] = str.match(sizeReg) || []

  const scale = parseInt(scaleStr.match(/\d+/)[0])

  const opt = {
    width: parseInt(width) * scale,
    height: parseInt(height) * scale
  }

  return opt
}

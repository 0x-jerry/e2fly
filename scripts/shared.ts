import fs from 'node:fs/promises'
import path from 'node:path'
import { exec } from '@0x-jerry/utils/node'

const platform = process.platform
export const isMacOS = platform === 'darwin'

export async function buildThenCopyTunHelper() {
  await exec('cargo build --package tun-helper --release')

  const extension = isMacOS ? '' : '.exe'

  const targetTriple = await getTargetTriple()
  const from = path.resolve(`target/release/tun-helper${extension}`)
  const to = path.resolve(`binaries/tun-helper-${targetTriple}${extension}`)
  await fs.copyFile(from, to)
}

async function getTargetTriple() {
  const targetTriple = await exec('rustc --print host-tuple', {
    collectOutput: true,
  })

  return targetTriple.trim()
}

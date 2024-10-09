import { defineConfig } from '@0x-jerry/x-release'
import fs from 'fs-extra'

export default defineConfig({
  publish: false,
  async beforeCommit(ctx) {
    const confPath = 'src-tauri/tauri.conf.json'
    const conf = await fs.readJson(confPath)

    conf.version = ctx.nextVersion.replace(/[a-z]+\./g, '')

    await fs.writeFile(confPath, JSON.stringify(conf, null, 2))
  },
})

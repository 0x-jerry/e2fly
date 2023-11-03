import { defineConfig } from '@0x-jerry/x-release'
import { readJson, writeFile } from 'fs-extra'

export default defineConfig({
  sequence: [
    //
    'pkg.update.version',
    async (ctx) => {
      const confPath = 'src-tauri/tauri.conf.json'
      const conf = await readJson(confPath)

      conf.package.version = ctx.nextVersion.replace(/[a-z]+\./g, '')

      await writeFile(confPath, JSON.stringify(conf, null, 2))
    },
    'git.commit',
    'git.tag',
    'git.push',
  ],
})

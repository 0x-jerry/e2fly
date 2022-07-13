import { defineConfig } from '@0x-jerry/x-release'
import { readFile, writeFile } from 'fs/promises'
import toml from '@iarna/toml'

export default defineConfig({
  sequence: [
    //
    'pkg.update.version',
    async (ctx) => {
      const tomlPath = 'src-tauri/Cargo.toml'
      const txt = await readFile(tomlPath, { encoding: 'utf-8' })
      const conf: any = toml.parse(txt)
      conf.package.version = ctx.nextVersion

      writeFile(tomlPath, toml.stringify(conf))
    },
    'git.commit',
    'git.tag',
    'git.push',
  ],
})

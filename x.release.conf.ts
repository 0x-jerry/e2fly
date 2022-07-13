import { defineConfig } from '@0x-jerry/x-release'
import { readFile, writeFile } from 'fs/promises'

export default defineConfig({
  sequence: [
    //
    'pkg.update.version',
    async (ctx) => {
      const tomlPath = 'src-tauri/Cargo.toml'
      const txt = await readFile(tomlPath, { encoding: 'utf-8' })

      const replacedTxt = txt.replace(/version = "[^"]+"/, `version = "${ctx.nextVersion}"`)

      writeFile(tomlPath, replacedTxt)
    },
    async (ctx) => await ctx.run(`cd src-tauri && cargo check`),
    'git.commit',
    'git.tag',
    'git.push',
  ],
})

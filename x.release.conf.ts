import { defineConfig } from '@0x-jerry/x-release'

export default defineConfig({
  sequence: [
    //
    'pkg.update.version',
    'git.commit',
    'git.tag',
    'git.push',
  ],
})

import type { Plugin, ResolvedConfig, ViteDevServer } from 'vite'
import { handleDev, handleBuild } from './handle'
import { resolveOptions } from './options'
import { ResolvedViteElectronBuilderOptions, ViteElectronBuilderOptions } from './types'

export function VitePluginElectronBuilder(userOptions: ViteElectronBuilderOptions): Plugin {
  let viteConfig: ResolvedConfig
  let options: ResolvedViteElectronBuilderOptions
  let currentMode: string

  return {
    name: 'vite-plugin-electron-builder',
    config(_, { mode }) {
      currentMode = mode
    },
    configResolved(config) {
      viteConfig = config
      options = resolveOptions(userOptions, viteConfig)
    },
    configureServer: ({ httpServer }: ViteDevServer) => {
      httpServer?.on('listening', () => {
        const address: any = httpServer.address()
        options.env.DEV_SERVER_URL = `http://${address.address}:${address.port}`

        handleDev(options)
      })
    },
    closeBundle: () => {
      if (currentMode == 'test') return
      handleBuild(options)
    },
  }
}

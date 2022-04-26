import { join } from 'path'
import { builtinModules } from 'module'
import { ResolvedConfig } from 'vite'
import { ResolvedViteElectronBuilderOptions, ViteElectronBuilderOptions } from './types'
import { buildConfig } from './build.config'

export function resolveOptions(options: ViteElectronBuilderOptions, viteConfig: ResolvedConfig) {
  const root = options.root || process.cwd()
  const external = Array.from(
    new Set([
      ...builtinModules.filter((x) => !/^_|^(internal|v8|node-inspect)\/|\//.test(x)),
      'electron',
      ...(Array.isArray(options.external) ? options.external : []),
    ])
  )

  const {
    outputFile = join(root, 'dist/main/index.js'),
    entryFile = join(root, 'src-main/index.ts'),
    tsconfig = join(root, 'tsconfig.json'),
    electronBuilderConfig = buildConfig,
    afterEsbuildBuild = async () => {},
  } = options

  const { env, command } = viteConfig

  const resolvedViteElectronBuilderOptions: ResolvedViteElectronBuilderOptions = {
    root,
    outputFile,
    entryFile,
    tsconfig,
    electronBuilderConfig,
    env,
    command,
    external,
    afterEsbuildBuild,
  }

  return resolvedViteElectronBuilderOptions
}

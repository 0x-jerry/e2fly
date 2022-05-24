import { Arrayable } from '@0x-jerry/utils'
import { Configuration as ElectronBuilderConfiguration } from 'electron-builder'

export interface ViteElectronBuilderOptions {
  root?: string
  entry: Arrayable<ElectronEntry>
  assetsDir?: Arrayable<AssetsEntry>
  tsconfig?: string
  external?: string[]
  electronBuilderConfig?: string | ElectronBuilderConfiguration
  afterEsbuildBuild?: () => Promise<void>
}

export interface AssetsEntry {
  input: string
  output: string
}

export interface ElectronEntry {
  input: string
  output: string
  electron?: boolean
}

export interface ResolvedViteElectronBuilderOptions extends Required<ViteElectronBuilderOptions> {
  env: {
    DEV_SERVER_URL: string
    [key: string]: any
  }
  command: 'build' | 'serve'
}

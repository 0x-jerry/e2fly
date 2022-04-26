import { Arrayable } from '@0x-jerry/utils'
import { Configuration as ElectronBuilderConfiguration } from 'electron-builder'

export interface ViteElectronBuilderOptions {
  root?: string
  entry: Arrayable<ElectronEntry>
  tsconfig?: string
  external?: string[]
  electronBuilderConfig?: string | ElectronBuilderConfiguration
  afterEsbuildBuild?: () => Promise<void>
}

export interface ElectronEntry {
  input: string
  output: string
  electron?: boolean
}
export interface ResolvedViteElectronBuilderOptions extends Required<ViteElectronBuilderOptions> {
  env: Record<string, any>
  command: 'build' | 'serve'
}

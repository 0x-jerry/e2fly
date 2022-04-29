import { platform } from 'os'
import { MacProxy } from './MacProxy'

export type SysProxyType = 'http' | 'socks'

export interface SysProxyConfig {
  ip: string
  port: number
}

export interface SysProxy {
  enable(type: SysProxyType, conf: SysProxyConfig): Promise<boolean> | boolean
  disable(type: SysProxyType): boolean | Promise<boolean>
}

function createProxyInstance(): SysProxy | null {
  if (platform() === 'darwin') {
    return new MacProxy()
  }

  return null
}

export const proxy = createProxyInstance()

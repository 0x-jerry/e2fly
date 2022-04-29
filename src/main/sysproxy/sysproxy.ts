import { platform } from 'os'
import { MacProxy } from './MacProxy'

export type SysProxyType = 'http' | 'socks'

export interface SysProxyConfig {
  ip: string
  port: number
}

export interface SysProxy {
  /**
   * Will throw error when enable failed.
   * @param type
   * @param conf
   * @throws {Error}
   */
  enable(type: SysProxyType, conf: SysProxyConfig): void | Promise<void>

  /**
   *
   * Will throw error when disable failed.
   * @param type
   * @throws {Error}
   */
  disable(type: SysProxyType): void | Promise<void>
}

function createProxyInstance(): SysProxy | null {
  if (platform() === 'darwin') {
    return new MacProxy()
  }

  return null
}

export const proxy = createProxyInstance()

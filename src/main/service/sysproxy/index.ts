import { isMac } from '@/main/utils'
import { MacProxy } from './MacProxy'
import { SysProxy, SysProxyOption } from './SysProx'

export { SysProxy } from './SysProx'

export type SysProxyService = SysProxy

export function createProxyInstance(opt: SysProxyOption): SysProxyService | null {
  if (isMac()) {
    return new MacProxy(opt.config)
  }

  return null
}

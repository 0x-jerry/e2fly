import { isMac } from '@/main/utils'
import { MacProxy } from './MacProxy'
import { SysProxy, SysProxyOption } from './sysproxy'

export { SysProxy } from './sysproxy'

export type SysProxyService = SysProxy

export function createProxyInstance(opt: SysProxyOption): SysProxyService | null {
  if (isMac()) {
    return new MacProxy(opt.config)
  }

  return null
}

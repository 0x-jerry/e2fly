// export * from './config'

import { ConfigService } from './config'
import { createProxyInstance, SysProxy } from './sysproxy'
import { V2flyService } from './v2fly'

interface Services {
  config: ConfigService
  v2fly: V2flyService
  sysProxy: SysProxy | null
}

export const services: Services = {} as Services

export async function initServices() {
  // config service
  const config = new ConfigService()
  await config.init()
  services.config = config

  // v2fly service
  const v2fly = new V2flyService(config)
  services.v2fly = v2fly

  // system proxy service
  services.sysProxy = createProxyInstance({ config })

  return services
}

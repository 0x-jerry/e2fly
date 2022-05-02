// export * from './config'

import { ConfigService } from './config'
import { V2flyService } from './v2fly'

interface Services {
  config: ConfigService
  v2fly: V2flyService
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

  return services
}

import { E2FlyConfig } from './types'

export * from './env'

export const config: E2FlyConfig = {
  proxy: {},
  v2fly: {
    inbound: [],
    outbound: {},
  },
}

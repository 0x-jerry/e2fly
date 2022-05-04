import { IV2RayOutbound } from '@0x-jerry/v2ray-schema'

export interface E2FlyConfig {
  proxy: ProxyConfig
  v2fly: V2FlyConfig
  outbound: {
    id: string
    label: string
    config: IV2RayOutbound
  }[]
}

export interface ProxyConfig {
  /**
   * Enable system proxy
   */
  system?: boolean
  /**
   * Enable PAC
   */
  pac?: boolean
}

export interface V2FlyConfig {
  bin: string
  http: V2flyConfigInbound
  socks: V2flyConfigInbound
}

export interface V2flyConfigInbound {
  address: string
  port: number
  auth?: {
    username: string
    password: string
  }
  sniffing?: {
    tls: boolean
    http: boolean
  }
}

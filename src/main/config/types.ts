import { IV2RayOutbound } from '@0x-jerry/v2ray-schema'

export interface E2FlyConfig {
  proxy: ProxyConfig
  v2fly: V2FlyConfig
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
  inbound: V2flyConfigInbound[]
  outbound: IV2RayOutbound
}

export interface V2flyConfigInbound {
  type: 'http' | 'socks'
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

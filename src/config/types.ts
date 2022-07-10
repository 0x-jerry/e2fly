// generate from https://transform.tools/json-to-typescript

export interface AppConfig {
  proxy: Proxy
  activeOutboundId: string
  v2fly: V2fly
  outbound: Outbound[]
}

export interface Proxy {
  system: boolean
  pac: boolean
}

export interface V2fly {
  bin: string
  http: Http
  socks: Socks
  stream: Stream
  routes: Routes
}

export interface Http {
  enabled: boolean
  address: string
  port: number
}

export interface Socks {
  enabled: boolean
  address: string
  port: number
}

export interface Stream {
  udp: boolean
  tcp: boolean
}

export interface Routes {
  bypassCN: boolean
  blockAds: boolean
}

export interface Outbound {
  id: string
  config: string
}

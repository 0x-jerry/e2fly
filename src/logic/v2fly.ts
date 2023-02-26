import type { V4, V4Config } from '@0x-jerry/v2ray-schema'
import { AppConfig } from '@/config'

interface V2rayBase64 {
  port: number
  tls: string
  /**
   * alert id
   */
  aid: number
  /**
   * version
   */
  v: string
  host: string
  type: string
  path: string
  /**
   * network
   */
  net: string
  /**
   * address
   */
  add: string
  /**
   * name
   */
  ps: string
  /**
   * uuid
   */
  id: string
}

export interface V2rayConfOption {
  b64: string
  mux: boolean
}

/**
 * Only support vmess protocol for now.
 * @returns
 */
export function getOutboundConfFromBase64(opt: V2rayConfOption): V4.outbounds.OutboundObject {
  const [_protocol, conf] = opt.b64.split('://')

  const config: V2rayBase64 = JSON.parse(window.atob(conf))

  const outbound: V4.outbounds.OutboundObject = {
    protocol: 'vmess',
    streamSettings: {
      wsSettings: {
        path: config.path,
        headers: {
          host: config.host,
        },
      },
      tlsSettings: {
        serverName: config.host,
        allowInsecure: false,
      },
      security: 'tls',
      network: 'ws',
    },
    settings: {
      _t: 'vmess',
      vnext: [
        {
          address: config.add,
          port: config.port,
          users: [
            {
              alterId: config.aid,
              id: config.id,
              security: 'auto',
              level: 0,
            },
          ],
        },
      ],
    },
  }

  if (opt.mux) {
    outbound.mux = {
      enabled: true,
      concurrency: 8,
    }
  }

  return outbound
}

//  ----------

function getLogConf(): V4.overview.LogObject {
  return {
    loglevel: 'warning',
  }
}

enum OutboundTag {
  DIRECT = 'direct',
  PROXY = 'proxy',
  BLOCK = 'block',
}

function getOutboundDirectConf(): V4.outbounds.OutboundObject {
  return {
    tag: OutboundTag.DIRECT,
    protocol: 'freedom',
    settings: {},
  }
}

function getOutboundBlockConf(): V4.outbounds.OutboundObject {
  return {
    tag: OutboundTag.BLOCK,
    protocol: 'blackhole',
    settings: {},
  }
}

function getHttpInbound(host: string, port: number): V4.inbounds.InboundObject {
  return {
    protocol: 'http',
    listen: host,
    port: port,
    sniffing: {
      enabled: true,
      destOverride: ['tls', 'http'],
    },
  }
}

function getSocksInbound(host: string, port: number): V4.inbounds.InboundObject {
  return {
    protocol: 'socks',
    listen: host,
    port: port,
    settings: {
      udp: true,
      auth: 'noauth',
    },
    sniffing: {
      enabled: true,
      destOverride: ['tls', 'http'],
    },
  }
}

function getRoutingConf(rules?: V4.routing.RuleObject[]): V4.routing.RoutingObject {
  const extraRules = rules || []

  // https://www.v2fly.org/config/routing.html#%E9%A2%84%E5%AE%9A%E4%B9%89%E5%9F%9F%E5%90%8D%E5%88%97%E8%A1%A8
  const proxyRules = [
    //
    'category-dev',
    'google',
    'apple',
    'twitter',
    'telegram',
  ]

  return {
    domainStrategy: 'IPIfNonMatch',
    domainMatcher: 'mph',
    rules: [
      {
        type: 'field',
        domain: proxyRules.map((n) => 'geosite:' + n),
        outboundTag: OutboundTag.PROXY,
      },
      {
        type: 'field',
        outboundTag: OutboundTag.DIRECT,
        ip: [
          'geoip:private', // 私有地址 IP，如路由器等
        ],
      },
      ...extraRules,
    ],
  }
}

export function getV2rayConfig(opt: AppConfig, outbound: V4.outbounds.OutboundObject): V4Config {
  const { v2fly } = opt

  const { routes } = v2fly

  const inbounds: V4.inbounds.InboundObject[] = []

  if (v2fly.http.enabled) {
    inbounds.push(getHttpInbound(v2fly.http.address, v2fly.http.port))
  }

  if (v2fly.socks.enabled) {
    inbounds.push(getSocksInbound(v2fly.socks.address, v2fly.socks.port))
  }

  const extraRules: V4.routing.RuleObject[] = []

  // enable bypass CN mainland
  if (routes.bypassCN) {
    extraRules.push(
      {
        type: 'field',
        outboundTag: OutboundTag.DIRECT,
        ip: [
          'geoip:cn', // 中国大陆的 IP
        ],
      },
      {
        type: 'field',
        outboundTag: OutboundTag.DIRECT,
        domain: ['geosite:cn'],
      },
    )
  }

  // block ads
  if (routes.blockAds) {
    extraRules.push({
      type: 'field',
      outboundTag: OutboundTag.BLOCK,
      domain: ['geosite:category-ads-all'],
    })
  }

  return {
    log: getLogConf(),
    inbounds,
    outbounds: [
      {
        ...outbound,
        tag: OutboundTag.PROXY,
      },
      getOutboundDirectConf(),
      getOutboundBlockConf(),
    ],
    routing: getRoutingConf(extraRules),
    dns: {
      servers: [
        //
        'quic+local://dns.adguard.com',
      ],
    },
  }
}

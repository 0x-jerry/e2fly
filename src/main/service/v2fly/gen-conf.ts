import {
  IStrategy,
  IV2Ray,
  LogLevel,
  V2RayProtocol,
  IV2RayOutbound,
  IV2rayLog,
  IV2RayInbound,
  IV2rayRouting,
} from '@0x-jerry/v2ray-schema'

function getLogConf(): IV2rayLog {
  return {
    loglevel: LogLevel.warning,
  }
}

enum OutboundTag {
  DIRECT = 'direct',
  PROXY = 'proxy',
  BLOCK = 'block',
}

function getOutboundDirectConf(): IV2RayOutbound {
  return {
    tag: OutboundTag.DIRECT,
    protocol: V2RayProtocol.FREEDOM,
    settings: {},
  }
}

function getOutboundBlockConf(): IV2RayOutbound {
  return {
    tag: OutboundTag.BLOCK,
    protocol: V2RayProtocol.BLACKHOLE,
    settings: {},
  }
}

function getHttpInbound(host: string, port: number): IV2RayInbound {
  return {
    protocol: V2RayProtocol.HTTP,
    listen: host,
    port: port,
    sniffing: {
      enabled: true,
      destOverride: ['tls', 'http'],
    },
  }
}

function getSocksInbound(host: string, port: number): IV2RayInbound {
  return {
    protocol: V2RayProtocol.SOCKS,
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

export interface V2rayConfigOption {
  proxy: {
    http: {
      host: string
      port: number
    }
    socks: {
      host: string
      port: number
    }
  }
  rules?: IV2rayRouting['rules']
}

function getRoutingConf(rules: IV2rayRouting['rules']): IV2rayRouting {
  const extraRules = rules || []

  return {
    domainStrategy: IStrategy.IPIfNonMatch,
    domainMatcher: 'mph',
    rules: [
      {
        type: 'field',
        domain: ['geosite:google'],
        outboundTag: OutboundTag.PROXY,
      },
      {
        type: 'field',
        outboundTag: OutboundTag.DIRECT,
        ip: [
          'geoip:cn', // 中国大陆的 IP
          'geoip:private', // 私有地址 IP，如路由器等
        ],
      },
      {
        type: 'field',
        outboundTag: OutboundTag.DIRECT,
        domain: ['geosite:cn'],
      },
      {
        type: 'field',
        outboundTag: OutboundTag.BLOCK,
        domain: ['geosite:category-ads-all'],
      },
      ...extraRules,
    ],
  }
}

export function getV2rayConfig(opt: V2rayConfigOption, ...outbounds: IV2RayOutbound[]): IV2Ray {
  return {
    log: getLogConf(),
    inbounds: [
      getHttpInbound(opt.proxy.http.host, opt.proxy.http.port),
      getSocksInbound(opt.proxy.socks.host, opt.proxy.socks.port),
    ],
    outbounds: [
      ...outbounds.map((outbound) => {
        return {
          ...outbound,
          tag: OutboundTag.PROXY,
        }
      }),
      getOutboundDirectConf(),
      getOutboundBlockConf(),
    ],
    routing: getRoutingConf(opt.rules),
  }
}

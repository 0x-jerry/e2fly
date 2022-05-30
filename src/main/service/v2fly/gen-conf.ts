import { E2FlyConfig } from '@/main/config'
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

function getRoutingConf(rules?: IV2rayRouting['rules']): IV2rayRouting {
  const extraRules = rules || []

  return {
    domainStrategy: IStrategy.IPIfNonMatch,
    domainMatcher: 'mph',
    rules: [
      {
        type: 'field',
        domain: ['geosite:github'],
        outboundTag: OutboundTag.PROXY,
      },
      {
        type: 'field',
        outboundTag: OutboundTag.DIRECT,
        ip: [
          'geoip:private', // 私有地址 IP，如路由器等
        ],
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

export function getV2rayConfig(opt: E2FlyConfig, outbound: IV2RayOutbound): IV2Ray {
  const { v2fly } = opt

  const inbounds: IV2RayInbound[] = []

  if (v2fly.http.enabled) {
    inbounds.push(getHttpInbound(v2fly.http.address, v2fly.http.port))
  }

  if (v2fly.socks.enabled) {
    inbounds.push(getSocksInbound(v2fly.socks.address, v2fly.socks.port))
  }

  const extraRules: IV2rayRouting['rules'] = []

  // enable bypass CN mainland
  if (true) {
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
      }
    )
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
  }
}

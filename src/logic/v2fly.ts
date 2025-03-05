import type { V2FlyConfig } from '@0x-jerry/v2ray-schema'
import { AppConfig } from '@/config'
import { OutboundObject } from '@0x-jerry/v2ray-schema/types/outbound'
import { LogObject } from '@0x-jerry/v2ray-schema/types/log'
import { InboundObject } from '@0x-jerry/v2ray-schema/types/inbound'
import { RoutingObject, RuleObject } from '@0x-jerry/v2ray-schema/types/routing'
import * as x2sp from '@0x-jerry/x2sp'

export interface V2rayConfOption {
  sharingString: string
  mux: boolean
}

/**
 * Only support vmess protocol for now.
 * @returns
 */
export function getOutboundConfFromBase64(opt: V2rayConfOption): OutboundObject {
  const config = x2sp.decode(opt.sharingString)

  const outbound: OutboundObject = {
    protocol: config.protocol,
    streamSettings: {
      wsSettings: {
        path: config.transport?.path,
      },
      tlsSettings: {
        serverName: config.host,
      },
      security: config.transport?.security,
      network: config.transport?.type,
    },
  }

  if (config.protocol === 'vmess') {
    outbound.settings = {
      _t: 'vmess',
      vnext: [
        {
          address: config.host,
          port: config.port,
          users: [
            {
              id: config.uuid,
              security: 'auto',
              level: 0,
            },
          ],
        },
      ],
    }
  }

  if (config.protocol === 'vless') {
    outbound.settings = {
      _t: 'vless',
      vnext: [
        {
          address: config.host,
          port: config.port,
          users: [
            {
              id: config.uuid,
              encryption: 'none',
            },
          ],
        },
      ],
    }
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

export function getLogConf(): LogObject {
  return {
    loglevel: 'debug',
  }
}

enum OutboundTag {
  DIRECT = 'direct',
  PROXY = 'proxy',
  BLOCK = 'block',
  DNS = 'dns-output',
}

function getOutboundDirectConf(): OutboundObject {
  return {
    tag: OutboundTag.DIRECT,
    protocol: 'freedom',
    settings: {},
  }
}

function getOutboundBlockConf(): OutboundObject {
  return {
    tag: OutboundTag.BLOCK,
    protocol: 'blackhole',
    settings: {},
  }
}

function getHttpInbound(port: number, allowLAN?: boolean): InboundObject {
  return {
    protocol: 'http',
    listen: allowLAN ? '0.0.0.0' : '127.0.0.1',
    port: port,
    sniffing: {
      enabled: true,
      destOverride: ['fakedns+others', 'tls', 'http', 'quic'],
    },
  }
}

function getSocksInbound(port: number, allowLAN?: boolean): InboundObject {
  return {
    protocol: 'socks',
    listen: allowLAN ? '0.0.0.0' : '127.0.0.1',
    port: port,
    settings: {
      udp: true,
      auth: 'noauth',
    },
    sniffing: {
      enabled: true,
      destOverride: ['fakedns+others', 'tls', 'http', 'quic'],
    },
  }
}

function getRoutingConf(rules?: RuleObject[]): RoutingObject {
  const extraRules = rules || []

  // https://www.v2fly.org/config/routing.html#%E9%A2%84%E5%AE%9A%E4%B9%89%E5%9F%9F%E5%90%8D%E5%88%97%E8%A1%A8
  const proxyRules = [
    //
    'category-dev',
    'google',
    'apple',
    'twitter',
    'telegram',
    'bing',
  ]

  return {
    domainStrategy: 'IPIfNonMatch',
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

export async function getV2rayConfig(
  opt: AppConfig,
  outbound: OutboundObject,
): Promise<V2FlyConfig> {
  const { v2fly, proxy } = opt

  const { routes } = v2fly

  const inbounds: InboundObject[] = []

  if (v2fly.http.enabled) {
    inbounds.push(getHttpInbound(v2fly.http.port, proxy.lan))
  }

  if (v2fly.socks.enabled) {
    inbounds.push(getSocksInbound(v2fly.socks.port, proxy.lan))
  }

  const extraRules: RuleObject[] = []

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
        {
          address: 'fakedns',
          domains: ['geosite:geolocation-!cn'],
          expectIPs: ['geoip:!cn'],
        },
        {
          address: '223.5.5.5',
          domains: ['geosite:cn'],
          expectIPs: ['geoip:cn'],
        },
        {
          address: 'https://1.1.1.1/dns-query',
          domains: ['geosite:geolocation-!cn'],
          expectIPs: ['geoip:!cn'],
        },
        '223.5.5.5',
        '1.1.1.1',
        'localhost',
      ],
    },
  }
}

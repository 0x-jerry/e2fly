import { IV2RayOutbound, V2RayProtocol, IVmessSecurity } from '@0x-jerry/v2ray-schema'

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
export function getOutboundConfFromBase64(opt: V2rayConfOption): IV2RayOutbound {
  const [_protocol, conf] = opt.b64.split('://')

  const config: V2rayBase64 = JSON.parse(window.atob(conf))

  const outbound: IV2RayOutbound = {
    protocol: V2RayProtocol.VMESS,
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
      vnext: [
        {
          address: config.add,
          port: config.port,
          users: [
            {
              alterId: config.aid,
              id: config.id,
              security: IVmessSecurity.AUTO,
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

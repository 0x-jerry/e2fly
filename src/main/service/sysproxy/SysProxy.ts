import { ConfigService } from '../config'

export type SysProxyType = 'http' | 'socks'

export interface SysProxyConfig {
  addr: string
  port: number
}

export interface SysProxyOption {
  config: ConfigService
}

export abstract class SysProxy {
  /**
   * Will throw error when enable failed.
   * @param type
   * @param conf
   * @private
   * @throws {Error}
   */
  abstract _enable(type: SysProxyType, conf: SysProxyConfig): void | Promise<void>

  /**
   *
   * Will throw error when disable failed.
   * @param type
   * @private
   * @throws {Error}
   */
  abstract _disable(type: SysProxyType): void | Promise<void>

  constructor(public conf: ConfigService) {}

  async start() {
    const conf = this.conf.config

    if (!conf.proxy.system) {
      await this.stop()
      return
    }

    // ----

    const { http, socks } = conf.v2fly

    if (http.enabled) {
      await this._enable('http', {
        addr: http.address,
        port: http.port,
      })
    } else {
      await this._disable('http')
    }

    if (socks.enabled) {
      await this._enable('socks', {
        addr: socks.address,
        port: socks.port,
      })
    } else {
      await this._disable('socks')
    }
  }

  async stop() {
    await this._disable('http')
    await this._disable('socks')
  }
}

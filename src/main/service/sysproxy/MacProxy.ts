import { execSync } from 'child_process'
import { SysProxy, SysProxyType, SysProxyConfig } from './SysProx'

type MacNetworkType = 'Ethernet' | 'Thunderbolt Ethernet' | 'Wi-Fi'

export class MacProxy extends SysProxy {
  networkTypes: MacNetworkType[] = ['Ethernet', 'Thunderbolt Ethernet', 'Wi-Fi']

  getAvailableNetworkTypes() {
    return this.networkTypes.filter((networkType) => {
      try {
        execSync('networksetup -getwebproxy ' + networkType)
        return true
      } catch (error) {
        return false
      }
    })
  }

  _enable(type: SysProxyType, conf: SysProxyConfig) {
    const availableNetworkTypes = this.getAvailableNetworkTypes()

    for (const netType of availableNetworkTypes) {
      if (type === 'http') {
        this.enableHttpProxy(conf, netType)
      } else if (type === 'socks') {
        this.enableSocksProxy(conf, netType)
      }
    }
  }

  _disable(type: SysProxyType) {
    const availableNetworkTypes = this.getAvailableNetworkTypes()
    for (const netType of availableNetworkTypes) {
      if (type === 'http') {
        this.disableHttpProxy(netType)
      } else if (type === 'socks') {
        this.disableSocksProxy(netType)
      }
    }
  }

  enableHttpProxy(conf: SysProxyConfig, networkType: MacNetworkType) {
    // set http proxy
    const http = `networksetup -setwebproxy ${networkType} ${conf.addr} ${conf.port}`
    execSync(http)

    // set https proxy
    const https = `networksetup -setsecurewebproxy ${networkType} ${conf.addr} ${conf.port}`
    execSync(https)
  }

  disableHttpProxy(networkType: MacNetworkType) {
    // set http proxy
    const http = `networksetup -setwebproxystate ${networkType} off`
    execSync(http)

    // set https proxy
    const https = `networksetup -setsecurewebproxystate ${networkType} off`
    execSync(https)
  }

  enableSocksProxy(conf: SysProxyConfig, networkType: MacNetworkType) {
    // set socks proxy
    const http = `networksetup -setsocksfirewallproxy ${networkType} ${conf.addr} ${conf.port}`
    execSync(http)
  }

  disableSocksProxy(networkType: MacNetworkType) {
    // set socks proxy
    const http = `networksetup -setsocksfirewallproxystate ${networkType} off`
    execSync(http)
  }
}

import { execSync } from 'child_process'
import { SysProxy, SysProxyType, SysProxyConfig } from './sysproxy'

type MacNetworkType = 'Ethernet' | 'Thunderbolt Ethernet' | 'Wi-Fi'

export class MacProxy implements SysProxy {
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

  enable(type: SysProxyType, conf: SysProxyConfig) {
    const availableNetworkTypes = this.getAvailableNetworkTypes()
    if (type === 'http') {
      for (const netType of availableNetworkTypes) {
        this.enableHttpProxy(conf, netType)
      }
    }
  }

  disable(type: SysProxyType) {
    const availableNetworkTypes = this.getAvailableNetworkTypes()
    if (type === 'http') {
      for (const netType of availableNetworkTypes) {
        this.disableHttpProxy(netType)
      }
    }
  }

  enableHttpProxy(conf: SysProxyConfig, networkType: MacNetworkType) {
    // set http proxy
    const http = `networksetup -setwebproxy ${networkType} ${conf.ip} ${conf.port} && networksetup -setproxybypassdomains ${networkType} 127.0.0.1 localhost`
    execSync(http)

    // set https proxy
    const https = `networksetup -setsecurewebproxy ${networkType} ${conf.ip} ${conf.port} && networksetup -setproxybypassdomains ${networkType} 127.0.0.1 localhost`
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
}

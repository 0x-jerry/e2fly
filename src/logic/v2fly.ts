// ... existing imports and code ...

// Add support for TUN config generation (scaffold, not implemented)
export function getTunInbound(port: number, allowLAN?: boolean): InboundObject {
  // TODO: Implement actual TUN inbound config as per v2ray/v2fly spec
  return {
    protocol: 'tun',
    listen: allowLAN ? '0.0.0.0' : '127.0.0.1',
    port: port,
    settings: {},
    sniffing: {
      enabled: true,
      destOverride: ['fakedns+others', 'tls', 'http', 'quic'],
    },
  }
}
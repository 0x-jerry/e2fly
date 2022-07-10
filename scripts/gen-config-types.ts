import fetch from 'node-fetch'
import { writeFile } from 'fs/promises'

const config = JSON.stringify({
  proxy: { system: false, pac: false },
  activeOutboundId: '8outt12ADTezpjmRpOtQ-',
  v2fly: {
    bin: 'v2ray',
    http: { enabled: true, address: '127.0.0.1', port: 6667 },
    socks: { enabled: true, address: '127.0.0.1', port: 6666 },
    stream: { udp: true, tcp: true },
    routes: { bypassCN: true, blockAds: true }
  },
  outbound: [
    {
      id: '8outt12ADTezpjmRpOtQ-',
      label: 'default',
      config: ''
    }
  ]
})

generate(config, 'typescript', 'src/config/config.types.ts')
generate(config, 'rust', 'src-tauri/src/config/model.rs')

async function generate(json: string, lang: string, dist: string) {
  const url = new URL('https://0x-jerry-dd-api.deno.dev/transform/json')

  url.searchParams.set('json', json)
  url.searchParams.set('lang', lang)
  url.searchParams.set('name', 'AppConfig')

  const res = await fetch(url.toString())

  const txt = await res.text()

  await writeFile(dist, txt)
}

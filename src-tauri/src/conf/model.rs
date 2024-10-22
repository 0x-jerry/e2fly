#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    #[serde(rename = "app")]
    pub app: App,

    #[serde(rename = "proxy")]
    pub proxy: Proxy,

    #[serde(rename = "active")]
    pub active: Active,

    #[serde(rename = "v2fly")]
    pub v2_fly: V2Fly,

    #[serde(rename = "outbound")]
    pub outbound: Vec<Outbound>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Active {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "outboundId")]
    pub outbound_id: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct App {
    #[serde(rename = "autoStartup")]
    pub auto_startup: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Outbound {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "config")]
    pub config: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Proxy {
    #[serde(rename = "system")]
    pub system: bool,

    #[serde(rename = "lan")]
    pub lan: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V2Fly {
    #[serde(rename = "bin")]
    pub bin: String,

    #[serde(rename = "http")]
    pub http: Listener,

    #[serde(rename = "socks")]
    pub socks: Listener,

    #[serde(rename = "stream")]
    pub stream: Stream,

    #[serde(rename = "routes")]
    pub routes: Routes,
}

impl Default for V2Fly {
    fn default() -> Self {
        let mut socks = Listener::default();

        socks.port += 1;

        Self {
            bin: "v2ray".to_string(),
            http: Listener::default(),
            socks,
            stream: Stream::default(),
            routes: Routes::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Listener {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "port")]
    pub port: i64,
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            enabled: true,
            address: "127.0.0.1".to_string(),
            port: 6666,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Routes {
    #[serde(rename = "bypassCN")]
    pub bypass_cn: bool,

    #[serde(rename = "blockAds")]
    pub block_ads: bool,
}

impl Default for Routes {
    fn default() -> Self {
        Self {
            bypass_cn: true,
            block_ads: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stream {
    #[serde(rename = "udp")]
    pub udp: bool,

    #[serde(rename = "tcp")]
    pub tcp: bool,
}

impl Default for Stream {
    fn default() -> Self {
        Self {
            udp: true,
            tcp: true,
        }
    }
}

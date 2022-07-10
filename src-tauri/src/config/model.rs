use serde::{Deserialize, Serialize};

// generate from: https://transform.tools/json-to-rust-serde

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub proxy: Proxy,
    pub active_outbound_id: String,
    pub v2fly: V2fly,
    pub outbound: Vec<Outbound>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proxy {
    pub system: bool,
    pub pac: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2fly {
    pub bin: String,
    pub http: Http,
    pub socks: Socks,
    pub stream: Stream,
    pub routes: Routes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub enabled: bool,
    pub address: String,
    pub port: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Socks {
    pub enabled: bool,
    pub address: String,
    pub port: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub udp: bool,
    pub tcp: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Routes {
    #[serde(rename = "bypassCN")]
    pub bypass_cn: bool,
    pub block_ads: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbound {
    pub id: String,
    pub config: String,
}

// ------------

impl AppConfig {
    pub fn new() -> Self {
        Self {
            proxy: Proxy::new(),
            active_outbound_id: String::from(""),
            v2fly: V2fly::new(),
            outbound: vec![],
        }
    }
}

impl Proxy {
    pub fn new() -> Self {
        Self {
            system: false,
            pac: false,
        }
    }
}

impl V2fly {
    pub fn new() -> Self {
        Self {
            bin: String::from("v2ray"),
            http: Http::new(),
            socks: Socks::new(),
            stream: Stream::new(),
            routes: Routes::new(),
        }
    }
}

impl Http {
    pub fn new() -> Self {
        Self {
            enabled: true,
            address: String::from("127.0.0.1"),
            port: 6667,
        }
    }
}

impl Socks {
    pub fn new() -> Self {
        Self {
            enabled: true,
            address: String::from("127.0.0.1"),
            port: 6666,
        }
    }
}

impl Stream {
    pub fn new() -> Self {
        Self {
            udp: true,
            tcp: true,
        }
    }
}

impl Routes {
    pub fn new() -> Self {
        Self {
            bypass_cn: true,
            block_ads: true,
        }
    }
}

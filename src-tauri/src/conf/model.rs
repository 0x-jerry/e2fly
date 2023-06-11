// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "app", default)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Active {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "outboundId")]
    pub outbound_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    #[serde(rename = "autoHideWhenBlur")]
    pub auto_hide_when_blur: bool,

    #[serde(rename = "autoStartup")]
    pub auto_startup: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outbound {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "config")]
    pub config: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    #[serde(rename = "system")]
    pub system: bool,

    #[serde(rename = "lan")]
    pub lan: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct V2Fly {
    #[serde(rename = "bin")]
    pub bin: String,

    #[serde(rename = "http")]
    pub http: Http,

    #[serde(rename = "socks")]
    pub socks: Http,

    #[serde(rename = "stream")]
    pub stream: Stream,

    #[serde(rename = "routes")]
    pub routes: Routes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Http {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "port")]
    pub port: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Routes {
    #[serde(rename = "bypassCN")]
    pub bypass_cn: bool,

    #[serde(rename = "blockAds")]
    pub block_ads: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    #[serde(rename = "udp")]
    pub udp: bool,

    #[serde(rename = "tcp")]
    pub tcp: bool,
}

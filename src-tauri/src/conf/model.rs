// ... previous code ...

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Proxy {
    #[serde(rename = "system")]
    pub system: bool,

    #[serde(rename = "lan")]
    pub lan: bool,

    // New: TUN mode support
    #[serde(rename = "tun")]
    pub tun: bool,
}
// ... rest of file ...
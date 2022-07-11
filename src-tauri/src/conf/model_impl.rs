use super::model::*;

impl AppConfig {
    pub fn new() -> Self {
        Self {
            proxy: Proxy::new(),
            active_outbound_id: String::from(""),
            v2_fly: V2Fly::new(),
            outbound: vec![],
        }
    }
}

impl Outbound {
    pub fn new() -> Self {
        Self {
            id: String::from(""),
            config: String::from(""),
            label: String::from(""),
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

impl V2Fly {
    pub fn new() -> Self {
        Self {
            bin: String::from("v2ray"),
            http: Http::new(),
            socks: Http::with_socks(),
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

    pub fn with_socks() -> Self {
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

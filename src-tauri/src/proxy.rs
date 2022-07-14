use std::io;

use crate::conf::model::AppConfig;

mod mac;

#[derive(Debug, Copy, Clone)]
pub struct ProxyConf<'a> {
    pub addr: &'a str,
    pub port: &'a str,
}

#[derive(Debug, Copy, Clone)]
pub enum SysProxyType {
    Http,
    Socks,
}

pub fn enable_proxy(proxy_type: SysProxyType, conf: ProxyConf) -> io::Result<()> {
    if cfg!(unix) {
        mac::enable_proxy(proxy_type, conf)?
    }

    Ok(())
}

pub fn disable_proxy(proxy_type: SysProxyType) -> io::Result<()> {
    if cfg!(unix) {
        mac::disable_proxy(proxy_type)?
    }

    Ok(())
}

pub fn set_proxy(conf: &AppConfig) {
    if !conf.proxy.system {
        disable_proxy(SysProxyType::Http).expect("disable http proxy failed");
        disable_proxy(SysProxyType::Socks).expect("disable socks proxy failed");
        return;
    }

    let http = &conf.v2_fly.http;

    if http.enabled {
        enable_proxy(
            SysProxyType::Http,
            ProxyConf {
                addr: http.address.as_str(),
                port: http.port.to_string().as_str(),
            },
        )
        .expect("set http proxy failed");
    }

    let socks = &conf.v2_fly.socks;

    if socks.enabled {
        enable_proxy(
            SysProxyType::Socks,
            ProxyConf {
                addr: socks.address.as_str(),
                port: socks.port.to_string().as_str(),
            },
        )
        .expect("set http proxy failed");
    }
}

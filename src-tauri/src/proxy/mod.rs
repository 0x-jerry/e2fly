use crate::conf::model::AppConfig;
use std::io;

#[cfg(unix)]
mod mac;
#[cfg(unix)]
use mac::ProxyImpl;

#[cfg(windows)]
mod win;
#[cfg(windows)]
use win::ProxyImpl;

#[derive(Debug, Clone)]
pub struct ProxyConf {
    pub addr: String,
    pub port: String,
}

#[derive(Debug, Clone)]
pub enum SysProxyType {
    Http(Option<ProxyConf>),
    #[allow(dead_code)]
    Socks(Option<ProxyConf>),
}

pub trait ProxyAction {
    fn enable(conf: SysProxyType) -> io::Result<()>;
    fn disable(conf: SysProxyType) -> io::Result<()>;
}

pub fn set_proxy(conf: &AppConfig) -> io::Result<()> {
    if !conf.proxy.system {
        ProxyImpl::disable(SysProxyType::Http(None))?;
        ProxyImpl::disable(SysProxyType::Socks(None))?;

        return Ok(());
    }

    let http = &conf.v2_fly.http;

    if http.enabled {
        ProxyImpl::enable(SysProxyType::Http(Some(ProxyConf {
            addr: http.address.clone(),
            port: http.port.to_string(),
        })))?;
    }

    let socks = &conf.v2_fly.socks;

    if socks.enabled {
        ProxyImpl::enable(SysProxyType::Socks(Some(ProxyConf {
            addr: socks.address.clone(),
            port: socks.port.to_string(),
        })))?;
    }

    Ok(())
}

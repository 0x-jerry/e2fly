use std::io;

use crate::utils::run_command;

use super::{ProxyAction, ProxyConf, SysProxyType};

pub struct ProxyImpl;

impl ProxyAction for ProxyImpl {
    fn enable(conf: SysProxyType) -> io::Result<()> {
        enable_proxy(conf)
    }

    fn disable(conf: SysProxyType) -> io::Result<()> {
        disable_proxy(conf)
    }
}

const PROXY_CMD: &str = "networksetup";

const NETWORK_TYPES: [&str; 3] = ["Ethernet", "Thunderbolt Ethernet", "Wi-Fi"];

fn get_available_network_types() -> Vec<&'static str> {
    let types: Vec<&str> = NETWORK_TYPES
        .iter()
        .filter(|x| run_command(PROXY_CMD, &["-getwebproxy", x]).is_ok())
        .copied()
        .collect();

    types
}

fn enable_http_proxy(net_type: &str, conf: &ProxyConf) -> io::Result<()> {
    // set http proxy
    run_command(
        PROXY_CMD,
        &[
            "-setwebproxy",
            net_type,
            conf.addr.as_str(),
            conf.port.as_str(),
        ],
    )?;

    // set https proxy
    run_command(
        PROXY_CMD,
        &[
            "-setsecurewebproxy",
            net_type,
            conf.addr.as_str(),
            conf.port.as_str(),
        ],
    )?;

    Ok(())
}

fn enable_socks_proxy(net_type: &str, conf: &ProxyConf) -> io::Result<()> {
    // set socks proxy
    run_command(
        PROXY_CMD,
        &[
            "-setsocksfirewallproxy",
            net_type,
            conf.addr.as_str(),
            conf.port.as_str(),
        ],
    )?;

    Ok(())
}

fn enable_proxy(proxy_type: SysProxyType) -> io::Result<()> {
    let types = get_available_network_types();

    for net_type in types {
        match proxy_type {
            SysProxyType::Http(Some(ref conf)) => {
                enable_http_proxy(net_type, conf)?;
            }
            SysProxyType::Socks(Some(ref conf)) => {
                enable_socks_proxy(net_type, conf)?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn disable_proxy(proxy_type: SysProxyType) -> io::Result<()> {
    let types = get_available_network_types();

    for net_type in types {
        match proxy_type {
            SysProxyType::Http(_) => {
                disable_http_proxy(net_type)?;
            }
            SysProxyType::Socks(_) => {
                disable_socks_proxy(net_type)?;
            }
        }
    }

    Ok(())
}

fn disable_http_proxy(net_type: &str) -> io::Result<()> {
    // set http proxy
    run_command(PROXY_CMD, &["-setwebproxystate", net_type, "off"])?;

    // set https proxy
    run_command(PROXY_CMD, &["-setsecurewebproxystate", net_type, "off"])?;

    Ok(())
}

fn disable_socks_proxy(net_type: &str) -> io::Result<()> {
    // set socks proxy
    run_command(PROXY_CMD, &["-setsocksfirewallproxystate", net_type, "off"])?;

    Ok(())
}

use std::io;

use crate::utils::run_command;

use super::{ProxyConf, SysProxyType};

const PROXY_CMD: &str = "networksetup";

const NETWORK_TYPES: [&str; 3] = ["Ethernet", "Thunderbolt Ethernet", "Wi-Fi"];

fn get_available_network_types() -> Vec<&'static str> {
    let types: Vec<&str> = NETWORK_TYPES
        .iter()
        .filter(|x| run_command(PROXY_CMD, &["-getwebproxy", x]).is_ok())
        .map(|x| *x)
        .collect();

    types
}

fn enable_http_proxy(net_type: &str, conf: ProxyConf) -> io::Result<()> {
    // set http proxy
    run_command(PROXY_CMD, &["-setwebproxy", net_type, conf.addr, conf.port])?;

    // set https proxy
    run_command(
        PROXY_CMD,
        &["-setsecurewebproxy", net_type, conf.addr, conf.port],
    )?;

    Ok(())
}

fn enable_socks_proxy(net_type: &str, conf: ProxyConf) -> io::Result<()> {
    // set socks proxy
    run_command(
        PROXY_CMD,
        &["-setsocksfirewallproxy", net_type, conf.addr, conf.port],
    )?;

    Ok(())
}

pub fn enable_proxy(proxy_type: SysProxyType, conf: ProxyConf) -> io::Result<()> {
    let types = get_available_network_types();

    for net_type in types {
        match proxy_type {
            SysProxyType::Http => {
                enable_http_proxy(net_type, conf)?;
            }
            SysProxyType::Socks => {
                enable_socks_proxy(net_type, conf)?;
            }
        }
    }

    Ok(())
}

pub fn disable_proxy(proxy_type: SysProxyType) -> io::Result<()> {
    let types = get_available_network_types();

    for net_type in types {
        match proxy_type {
            SysProxyType::Http => {
                disable_http_proxy(net_type)?;
            }
            SysProxyType::Socks => {
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

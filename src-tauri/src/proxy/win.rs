use std::io;

use crate::utils::run_command;

use super::{ProxyAction, SysProxyType};

pub struct ProxyImpl;

impl ProxyAction for ProxyImpl {
    fn enable(conf: SysProxyType) -> io::Result<()> {
        enable_proxy(conf)
    }

    fn disable(_conf: SysProxyType) -> io::Result<()> {
        disable_proxy()
    }
}

// https://superuser.com/a/1323579
// reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyEnable /t REG_DWORD /d 1
// reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyServer /t REG_SZ /d name:port
// reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyUser /t REG_SZ /d username
// reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyPass /t REG_SZ /d password
// netsh winhttp import proxy source=ie

const INTERNET_SETTING_KEY: &str =
    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";

/// only support http proxy
fn enable_proxy(proxy_type: SysProxyType) -> io::Result<()> {
    let conf = match proxy_type {
        SysProxyType::Http(proxy_conf) => proxy_conf,
        _ => None,
    };

    if conf.is_none() {
        return Ok(());
    }

    let conf = conf.unwrap();

    let addr = format!("{}:{}", conf.addr, conf.port);

    run_command(
        "reg",
        &[
            "add",
            INTERNET_SETTING_KEY,
            "/f",
            "/v",
            "ProxyServer",
            "/t",
            "REG_SZ",
            "/d",
            addr.as_str(),
        ],
    )?;

    run_command(
        "reg",
        &[
            "add",
            INTERNET_SETTING_KEY,
            "/f",
            "/v",
            "ProxyEnable",
            "/t",
            "REG_DWORD",
            "/d",
            "1",
        ],
    )?;

    Ok(())
}

fn disable_proxy() -> io::Result<()> {
    run_command(
        "reg",
        &[
            "add",
            INTERNET_SETTING_KEY,
            "/f",
            "/v",
            "ProxyEnable",
            "/t",
            "REG_DWORD",
            "/d",
            "0",
        ],
    )?;

    Ok(())
}

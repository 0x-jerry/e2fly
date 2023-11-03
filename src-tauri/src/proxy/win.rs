use std::io;

use crate::utils::run_command;

use super::{ProxyConf, SysProxyType};

// https://superuser.com/a/1323579
// reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyEnable /t REG_DWORD /d 1
// reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyServer /t REG_SZ /d name:port
// reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyUser /t REG_SZ /d username
// reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings" /v ProxyPass /t REG_SZ /d password
// netsh winhttp import proxy source=ie

const INTERNET_SETTING_KEY: &str =
    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";

pub fn enable_proxy(_proxy_type: SysProxyType, conf: ProxyConf) -> io::Result<()> {
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

pub fn disable_proxy(_proxy_type: SysProxyType) -> io::Result<()> {
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

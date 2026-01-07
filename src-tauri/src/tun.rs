use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use tauri::{AppHandle, Manager, Runtime};

use crate::{conf::AppConfigExt, utils::relative_command_path};

const TUN_HELPER_PROGRAM: &str = "tun-helper";
const TUN_GATEWAY: &str = "198.18.0.1";
const TUN_PROGRAM: &str = "hev-socks5-tunnel";
const TUN_PID_FILE: &str = "tun.pid";
const TUN_CONFIG_FILE: &str = "tun.conf.yaml";

pub async fn enable_tun<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    let tun_program = relative_command_path(Path::new(TUN_PROGRAM))?;
    let tun_conf_path = write_tun_config_file(app)?;

    let conf_dir = app.path().app_config_dir()?;
    let pid_path = conf_dir.join(TUN_PID_FILE);

    let args = [
        "start",
        "--program-path",
        tun_program.to_str().unwrap(),
        "--config-path",
        tun_conf_path.to_str().unwrap(),
        "--gateway",
        TUN_GATEWAY,
        "--pid-path",
        pid_path.to_str().unwrap(),
    ];

    println!("Tun mode enable {:?}", args);
    exec_tun_helper(&args)?;

    Ok(())
}

pub fn disable_tun<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    let conf_dir = app.path().app_config_dir()?;
    let pid_path = conf_dir.join(TUN_PID_FILE);

    let args = [
        //
        "stop",
        "--pid-path",
        pid_path.to_str().unwrap(),
    ];

    println!("Tun mode disable {:?}", args);
    exec_tun_helper(&args)?;

    Ok(())
}

pub fn is_tun_mode_enabled<R: Runtime>(app: &AppHandle<R>) -> bool {
    app.path()
        .app_config_dir()
        .unwrap()
        .join(TUN_PID_FILE)
        .exists()
}

fn exec_tun_helper(args: &[&str]) -> Result<()> {
    let tun_helper_program = relative_command_path(Path::new(TUN_HELPER_PROGRAM))?;

    let mut p = privilege::runas::Command::new(tun_helper_program);

    let t = p.gui(true).hide(true).args(args).run()?;

    if !t.success() {
        bail!("Failed to enable tun mode {}", t.code().unwrap())
    }

    Ok(())
}

fn write_tun_config_file<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf> {
    let conf_dir = app.path().app_config_dir()?;
    let conf_path = conf_dir.join(TUN_CONFIG_FILE);
    let port = app.app_config().v2_fly.socks.port;

    let content = format!(
        r#"
tunnel:
    name: utun9
    mtu: 8500
    multi-queue: false
    ipv4: {}
    ipv6: "fc00::1"
socks5:
    port: {}
    address: 127.0.0.1
    udp: tcp
        "#,
        TUN_GATEWAY, port,
    );
    std::fs::write(&conf_path, content)?;
    Ok(conf_path)
}

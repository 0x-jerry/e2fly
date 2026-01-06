use std::path::PathBuf;

use anyhow::{bail, Result};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_shell::ShellExt;

use crate::utils::kill_by_pid;

pub async fn enable_tun<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    if !check_admin_privileges()? {
        // TODO: create dialog to prompt user for admin privileges
        return Ok(());
    }

    let program = app.shell().sidecar("hev-socks5-tunnel")?;

    let tun_conf_path = write_tun_config_file(app)?;

    let t = program.args([tun_conf_path]).output().await?;

    if !t.status.success() {
        let msg = String::from_utf8(t.stderr)?;
        bail!("Start tun failed: {}", msg);
    }

    Ok(())
}

pub fn disable_tun<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    let conf_dir = app.path().app_config_dir()?;
    let pid_path = conf_dir.join("tun.pid");

    if !pid_path.exists() {
        return Ok(());
    }

    let pid = std::fs::read_to_string(&pid_path)?;
    let pid = pid.trim().parse::<u32>()?;

    kill_by_pid(pid);

    Ok(())
}

fn write_tun_config_file<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf> {
    let conf_dir = app.path().app_config_dir()?;
    let conf_path = conf_dir.join("tun_conf.yaml");
    let pid_path = conf_dir.join("tun.pid");

    let content = format!(
        r#"
tunnel:
    name: utun9
    mtu: 8500
    multi-queue: false
    ipv4: 198.18.0.1
    ipv6: "fc00::1"
socks5:
    port: 10021
    address: 127.0.0.1
    udp: tcp
misc:
    pid-file: {}
        "#,
        pid_path.to_str().unwrap()
    );

    std::fs::write(&conf_path, content)?;

    Ok(conf_path)
}

fn check_admin_privileges() -> Result<bool> {
    // todo
    Ok(false)
}

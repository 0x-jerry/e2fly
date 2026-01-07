mod net;
mod utils;

use std::{fs, path::Path, process::Command, thread, time::Duration};

use anyhow::{Result, bail};

use crate::{net::setup_auto_routes, utils::kill_by_pid};

pub async fn enable_tun<T: AsRef<Path>>(
    program_path: T,
    config_path: T,
    gateway: &str,
) -> Result<()> {
    let mut program = Command::new(program_path.as_ref());

    let t = program.args([config_path.as_ref()]).spawn()?.wait()?;

    if !t.success() {
        bail!("Start tun failed: {}", t);
    }

    println!("Start TUN success");

    thread::sleep(Duration::from_secs(1));
    setup_auto_routes(gateway).await?;

    println!("Setup auto routes success");

    Ok(())
}

pub fn disable_tun<T: AsRef<Path>>(pid_file_path: T) -> Result<()> {
    let pid_path = pid_file_path.as_ref();

    if !pid_path.exists() {
        return Ok(());
    }

    let pid = std::fs::read_to_string(&pid_path)?;
    let pid = pid.trim().parse::<u32>()?;

    kill_by_pid(pid);

    fs::remove_file(pid_path)?;

    Ok(())
}

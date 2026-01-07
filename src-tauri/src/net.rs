use anyhow::{anyhow, Result};

pub fn get_default_interface_name() -> Result<String> {
    let i = netdev::get_default_interface().map_err(|msg| anyhow!("{}", msg))?;

    #[cfg(windows)]
    let name = i.friendly_name.unwrap_or_default();

    #[cfg(unix)]
    let name = i.name;

    return Ok(name);
}

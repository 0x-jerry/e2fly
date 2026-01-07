use anyhow::{anyhow, Result};

pub fn get_default_interface_name() -> Result<String> {
    let i = netdev::get_default_interface().map_err(|msg| anyhow!("{}", msg))?;

    Ok(i.name)
}

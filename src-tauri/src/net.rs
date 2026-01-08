use anyhow::Result;

pub fn get_default_interface_name() -> Result<String> {
    #[cfg(windows)]
    let name = {
        let mut name = String::new();

        let ints = netdev::interface::get_interfaces();
        for int in ints {
            if int.gateway.is_some() {
                name = int.friendly_name.clone().unwrap();
                break;
            }
        }

        name
    };

    #[cfg(unix)]
    let name = {
        use anyhow::anyhow;
        let i = netdev::get_default_interface().map_err(|msg| anyhow!("{}", msg))?;
        i.name
    };

    return Ok(name);
}

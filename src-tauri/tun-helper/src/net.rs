use anyhow::Result;
use net_route::{Handle, Route};

pub async fn setup_auto_routes(interface_name: &str) -> Result<()> {
    let handle = Handle::new()?;

    let routes = [
        ("1.0.0.0", 8),
        ("2.0.0.0", 7),
        ("4.0.0.0", 6),
        ("8.0.0.0", 5),
        ("16.0.0.0", 4),
        ("32.0.0.0", 3),
        ("64.0.0.0", 2),
        ("128.0.0.0", 1),
    ];

    let interfaces = netdev::interface::get_interfaces();
    let int = interfaces
        .iter()
        .find(|i| i.friendly_name == Some(interface_name.into()));

    let index = match int {
        Some(i) => i.index,
        None => return Ok(()),
    };

    println!("index {}", index);

    for route in routes {
        let route = Route::new(route.0.parse().unwrap(), route.1).with_ifindex(index);

        match handle.add(&route).await {
            Ok(_) => {
                println!("route add {:?}", route);
            }
            Err(err) => {
                println!("route add err: {}", err)
            }
        }
    }

    Ok(())
}

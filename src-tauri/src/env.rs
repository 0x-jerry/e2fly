use std::env::{self};

use dotenv::dotenv;

pub fn is_dev() -> bool {
    dotenv().ok();

    let dev = match env::var("E2FLY_DEVELOPMENT") {
        Ok(v) => v,
        Err(_e) => String::from(""),
    };

    return dev == "true";
}

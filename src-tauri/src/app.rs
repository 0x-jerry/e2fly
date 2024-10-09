use std::process::exit;

use crate::{ipc::read_conf, proxy::set_proxy, v2fly};

pub fn exit_app() {
    v2fly::stop();

    let mut conf = read_conf();

    if conf.proxy.system {
        conf.proxy.system = false;
        set_proxy(&conf);
    }

    exit(0);
}

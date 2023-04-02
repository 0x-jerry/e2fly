#[path = "lib/mod.rs"]
mod lib;

use crate::conf::get_v2fly_conf_path;
use crate::conf::model::AppConfig;
use lib::nonblock::read_available_to_string;
use std::ffi::OsStr;
use std::io;
use std::process::{Child, Command, Stdio};

use std::sync::{Arc, Mutex};

thread_local! {
    static SINGLETON_POOL: Arc<V2Ray> = Arc::new(Default::default());
}

pub fn get_v2ray_instance() -> Arc<V2Ray> {
    SINGLETON_POOL.with(|singleton_pool| singleton_pool.clone())
}

#[derive(Default)]
pub struct V2Ray {
    program: Mutex<Option<Child>>,
}

impl V2Ray {
    fn run<S, A>(&self, program_path: S, args: A) -> io::Result<()>
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut p = self.program.try_lock().expect("get v2fly instance failed");

        p.as_mut().map(|x| x.kill());

        let program = Command::new(&program_path)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()?;

        *p = Some(program);

        Ok(())
    }

    pub fn stop(&self) {
        let mut p = self.program.try_lock().expect("get v2fly instance failed");

        p.as_mut().map(|x| x.kill());
    }

    pub fn read_all(&self) -> String {
        let mut p = self.program.try_lock().expect("get v2fly instance failed");

        if p.is_none() {
            return "".to_string();
        }

        let mut stdout = p.as_mut().unwrap().stdout.as_mut().unwrap();

        read_available_to_string(&mut stdout)
    }

    pub fn start(&self, app_conf: &AppConfig) -> io::Result<()> {
        let v2ray = get_v2ray_instance();

        v2ray.run(
            app_conf.v2_fly.bin.as_str(),
            ["run", "-c", get_v2fly_conf_path().to_str().unwrap()],
        )
    }
}

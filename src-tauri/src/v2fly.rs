use crate::lib::nonblock::read_available_to_string;
use std::ffi::OsStr;
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
    pub fn run<S, A>(&self, program_path: S, args: A)
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut p = self.program.try_lock().expect("get v2fly instance failed");

        p.as_mut().map(|x| x.kill());

        let program = Command::new(&program_path)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect(format!("Spawn program {:?} failed", program_path.as_ref()).as_str());

        *p = Some(program);
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
}

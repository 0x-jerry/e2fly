use crate::conf::get_v2fly_conf_path;
use crate::conf::model::AppConfig;
use crate::logger::Logger;
use crate::utils::{hide_windows_cmd_window, tail_from_file};
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::path::PathBuf;
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
    log_file: Mutex<Option<PathBuf>>,
}

impl V2Ray {
    fn run<S, A>(&self, program_path: S, args: A) -> io::Result<()>
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.stop();

        let mut program = Command::new(&program_path);

        hide_windows_cmd_window(&mut program);

        self.log_file
            .try_lock()
            .expect("log file")
            .as_ref()
            .map(|p| {
                let stdout= Logger::from_path(p.to_str().unwrap());
                let stderr= Logger::from_path(p.to_str().unwrap());

                program.stdout(stdout);
                program.stderr(stderr);
            });

        program.args(args);

        let child = program.spawn()?;

        let mut p = self.program.try_lock().expect("get v2fly instance failed");
        *p = Some(child);

        Ok(())
    }

    pub fn stop(&self) {
        let mut p = self.program.try_lock().expect("get v2fly instance failed");

        p.as_mut().map(|x| {
            x.kill().unwrap_or_default();
            x.wait().unwrap_or_default();
        });
    }

    pub fn start(&self, app_conf: &AppConfig) -> io::Result<()> {
        self.run(
            app_conf.v2_fly.bin.as_str(),
            ["run", "-c", get_v2fly_conf_path().to_str().unwrap()],
        )
    }

    pub fn set_log_file(&self, log_file: PathBuf) -> io::Result<()> {
        let mut f = self.log_file.try_lock().expect("get log file path");
        *f = Some(log_file);

        Ok(())
    }

    pub fn read_logs(&self) -> Vec<String> {
        let log_file = self.log_file.try_lock().unwrap();

        match log_file.as_ref() {
            Some(file) => {
                let f = File::open(file);

                if f.is_err() {
                    return vec![];
                }

                match f {
                    Ok(x) => tail_from_file(&x, 1000),
                    Err(_) => vec![],
                }
            }
            None => {
                return vec![];
            }
        }
    }
}

use crate::conf::get_v2fly_conf_path;
use crate::conf::model::AppConfig;
use crate::logger::Logger;
use crate::utils::{hide_windows_cmd_window, tail_from_file};
use std::cell::RefCell;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Result;
use std::path::PathBuf;
use std::process::{Child, Command};

thread_local! {
    static V2RAY: RefCell<V2Ray> = RefCell::new(V2Ray::default());
}

#[derive(Default)]
pub struct V2Ray {
    program: Option<Child>,
    log_file: Option<PathBuf>,
}

fn run<S, A>(program_path: S, args: A) -> Result<()>
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    stop();

    let mut program = Command::new(&program_path);

    hide_windows_cmd_window(&mut program);

    V2RAY.with_borrow_mut(|v2ray| -> Result<()> {
        v2ray.log_file.as_ref().map(|p| {
            let stdout = Logger::from(p);
            let stderr = Logger::from(p);

            program.stdout(stdout);
            program.stderr(stderr);
        });

        program.args(args);

        let child = program.spawn()?;

        v2ray.program = Some(child);

        Ok(())
    })
}

pub fn stop() {
    V2RAY.with_borrow_mut(|v2ray| {
        v2ray.program.as_mut().map(|x| {
            x.kill().unwrap_or_default();
            x.wait().unwrap_or_default();
        });
    })
}

pub fn start(app_conf: &AppConfig) -> Result<()> {
    run(
        app_conf.v2_fly.bin.as_str(),
        ["run", "-c", get_v2fly_conf_path().to_str().unwrap()],
    )
}

pub fn set_log_file(log_file: PathBuf) {
    V2RAY.with_borrow_mut(|v2ray| {
        v2ray.log_file = Some(log_file);
    });
}

pub fn read_logs() -> Vec<String> {
    V2RAY.with_borrow(|v2ray| match v2ray.log_file.as_ref() {
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
    })
}

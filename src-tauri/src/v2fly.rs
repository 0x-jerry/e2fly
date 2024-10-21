use tauri::async_runtime::Mutex;
use tauri::{is_dev, AppHandle, Manager, Runtime};

use crate::conf::{self};
use crate::logger::Logger;
use crate::utils::{hide_windows_cmd_window, tail_from_file};
use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Child, Command};

pub struct FlyState(Mutex<FlyStateInner>);

impl FlyState {
    pub fn init<R: Runtime>(app: &AppHandle<R>) {
        let fly = FlyStateInner::new(app);

        let fly_state = FlyState(Mutex::new(fly));

        app.manage(fly_state);
    }

    pub async fn restart(&self) -> Result<(), std::io::Error> {
        self.0.lock().await.restart()
    }

    pub async fn stop(&self) {
        self.0.lock().await.stop();
    }

    pub async fn read_log(&self) -> Vec<String> {
        self.0.lock().await.read_log()
    }
}

struct FlyStateInner {
    program: Option<Child>,
    log_file: PathBuf,
}

impl FlyStateInner {
    pub fn new<R: Runtime>(app: &AppHandle<R>) -> FlyStateInner {
        let log_file = get_log_file_path(app);

        let fly = FlyStateInner {
            program: None,
            log_file,
        };

        fly
    }

    pub fn read_log(&self) -> Vec<String> {
        let f = File::open(&self.log_file);

        if f.is_err() {
            return vec![];
        }

        match f {
            Ok(x) => tail_from_file(&x, 1000),
            Err(_) => vec![],
        }
    }

    pub fn restart(&mut self) -> Result<(), std::io::Error> {
        self.stop();

        let config = conf::read();

        let conf_path = conf::get_v2fly_conf_path();
        let args = ["run", "-c", conf_path.to_str().unwrap()];

        self.start_with_args(config.v2_fly.bin, args)
    }

    fn start_with_args<Str, Args>(&mut self, bin: Str, args: Args) -> Result<(), std::io::Error>
    where
        Str: AsRef<OsStr>,
        Args: IntoIterator<Item: AsRef<OsStr>>,
    {
        //

        let mut program = Command::new(&bin);

        hide_windows_cmd_window(&mut program);

        let stdout = Logger::from(&self.log_file);
        let stderr = Logger::from(&self.log_file);

        program.stdout(stdout);
        program.stderr(stderr);

        program.args(args);

        let child = program.spawn()?;

        self.program = Some(child);

        Ok(())
    }

    pub fn stop(&mut self) {
        self.program
            .as_mut()
            .map(|p| p.kill().expect("kill failed"));
    }
}

fn get_log_file_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    let app_log_dir = app.path().app_log_dir().expect("get app log dir failed");

    let file_name = if is_dev() {
        "v2ray.dev.log"
    } else {
        "v2ray.log"
    };

    let log_file_path = app_log_dir.join(file_name);

    return log_file_path;
}

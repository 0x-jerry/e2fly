use tauri::async_runtime::Mutex;
use tauri::{is_dev, AppHandle, Manager, Runtime, State};

use crate::conf::AppConfigState;
use crate::logger::Logger;
use crate::utils::{hide_windows_cmd_window, tail_from_file};
use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Child, Command};

pub struct FlyState<R: Runtime> {
    inner: Mutex<FlyStateInner>,
    app_handle: AppHandle<R>,
}

impl<R: Runtime> FlyState<R> {
    pub fn init(app: &AppHandle<R>) {
        let fly = FlyStateInner::new(app);

        let fly_state = FlyState {
            inner: Mutex::new(fly),
            app_handle: app.clone(),
        };

        app.manage(fly_state);
    }

    pub fn restart(&self) -> Result<(), std::io::Error> {
        let app_conf_state = self.app_handle.state::<AppConfigState>();
        let app_conf = app_conf_state.lock().unwrap();

        self.inner.blocking_lock().restart(
            app_conf.conf.v2_fly.bin.clone(),
            app_conf.v2ray_config_path(),
        )
    }

    pub fn stop(&self) {
        self.inner.blocking_lock().stop();
    }

    pub fn read_log(&self) -> Vec<String> {
        self.inner.blocking_lock().read_log()
    }
}

pub trait FlyStateExt<R: Runtime> {
    fn fly_state(&self) -> State<'_, FlyState<R>>;
}

impl<R: Runtime> FlyStateExt<R> for AppHandle<R> {
    fn fly_state(&self) -> State<'_, FlyState<R>> {
        let t = self.state::<FlyState<R>>();

        t
    }
}

struct FlyStateInner {
    program: Option<Child>,
    log_file: PathBuf,
}

impl FlyStateInner {
    pub fn new<R: Runtime>(app: &AppHandle<R>) -> FlyStateInner {
        let log_file = get_log_file_path(app);

        FlyStateInner {
            program: None,
            log_file,
        }
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

    pub fn restart(&mut self, bin: String, conf_path: PathBuf) -> Result<(), std::io::Error> {
        self.stop();

        let args = ["run", "-c", conf_path.to_str().unwrap()];

        self.start_with_args(bin, args)
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
        if let Some(p) = self.program.as_mut() {
            p.kill().expect("kill failed")
        }
    }
}

fn get_log_file_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    let app_log_dir = app.path().app_log_dir().expect("get app log dir failed");

    let file_name = if is_dev() {
        "v2ray.dev.log"
    } else {
        "v2ray.log"
    };

    app_log_dir.join(file_name)
}

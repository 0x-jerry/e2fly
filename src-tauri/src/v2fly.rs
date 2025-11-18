use crate::conf::AppConfigState;
use crate::logger::Logger;
use crate::utils::{hide_windows_cmd_window, tail_from_file};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::PathBuf;
use std::process::Command;
use tauri::{is_dev, AppHandle, Manager, Runtime, State};

pub struct FlyState<R: Runtime> {
    log_file: PathBuf,
    app_handle: AppHandle<R>,
}

impl<R: Runtime> FlyState<R> {
    pub fn init(app: &AppHandle<R>) {
        let fly_state = FlyState {
            log_file: get_log_file_path(app),
            app_handle: app.clone(),
        };

        app.manage(fly_state);
    }

    pub fn restart(&self) -> Result<(), std::io::Error> {
        let app_conf_state = self.app_handle.state::<AppConfigState>();
        let app_conf = app_conf_state.lock().unwrap();

        self.stop();

        let v2ray_conf_path = app_conf.v2ray_config_path();
        let args = ["run", "-c", v2ray_conf_path.to_str().unwrap()];
        self.start_with_args(app_conf.conf.v2_fly.bin.clone(), args)
    }

    fn start_with_args<Str, Args>(&self, bin: Str, args: Args) -> Result<(), std::io::Error>
    where
        Str: AsRef<OsStr>,
        Args: IntoIterator<Item: AsRef<OsStr>>,
    {
        let mut program = Command::new(&bin);

        hide_windows_cmd_window(&mut program);

        let stdout = Logger::from(&self.log_file);
        let stderr = Logger::from(&self.log_file);

        program.stdout(stdout);
        program.stderr(stderr);

        program.args(args);

        let child = program.spawn()?;
        let pid = child.id();

        let pid_path = get_pid_file_path(&self.app_handle);
        fs::write(&pid_path, pid.to_string()).expect("write pid to file failed");

        Ok(())
    }

    pub fn stop(&self) {
        let pid_file_path = get_pid_file_path(&self.app_handle);
        if let Ok(content) = fs::read(&pid_file_path) {
            let str = String::from_utf8(content).expect("convert to string");
            let pid = str.parse::<u32>().expect("parse pid failed");

            kill_by_pid(pid);

            fs::remove_file(&pid_file_path).expect("remove pid file failed");
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

fn get_log_file_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    let app_log_dir = app.path().app_log_dir().expect("get app log dir failed");

    let file_name = if is_dev() {
        "v2ray.dev.log"
    } else {
        "v2ray.log"
    };

    app_log_dir.join(file_name)
}

fn get_pid_file_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    let app_log_dir = app.path().app_config_dir().expect("get app log dir failed");

    let pid_file_name = if is_dev() {
        "v2ray_pid.dev.txt"
    } else {
        "v2ray_pid.txt"
    };

    app_log_dir.join(pid_file_name)
}

fn kill_by_pid(pid: u32) {
    let mut sys = sysinfo::System::new();
    sys.refresh_all();

    let pid = sysinfo::Pid::from_u32(pid);

    if let Some(process) = sys.process(pid) {
        process.kill();
    }
}

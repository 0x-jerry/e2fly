use rev_buf_reader::RevBufReader;
use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use tauri::{utils::platform, AppHandle, Manager, Result, Runtime, WebviewWindow};

use crate::const_var::WINDOW_NAME;

pub fn run_command(cmd: &str, args: &[&str]) -> io::Result<String> {
    let mut command = Command::new(cmd);

    hide_windows_cmd_window(&mut command);

    let mut stdout = command
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .expect("!stdout");

    let mut s = String::new();

    stdout.read_to_string(&mut s)?;

    Ok(s)
}

/// https://stackoverflow.com/a/60958956
pub fn hide_windows_cmd_window(cmd: &mut Command) {
    // avoid build warning
    let _ = cmd;

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        cmd.creation_flags(CREATE_NO_WINDOW);
    }
}

pub fn tail_from_file(file: &File, limit: usize) -> Vec<String> {
    let buf = RevBufReader::new(file);
    buf.lines()
        .take(limit)
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn toggle_main_window<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    app.get_webview_window(WINDOW_NAME)
        .map(|win| -> Result<()> {
            if win.is_visible()? && !win.is_minimized()? {
                win.hide()?;
            } else {
                show_window(&win)?;
            }

            Ok(())
        })
        .unwrap_or_else(|| Ok(()))
}

pub fn show_window<R: Runtime>(win: &WebviewWindow<R>) -> Result<()> {
    win.set_always_on_top(true)?;
    win.show()?;

    if win.is_minimized()? {
        win.unminimize()?;
    }

    win.set_focus()?;
    win.set_always_on_top(false)?;

    Ok(())
}

pub fn kill_by_pid(pid: u32) {
    #[cfg(windows)]
    {
        use anyhow::Result;

        if let Err(err) = win_kill_process_by_pid(pid) {
            log::info!("kill process failed, error: {}", err);
        }

        fn win_kill_process_by_pid(pid: u32) -> Result<()> {
            use windows::Win32::Foundation::{CloseHandle, HANDLE};
            use windows::Win32::System::Threading::{
                OpenProcess, TerminateProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE,
            };

            unsafe {
                let desired_access = PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION;

                let process_handle: HANDLE = OpenProcess(desired_access, false, pid)?;

                // Use a guard or defer equivalent for closing the handle
                // The HANDLE type doesn't implement Drop automatically in windows-rs for all scenarios,
                // so explicit closing is a good practice, or use an external crate like 'scopeguard'.

                TerminateProcess(process_handle, 1)?;
                CloseHandle(process_handle)?;
            };

            Ok(())
        }
    }

    #[cfg(not(windows))]
    {
        let mut sys = sysinfo::System::new();
        sys.refresh_all();

        let pid = sysinfo::Pid::from_u32(pid);

        if let Some(process) = sys.process(pid) {
            process.kill_and_wait().unwrap_or_default();
        }
    }
}

pub fn relative_command_path(
    command: &Path,
) -> std::result::Result<PathBuf, tauri_plugin_shell::Error> {
    match platform::current_exe()?.parent() {
        #[cfg(windows)]
        Some(exe_dir) => {
            let mut command_path = exe_dir.join(command);
            let already_exe = command_path.extension().is_some_and(|ext| ext == "exe");
            if !already_exe {
                // do not use with_extension to retain dots in the command filename
                command_path.as_mut_os_string().push(".exe");
            }

            let command_path_str = command_path.to_str().unwrap();

            command_path = if command_path_str.starts_with("\\\\?\\") {
                command_path_str[4..].into()
            } else {
                command_path
            };

            Ok(command_path)
        }
        #[cfg(not(windows))]
        Some(exe_dir) => {
            let mut command_path = exe_dir.join(command);
            if command_path.extension().is_some_and(|ext| ext == "exe") {
                command_path.set_extension("");
            }
            Ok(command_path)
        }
        None => Err(tauri_plugin_shell::Error::CurrentExeHasNoParent),
    }
}

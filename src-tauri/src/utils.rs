use rev_buf_reader::RevBufReader;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::{
    fs::File,
    io::{self, BufRead, Read},
    process::{Command, Stdio},
};

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

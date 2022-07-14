use std::{
    io::{self, Read},
    process::{Command, Stdio},
};

pub fn run_command(cmd: &str, args: &[&str]) -> io::Result<String> {
    let mut stdout = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .expect("!stdout");

    let mut s = String::new();

    stdout.read_to_string(&mut s)?;

    Ok(s)
}

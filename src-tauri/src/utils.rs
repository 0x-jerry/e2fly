use rev_buf_reader::RevBufReader;
use std::{
    fs::File,
    io::{self, BufRead, Read},
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

pub fn tail_from_file(file: &File, limit: usize) -> Vec<String> {
    let buf = RevBufReader::new(file);
    buf.lines()
        .take(limit)
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

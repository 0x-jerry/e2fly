use std::{fs::File, io::BufRead};

use rev_buf_reader::RevBufReader;

pub fn tail_from_file(file: &File, limit: usize) -> Vec<String> {
    let buf = RevBufReader::new(file);
    buf.lines()
        .take(limit)
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

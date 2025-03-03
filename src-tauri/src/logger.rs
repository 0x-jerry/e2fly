use file_rotate::{compression::Compression, suffix::AppendCount, ContentLimit, FileRotate};
use std::{
    io::{Read, Write},
    path::Path,
    process::Stdio,
};

pub struct Logger(FileRotate<AppendCount>);

impl<T: AsRef<Path>> From<T> for Logger {
    fn from(path: T) -> Self {
        let log = FileRotate::new(
            path,
            AppendCount::new(2),
            ContentLimit::Lines(10000),
            Compression::None,
            None,
        );

        Self(log)
    }
}

impl From<Logger> for Stdio {
    fn from(val: Logger) -> Self {
        let (mut reader, writer) = os_pipe::pipe().unwrap();

        std::thread::spawn(move || {
            let mut log = val.0;
            let mut buffer = [0; 4096];
            loop {
                let bytes_read = reader.read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    break;
                }
                log.write_all(&buffer[..bytes_read]).unwrap();
            }
        });

        Stdio::from(writer)
    }
}

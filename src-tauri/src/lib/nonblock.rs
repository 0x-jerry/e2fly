//! Read available data from file descriptors without blocking
//!
//! Useful for nonblocking reads from sockets, named pipes, and child stdout/stderr
//!
//! # Example
//!
//! ```no_run
//! use std::io::Read;
//! use std::process::{Command, Stdio};
//! use std::time::Duration;
//! use nonblock::NonBlockingReader;
//!
//! let mut child = Command::new("some-executable")
//!                         .stdout(Stdio::piped())
//!                         .spawn().unwrap();
//! let stdout = child.stdout.take().unwrap();
//! let mut noblock_stdout = NonBlockingReader::from_fd(stdout).unwrap();
//! while !noblock_stdout.is_eof() {
//!     let mut buf = String::new();
//!     noblock_stdout.read_available_to_string(&mut buf).unwrap();
//!     std::thread::sleep(Duration::from_secs(5));
//! }
//! ```
extern crate libc;
use libc::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK};
use std::io::{self, ErrorKind, Read};
use std::os::unix::io::{AsRawFd, RawFd};
use std::process::ChildStdout;

/// Simple non-blocking wrapper for reader types that implement AsRawFd
pub struct NonBlockingReader<'a> {
    eof: bool,
    reader: &'a mut ChildStdout,
}

impl<'a> NonBlockingReader<'a> {
    /// Initialize a NonBlockingReader from the reader's file descriptor.
    ///
    /// The reader will be managed internally,
    ///   and O_NONBLOCK will be set the file descriptor.
    pub fn from_fd(reader: &'a mut ChildStdout) -> io::Result<NonBlockingReader<'a>> {
        let fd = reader.as_raw_fd();
        set_blocking(fd, false)?;
        Ok(NonBlockingReader { reader, eof: false })
    }

    /// Reads any available data from the reader without blocking, placing them into `buf`.
    ///
    /// If successful, this function will return the total number of bytes read.
    ///  0 bytes read may indicate the EOF has been reached or that reading
    ///  would block because there is not any data immediately available.
    ///  Call `is_eof()` after this method to determine if EOF was reached.
    ///
    /// ## Errors
    ///
    /// If this function encounters an error of the kind `ErrorKind::Interrupted`
    ///   then the error is ignored and the operation will continue.
    ///   If it encounters `ErrorKind::WouldBlock`, then this function immediately returns
    ///   the total number of bytes read so far.
    ///
    /// If any other read error is encountered then this function immediately returns.
    ///   Any bytes which have already been read will be appended to buf.
    ///
    /// ## Examples
    /// ```no_run
    /// # use std::io::Read;
    /// # use std::net::TcpStream;
    /// # use std::time::Duration;
    /// # use nonblock::NonBlockingReader;
    /// #
    /// let client = TcpStream::connect("127.0.0.1:34567").unwrap();
    /// let mut noblock_stdout = NonBlockingReader::from_fd(client).unwrap();
    /// let mut buf = Vec::new();
    /// noblock_stdout.read_available(&mut buf).unwrap();
    /// ```
    pub fn read_available(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut buf_len = 0;
        loop {
            let mut bytes = [0u8; 1024];
            match self.reader.read(&mut bytes[..]) {
                // EOF
                Ok(0) => {
                    self.eof = true;
                    break;
                }
                // Not EOF, but no more data currently available
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => {
                    self.eof = false;
                    break;
                }
                // Ignore interruptions, continue reading
                Err(ref err) if err.kind() == ErrorKind::Interrupted => {}
                // bytes available
                Ok(len) => {
                    buf_len += len;
                    buf.append(&mut bytes[0..(len)].to_owned())
                }
                // IO Error encountered
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(buf_len)
    }

    /// Reads any available data from the reader without blocking, placing them into `buf`.
    ///
    /// If successful, this function returns the number of bytes which were read and appended to buf.
    ///
    /// ## Errors
    ///
    /// This function inherits all the possible errors of `read_available()`.
    ///   In the case of errors that occur after successfully reading some data,
    ///   the successfully read data will still be parsed and appended to `buf`.
    ///
    /// Additionally, if the read data cannot be parsed as UTF-8,
    ///   then `buf` will remain unmodified, and this method will return `ErrorKind::InvalidData`
    ///   with the `FromUtf8Error` containing any data that was read.
    ///
    /// ## Examples
    /// ```no_run
    /// # use std::io::Read;
    /// # use std::process::{Command, Stdio};
    /// # use std::time::Duration;
    /// # use nonblock::NonBlockingReader;
    /// #
    /// let mut child = Command::new("foo").stdout(Stdio::piped()).spawn().unwrap();
    /// let stdout = child.stdout.take().unwrap();
    /// let mut noblock_stdout = NonBlockingReader::from_fd(stdout).unwrap();
    /// let mut buf = String::new();
    /// noblock_stdout.read_available_to_string(&mut buf).unwrap();
    /// ```
    ///
    /// In theory, since this function only reads immediately available data,
    ///   There may not be any guarantee that the data immediately available ends
    ///   on a UTF-8 alignment, so it might be worth a bufferred wrapper
    ///   that manages the captures a final non-UTF-8 character and prepends it to the next call,
    ///   but in practice, this has worked as expected.
    pub fn read_available_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        let mut byte_buf: Vec<u8> = Vec::with_capacity(1024);
        let res = self.read_available(&mut byte_buf);
        match String::from_utf8(byte_buf) {
            Ok(utf8_buf) => {
                // append any read data before returning the `read_available` result
                buf.push_str(&utf8_buf);
                res
            }
            Err(err) => {
                // check for read error before returning the UTF8 Error
                let _ = res?;
                Err(io::Error::new(ErrorKind::InvalidData, err))
            }
        }
    }
}

fn set_blocking(fd: RawFd, blocking: bool) -> io::Result<()> {
    let flags = unsafe { fcntl(fd, F_GETFL, 0) };
    if flags < 0 {
        return Err(io::Error::last_os_error());
    }

    let flags = if blocking {
        flags & !O_NONBLOCK
    } else {
        flags | O_NONBLOCK
    };
    let res = unsafe { fcntl(fd, F_SETFL, flags) };
    if res != 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}

/// based on: https://github.com/anowell/nonblock-rs
pub fn read_available_to_string(stream: &mut ChildStdout) -> String {
    let mut nb_stdout = NonBlockingReader::from_fd(stream).unwrap();

    let mut s = String::new();
    nb_stdout.read_available_to_string(&mut s).unwrap();

    s
}

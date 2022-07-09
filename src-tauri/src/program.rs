use std::ffi::OsStr;
use std::io::{self, BufRead, BufReader};
use std::process::{Child, Command, Stdio};

pub struct Program {
    pub program: Child,
}

impl Program {
    pub fn run<S, A>(program_path: S, args: A) -> Self
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let program = Command::new(&program_path)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect(format!("Spawn program {:?} failed", program_path.as_ref()).as_str());

        Self { program }
    }

    pub fn read_all(&mut self) -> Box<Vec<String>> {
        let stdout = self.program.stdout.as_mut().unwrap();
        let mut reader = BufReader::new(stdout);

        let mut output: Box<Vec<String>> = Box::new(vec![]);

        loop {
            let mut buf = String::new();

            match reader.read_line(&mut buf) {
                Ok(size) => {
                    if size == 0 {
                        break;
                    }

                    output.push(buf);
                }
                Err(_) => break,
            }
        }

        output
    }
}

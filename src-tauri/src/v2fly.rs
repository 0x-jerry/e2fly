use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

use std::sync::{Arc, Mutex};

thread_local! {
    static SINGLETON_POOL: Arc<V2Ray> = Arc::new(Default::default());
}

pub fn get_v2ray_instance() -> Arc<V2Ray> {
    SINGLETON_POOL.with(|singleton_pool| singleton_pool.clone())
}

#[derive(Default)]
pub struct V2Ray {
    program: Mutex<Option<Child>>,
}

impl V2Ray {
    pub fn run<S, A>(&self, program_path: S, args: A)
    where
        A: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut p = self.program.try_lock().unwrap();

        p.as_mut().map(|x| x.kill());

        let program = Command::new(&program_path)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect(format!("Spawn program {:?} failed", program_path.as_ref()).as_str());

        *p = Some(program);
    }

    pub fn stop(&self) {
        let mut p = self.program.try_lock().unwrap();

        p.as_mut().map(|x| x.kill());
    }

    pub fn read_all(&self) -> Box<Vec<String>> {
        let mut p = self.program.try_lock().unwrap();
        let mut output: Box<Vec<String>> = Box::new(vec![]);

        if p.is_none() {
            return output;
        }

        let stdout = p.as_mut().unwrap().stdout.as_mut().unwrap();
        let mut reader = BufReader::new(stdout);

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

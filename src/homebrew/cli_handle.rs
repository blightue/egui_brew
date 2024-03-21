use std::{
    io::{BufRead, BufReader, BufWriter},
    process::{Child, ChildStdin, ChildStdout},
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

// TODO: use channel to get and set stdout/stdin/child on another thread, for avoiding read stdout blocking
pub struct CliHandle {
    pub stdout: Receiver<String>,
    pub stdin: BufWriter<ChildStdin>,
    pub child: Arc<Mutex<Child>>,
}

unsafe impl Send for CliHandle {}

impl CliHandle {
    pub fn new(
        mut stdout: BufReader<ChildStdout>,
        stdin: BufWriter<ChildStdin>,
        child: Child,
    ) -> Self {
        let (out_tx, out_rx) = std::sync::mpsc::channel();
        let child = Arc::new(Mutex::new(child));
        let out_check_child = Arc::clone(&child);
        thread::spawn(move || {
            let mut buffer = String::new();
            while out_check_child
                .lock()
                .unwrap()
                .try_wait()
                .unwrap()
                .is_none()
            {
                match stdout.read_line(&mut buffer) {
                    Ok(_) => {
                        out_tx.send(buffer.clone()).unwrap();
                        buffer.clear();
                    }
                    Err(_) => break,
                }
            }
        });

        Self {
            stdout: out_rx,
            stdin: stdin,
            child: child,
        }
    }
}

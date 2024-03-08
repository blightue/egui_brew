use std::{
    io::{BufReader, BufWriter},
    process::{Child, ChildStdin, ChildStdout},
    sync::{Arc, Mutex},
};

// TODO: use channel to get and set stdout/stdin/child on another thread, for avoiding read stdout blocking
pub struct CliHandle {
    pub stdout: BufReader<ChildStdout>,
    pub stdin: BufWriter<ChildStdin>,
    pub child: Arc<Mutex<Child>>,
}

unsafe impl Send for CliHandle {}

impl CliHandle {
    pub fn new(stdout: BufReader<ChildStdout>, stdin: BufWriter<ChildStdin>, child: Child) -> Self {
        Self {
            stdout: stdout,
            stdin: stdin,
            child: Arc::new(Mutex::new(child)),
        }
    }
}

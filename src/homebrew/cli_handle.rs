use std::{
    io::{BufReader, BufWriter},
    process::{Child, ChildStdin, ChildStdout},
    sync::Arc,
};

pub struct CliHandle {
    pub stdout: BufReader<ChildStdout>,
    pub stdin: BufWriter<ChildStdin>,
    pub child: Arc<Child>,
}

unsafe impl Send for CliHandle {}

impl CliHandle {
    pub fn new(
        stdout: BufReader<ChildStdout>,
        stdin: BufWriter<ChildStdin>,
        child: Arc<Child>,
    ) -> Self {
        Self {
            stdout: stdout,
            stdin: stdin,
            child: child,
        }
    }
}

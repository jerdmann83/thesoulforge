#[derive(Debug)]
pub struct RuntimeError {
    pub msg: String,
    pub line: usize,
}

impl RuntimeError {
    pub fn new(msg: &str, line: usize) -> Self {
        Self {
            msg: msg.to_string(),
            line: line,
        }
    }
}

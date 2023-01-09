#[derive(Debug)]
pub struct LogFile {
    path: String,
}

impl LogFile {
    pub fn new(f: &str) -> LogFile {
        LogFile {
            path: f.to_string(),
        }
    }
}

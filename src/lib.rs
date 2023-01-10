use std::{fs::File, io, path::Path};

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

pub fn read_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

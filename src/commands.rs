pub mod debug;
pub mod report;

use std::{error::Error, fmt, path::PathBuf};

#[derive(Debug, Clone)]
struct NoSuchFileError {}
impl Error for NoSuchFileError {}
impl fmt::Display for NoSuchFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no such file")
    }
}

fn parse_file_path(input: &str) -> Result<PathBuf, NoSuchFileError> {
    let path = PathBuf::from(input);
    match path.is_file() {
        true => Ok(path),
        false => Err(NoSuchFileError {}),
    }
}

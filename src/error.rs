use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("io error `{0}`")]
    IoError(String),
    #[error("no file found for path `{0}`")]
    FileOpenedError(String),
    #[error("invalid pfs0 header magic: `{0}`")]
    InvalidPFS0Magic(u32),
    #[error("invalid input: `{0}`")]
    InvalidInput(String),
}

impl From<std::io::Error> for FileSystemError {
    fn from(value: std::io::Error) -> Self {
        return FileSystemError::IoError(value.to_string());
    }
}

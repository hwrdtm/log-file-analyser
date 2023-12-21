#[derive(Debug)]
pub struct Error {
    inner: String,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            inner: e.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

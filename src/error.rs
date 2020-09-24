use std::ffi::NulError;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum SeError {
    NulErr(NulError),
    Utf8Err(Utf8Error),
    IoErr(IoError),
    GenericFailure(String),
    InvalidPath(PathBuf),
}

impl From<NulError> for SeError {
    fn from(e: NulError) -> Self {
        SeError::NulErr(e)
    }
}

impl From<Utf8Error> for SeError {
    fn from(e: Utf8Error) -> Self {
        SeError::Utf8Err(e)
    }
}

impl From<IoError> for SeError {
    fn from(e: IoError) -> Self {
        SeError::IoErr(e)
    }
}

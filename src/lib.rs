mod context;
mod utils;

pub use context::*;
pub use utils::*;

pub type Result<T> = std::result::Result<T, Error>;

use std::ffi::NulError;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    NulErr(NulError),
    Utf8Err(Utf8Error),
    IoErr(IoError),
    GenericFailure(String),
    InvalidPath(PathBuf),
}

impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        Error::NulErr(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::Utf8Err(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::IoErr(e)
    }
}
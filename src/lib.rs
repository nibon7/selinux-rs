mod context;
mod utils;

pub use context::*;
pub use utils::*;

pub type Result<T> = std::result::Result<T, Error>;

use std::ffi::NulError;

#[derive(Debug)]
pub enum Error {
    Generic,
    InvalidPath(Vec<u8>),
    NoSpace,
    QuotaEnforcement,
    NotSupported,
    SysErrno(isize),
}

impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        Error::InvalidPath(e.into_vec())
    }
}

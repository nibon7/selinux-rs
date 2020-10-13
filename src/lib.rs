mod context;
mod utils;

pub use context::*;
pub use utils::*;
pub mod avc;

/// Common result type used by this library.
pub type Result<T> = std::result::Result<T, Error>;

use std::ffi::NulError;

/// Common error type used by this library.
#[derive(Debug)]
pub enum Error {
    /// Generic failure without an errno returned.
    Generic,
    /// Invalid file path input.
    InvalidPath(Vec<u8>),
    /// Common failure with an errno returned.
    IoError(std::io::Error),
}

impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        Error::InvalidPath(e.into_vec())
    }
}

pub(crate) fn handle_errno(ret: libc::c_int) -> Result<()> {
    match ret {
        0 => Ok(()),
        _ => {
            debug_assert_eq!(ret, -1); // assert the value only on debug build
            Err(Error::IoError(errno::errno().into()))
        }
    }
}

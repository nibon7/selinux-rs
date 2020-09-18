extern crate errno;
extern crate libc;

mod context;
mod error;
mod ffi;
mod scon;
mod utils;

pub use context::*;
pub use error::*;
pub use scon::*;
pub use utils::*;

pub type Result<T> = std::result::Result<T, SeError>;

extern crate errno;
extern crate libc;
extern crate selinux_macros;

mod context;
mod error;
mod scon;
mod utils;

pub use context::*;
pub use error::*;
pub use scon::*;
pub use utils::*;

pub type Result<T> = std::result::Result<T, SeError>;

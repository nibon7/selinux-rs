extern crate errno;
extern crate libc;

mod context;
mod error;
mod utils;

pub use context::*;
pub use error::*;
pub use utils::*;

pub type Result<T> = std::result::Result<T, SeError>;

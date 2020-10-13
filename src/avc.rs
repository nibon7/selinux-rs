use crate::{handle_errno, Result};
use selinux_sys::*;

/// Initializes the userspace AVC and must be called before any other AVC
/// operation can be performed
pub fn open(enforce: bool) -> Result<()> {
    let (opt, nopt) = if enforce {
        (
            &mut selinux_opt {
                type_: AVC_OPT_SETENFORCE as i32,
                value: std::ptr::null_mut(),
            } as *mut selinux_opt,
            1,
        )
    } else {
        (std::ptr::null_mut() as *mut selinux_opt, 0)
    };
    handle_errno(unsafe { avc_open(opt, nopt) })
}

/// Destroys the userspace AVC, freeing all internal memory structures.  After
/// this call has been made, open() must be called again before any AVC
/// operations can be performed.
pub fn destroy() {
    unsafe { avc_destroy() }
}

/// Flushes the userspace AVC, causing it to forget any cached access decisions.
/// The userspace AVC normally calls this function automatically when needed.
pub fn reset() -> Result<()> {
    handle_errno(unsafe { avc_reset() })
}

/// Attempts to free unused memory within the userspace AVC, but does not flush
/// any cached access decisions.  Under normal operation, calling this function
/// should not be necessary
pub fn cleanup() {
    unsafe { avc_cleanup() }
}

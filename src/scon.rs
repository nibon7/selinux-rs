use crate::{ffi, Result, SeError};
use errno;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::ptr;

pub struct SCon(*mut c_char);

macro_rules! wrap_getcon {
    ($f:ident) => {
        pub fn $f() -> Option<SCon> {
            let mut p = ptr::null_mut();
            match unsafe { ffi::$f(&mut p) } {
                0 if !p.is_null() => Some(SCon { 0: p }),
                _ => None,
            }
        }
    };
}

macro_rules! wrap_getcon_path {
    ($f:ident) => {
        pub fn $f<P: AsRef<Path>>(path: P) -> Option<SCon> {
            let mut p = ptr::null_mut();
            let cs = path.as_ref().to_str().and_then(|s| CString::new(s).ok())?;

            match unsafe { ffi::$f(cs.as_ptr(), &mut p) } {
                0 if !p.is_null() => Some(SCon { 0: p }),
                _ => None,
            }
        }
    };
}

macro_rules! wrap_getcon_fd {
    ($f:ident) => {
        pub fn $f(f: &File) -> Option<SCon> {
            let mut p = ptr::null_mut();
            let fd = f.as_raw_fd();

            match unsafe { ffi::$f(fd, &mut p) } {
                0 if !p.is_null() => Some(SCon { 0: p }),
                _ => None,
            }
        }
    };
}

macro_rules! wrap_setcon {
    ($f:ident) => {
        pub fn $f(scon: &str) -> Result<()> {
            let cs = CString::new(scon)?;
            let p = cs.as_ptr();

            match unsafe { ffi::$f(p as *const i8) } {
                0 => Ok(()),
                _ => Err(io::Error::from_raw_os_error(errno::errno().0))
                    .map_err(|e| SeError::IoErr(e)),
            }
        }
    };
}

impl SCon {
    wrap_getcon!(getcon);
    wrap_getcon!(getcon_raw);
    wrap_getcon!(getprevcon);
    wrap_getcon!(getprevcon_raw);
    wrap_getcon!(getexeccon);
    wrap_getcon!(getexeccon_raw);
    wrap_getcon!(getfscreatecon);
    wrap_getcon!(getfscreatecon_raw);
    wrap_getcon!(getkeycreatecon);
    wrap_getcon!(getkeycreatecon_raw);
    wrap_getcon!(getsockcreatecon);
    wrap_getcon!(getsockcreatecon_raw);

    pub fn getpidcon(pid: i32) -> Option<Self> {
        let mut p = ptr::null_mut();
        match unsafe { ffi::getpidcon(pid, &mut p) } {
            0 if !p.is_null() => Some(Self { 0: p }),
            _ => None,
        }
    }

    wrap_getcon_path!(getfilecon);
    wrap_getcon_path!(getfilecon_raw);
    wrap_getcon_path!(lgetfilecon);
    wrap_getcon_path!(lgetfilecon_raw);
    wrap_getcon_fd!(getpeercon);
    wrap_getcon_fd!(getpeercon_raw);
    wrap_getcon_fd!(fgetfilecon);
    wrap_getcon_fd!(fgetfilecon_raw);

    wrap_setcon!(setcon);
    wrap_setcon!(setcon_raw);
    wrap_setcon!(setexeccon);
    wrap_setcon!(setexeccon_raw);
    wrap_setcon!(setfscreatecon);
    wrap_setcon!(setfscreatecon_raw);
    wrap_setcon!(setkeycreatecon);
    wrap_setcon!(setkeycreatecon_raw);
    wrap_setcon!(setsockcreatecon);
    wrap_setcon!(setsockcreatecon_raw);

    pub fn setexecfilecon<P: AsRef<Path>>(filename: P, fallback_type: &str) -> Result<()> {
        let cs_type = CString::new(fallback_type)?;
        let p_type = cs_type.as_ptr();

        let cs_filename = CString::new(filename.as_ref().to_str().unwrap())?;
        let p_filename = cs_filename.as_ptr();

        match unsafe { ffi::setexecfilecon(p_filename, p_type as *const i8) } {
            0 => Ok(()),
            _ => Err(io::Error::from_raw_os_error(errno::errno().0)).map_err(|e| SeError::IoErr(e)),
        }
    }
}

impl Drop for SCon {
    fn drop(&mut self) {
        unsafe { ffi::freecon(self.0) }
    }
}

impl fmt::Display for SCon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cs = unsafe { CStr::from_ptr(self.0) };

        match cs.to_str() {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl fmt::Debug for SCon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cs = unsafe { CStr::from_ptr(self.0) };

        match cs.to_str() {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

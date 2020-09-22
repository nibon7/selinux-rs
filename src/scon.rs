use crate::{Result, SeError};
use errno;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::ptr;

pub struct SCon(*mut c_char);

macro_rules! wrap_getcon {
    ($f:ident) => {
        pub fn $f() -> Option<SCon> {
            let mut p = ptr::null_mut();
            match unsafe { selinux_sys::$f(&mut p) } {
                0 if !p.is_null() => Some(SCon { 0: p }),
                _ => None,
            }
        }
    };
}

macro_rules! wrap_getcon_path {
    ($f:ident) => {
        pub fn $f<P: AsRef<Path>>(path: P) -> Option<SCon> {
            if !path.as_ref().exists() {
                return None;
            }

            let cs = path.as_ref().to_str().and_then(|s| CString::new(s).ok())?;
            let mut p = ptr::null_mut();

            match unsafe { selinux_sys::$f(cs.as_ptr(), &mut p) } {
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

            match unsafe { selinux_sys::$f(f.as_raw_fd(), &mut p) } {
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

            match unsafe { selinux_sys::$f(cs.as_ptr()) } {
                0 => Ok(()),
                _ => Err(io::Error::from_raw_os_error(errno::errno().0))
                    .map_err(|e| SeError::IoErr(e)),
            }
        }
    };
}

macro_rules! wrap_setcon_path {
    ($f:ident) => {
        pub fn $f<P: AsRef<Path>>(p: P, scon: &str) -> Result<()> {
            if !p.as_ref().exists() {
                return Err(SeError::IoErr(io::Error::new(
                    io::ErrorKind::NotFound,
                    "file not found",
                )));
            }

            let cs = p
                .as_ref()
                .to_str()
                .ok_or(SeError::IoErr(io::Error::new(
                    io::ErrorKind::Other,
                    "fail to convert path to str",
                )))
                .and_then(|s| CString::new(s).map_err(|e| SeError::NulErr(e)))?;

            let s = CString::new(scon)?;

            match unsafe { selinux_sys::$f(cs.as_ptr(), s.as_ptr()) } {
                0 => Ok(()),
                _ => Err(io::Error::from_raw_os_error(errno::errno().0))
                    .map_err(|e| SeError::IoErr(e)),
            }
        }
    };
}

macro_rules! wrap_setcon_fd {
    ($f:ident) => {
        pub fn $f(f: &File, scon: &str) -> Result<()> {
            let cs = CString::new(scon)?;

            match unsafe { selinux_sys::$f(f.as_raw_fd(), cs.as_ptr()) } {
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
        match unsafe { selinux_sys::getpidcon(pid, &mut p) } {
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

    wrap_setcon_path!(setfilecon);
    wrap_setcon_path!(setfilecon_raw);
    wrap_setcon_path!(lsetfilecon);
    wrap_setcon_path!(lsetfilecon_raw);
    wrap_setcon_path!(setexecfilecon);

    wrap_setcon_fd!(fsetfilecon);
    wrap_setcon_fd!(fsetfilecon_raw);
}

impl Drop for SCon {
    fn drop(&mut self) {
        unsafe { selinux_sys::freecon(self.0) }
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

pub trait Getcon {
    fn getcon(&self) -> Option<SCon>;
}

pub trait Setcon {
    fn setcon(&self, s: &str) -> Result<()>;
}

impl Getcon for Path {
    fn getcon(&self) -> Option<SCon> {
        crate::SCon::getfilecon(&self)
    }
}

impl Getcon for PathBuf {
    fn getcon(&self) -> Option<SCon> {
        self.as_path().getcon()
    }
}

impl Getcon for File {
    fn getcon(&self) -> Option<SCon> {
        crate::SCon::fgetfilecon(&self)
    }
}

impl Setcon for Path {
    fn setcon(&self, scon: &str) -> Result<()> {
        crate::SCon::setfilecon(&self, scon)
    }
}

impl Setcon for PathBuf {
    fn setcon(&self, scon: &str) -> Result<()> {
        self.as_path().setcon(scon)
    }
}

impl Setcon for File {
    fn setcon(&self, scon: &str) -> Result<()> {
        crate::SCon::fsetfilecon(&self, scon)
    }
}

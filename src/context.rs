use crate::{Error, Result};
use errno::errno;
use selinux_sys::*;
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

pub struct Context {
    user: String,
    role: String,
    _type: String,
    range: String,
}

#[derive(Default)]
pub struct ContextBuilder {
    user: String,
    role: String,
    _type: String,
    range: String,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> Option<Context> {
        if !self.user.is_empty() && !self.role.is_empty() && !self._type.is_empty() {
            Some(Context {
                user: self.user.clone(),
                role: self.role.clone(),
                _type: self._type.clone(),
                range: self.range.clone(),
            })
        } else {
            None
        }
    }

    pub fn user(&mut self, user: &str) -> &mut Self {
        self.user = user.to_owned();
        self
    }

    pub fn role(&mut self, role: &str) -> &mut Self {
        self.role = role.to_owned();
        self
    }

    pub fn _type(&mut self, _type: &str) -> &mut Self {
        self._type = _type.to_owned();
        self
    }

    pub fn range(&mut self, range: &str) -> &mut Self {
        self.range = range.to_owned();
        self
    }
}

macro_rules! context_access {
    ($field:ident, $setter:ident) => {
        pub fn $field(&self) -> &str {
            self.$field.as_str()
        }

        pub fn $setter(&mut self, $field: &str) {
            self.$field = $field.to_owned();
        }
    };
}

macro_rules! wrap_ffi_get {
    ($func:ident, $c:ident, $args:tt) => {{
        let mut $c = std::ptr::null_mut();
        unsafe {
            match $func $args {
                x if x != -1 && !$c.is_null() => {
                    let mut res = None;
                    if let Some(s) = CStr::from_ptr($c).to_str().ok() {
                        res = Context::new(s);
                    }
                    selinux_sys::freecon($c);
                    res
                },
                _ => None,
            }
        }
    }}
}

macro_rules! get_context {
    ($(#[$attr:meta])* $func:ident, $wrap:ident) => {
        $(#[$attr])*
        pub fn $func() -> Option<Self> {
            wrap_ffi_get!($wrap, context, (&mut context))
        }
    };
}

fn handle_error(ret: libc::c_int) -> Result<()> {
    match ret {
        0 => Ok(()),
        _ => Err(Error::Generic),
    }
}

fn handle_file_error(ret: libc::c_int) -> Result<()> {
    match ret {
        0 => Ok(()),
        _ => {
            debug_assert_eq!(ret, -1); // assert the value only on debug build
            Err(match errno().into() {
                libc::ENOSPC => Error::NoSpace,
                libc::EDQUOT => Error::QuotaEnforcement,
                libc::ENOTSUP => Error::NotSupported,
                e => Error::SysErrno(e as isize),
            })
        }
    }
}

macro_rules! set_context {
    ($(#[$attr:meta])* $func:ident, $wrap:ident) => {
        $(#[$attr])*
        pub fn $func(&self) -> Result<()> {
            let cs = self.to_cstring();
            unsafe { handle_error(selinux_sys::$wrap(cs.as_ptr())) }
        }
    };
}

macro_rules! set_path_context {
    ($(#[$attr:meta])* $func:ident, $wrap:ident, $error:ident) => {
        $(#[$attr])*
        pub fn $func(&self, path: impl AsRef<Path>) -> Result<()> {
            let path = CString::new(path.as_ref().as_os_str().as_bytes())?;
            let cs = self.to_cstring();
            unsafe { $error(selinux_sys::$wrap(path.as_ptr(), cs.as_ptr())) }
        }
    };
}

macro_rules! set_fd_context {
    ($func:ident, $wrap:ident, $error:ident) => {
        pub fn $func(&self, fd: &impl AsRawFd) -> Result<()> {
            let cs = self.to_cstring();
            unsafe { $error(selinux_sys::$wrap(fd.as_raw_fd(), cs.as_ptr())) }
        }
    };
}

macro_rules! get_path_context {
    ($(#[$attr:meta])* $func:ident, $wrap:ident) => {
        pub fn $func(path: impl AsRef<Path>) -> Option<Self> {
            let cs = path.as_ref().to_str().and_then(|s| CString::new(s).ok())?;
            wrap_ffi_get!($wrap, context, (cs.as_ptr(), &mut context))
        }
    };
}

macro_rules! get_fd_context {
    ($(#[$attr:meta])* $func:ident, $wrap:ident) => {
        pub fn $func(fd: &impl AsRawFd) -> Option<Self> {
            wrap_ffi_get!($wrap, context, (fd.as_raw_fd(), &mut context))
        }
    };
}

impl Context {
    pub fn new(context: &str) -> Option<Self> {
        let mut iter = context.split(":");
        let user = iter.next()?.to_owned();
        let role = iter.next()?.to_owned();
        let _type = iter.next()?.to_owned();
        let range = iter.collect::<Vec<&str>>().join(":");
        Some(Context {
            user,
            role,
            _type,
            range,
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}:{}:{}", self.user, self.role, self._type, self.range)
    }

    // NOTE: CString's as_ptr() do not keep it from being dropped, you must keep
    // a binding or reference yourself
    fn to_cstring(&self) -> CString {
        CString::new(self.to_string()).unwrap()
    }

    context_access!(user, set_user);
    context_access!(role, set_role);
    context_access!(_type, set_type);
    context_access!(range, set_range);

    get_context!(
        /// Retrieves the context of the current process.
        current,
        getcon
    );

    get_context!(
        /// Retrieves  the  context of the current process without context
        /// translation.
        current_raw,
        getcon_raw
    );

    get_context!(
        /// Same as `current` but gets the context before the last exec.
        previous,
        getprevcon
    );

    get_context!(
        /// Same as `previous` but do not perform context translation.
        previous_raw,
        getprevcon_raw
    );

    get_context!(
        /// Retrieves the context used for executing a new process.
        execute,
        getexeccon
    );

    get_context!(
        /// Identical to `execute` without context translation.
        execute_raw,
        getexeccon_raw
    );

    get_context!(fs_create, getfscreatecon);
    get_context!(fs_create_raw, getfscreatecon_raw);
    get_context!(key_create, getkeycreatecon);
    get_context!(key_create_raw, getkeycreatecon_raw);
    get_context!(socket_create, getsockcreatecon);
    get_context!(socket_create_raw, getsockcreatecon_raw);

    get_path_context!(file, getfilecon);
    get_path_context!(file_raw, getfilecon_raw);
    get_path_context!(file_nolink, lgetfilecon);
    get_path_context!(file_nolink_raw, lgetfilecon_raw);

    get_fd_context!(peer, getpeercon);
    get_fd_context!(peer_raw, getpeercon_raw);
    get_fd_context!(fd, fgetfilecon);
    get_fd_context!(fd_raw, fgetfilecon_raw);

    set_context!(set_current, setcon);
    set_context!(set_current_raw, setcon_raw);
    set_context!(set_exec, setexeccon);
    set_context!(set_exec_raw, setexeccon_raw);
    set_context!(set_fs_create, setfscreatecon);
    set_context!(set_fs_create_raw, setfscreatecon_raw);
    set_context!(set_key_create, setkeycreatecon);
    set_context!(set_key_create_raw, setkeycreatecon_raw);
    set_context!(set_socket_create, setsockcreatecon);
    set_context!(set_socket_create_raw, setsockcreatecon_raw);

    set_path_context!(set_file, setfilecon, handle_file_error);
    set_path_context!(set_file_raw, setfilecon_raw, handle_file_error);
    set_path_context!(set_file_nolink, lsetfilecon, handle_file_error);
    set_path_context!(set_file_nolink_raw, lsetfilecon_raw, handle_file_error);

    // TODO: Is this function pass a errno ?
    set_path_context!(set_execfile, setexecfilecon, handle_error);

    set_fd_context!(set_fd, fsetfilecon, handle_file_error);
    set_fd_context!(set_fd_raw, fsetfilecon_raw, handle_file_error);
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self.user == other.user
            && self.role == other.role
            && self._type == other._type
            && self.range == other.range
    }
}

impl AsRef<Context> for Context {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait Getcon {
    fn getcon(&self) -> Option<Context>;
}

pub trait Setcon {
    fn setcon(&self, con: impl AsRef<Context>) -> Result<()>;
}

impl Getcon for Path {
    fn getcon(&self) -> Option<Context> {
        Context::file(self)
    }
}

impl Setcon for Path {
    fn setcon(&self, con: impl AsRef<Context>) -> Result<()> {
        con.as_ref().set_file(self)
    }
}

impl Getcon for PathBuf {
    fn getcon(&self) -> Option<Context> {
        Context::file(self.as_path())
    }
}

impl Setcon for PathBuf {
    fn setcon(&self, con: impl AsRef<Context>) -> Result<()> {
        con.as_ref().set_file(self.as_path())
    }
}

impl Getcon for File {
    fn getcon(&self) -> Option<Context> {
        Context::fd(self)
    }
}

impl Setcon for File {
    fn setcon(&self, con: impl AsRef<Context>) -> Result<()> {
        con.as_ref().set_fd(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTEXT: &str = "user_u:role_r:type_t:s0-s0:c0.c1023";
    const CONTEXT_WITHOUT_RANGE: &str = "user_u:role_r:type_t";

    #[test]
    fn context_basic() {
        let mut context = Context::new(CONTEXT).unwrap();
        assert_eq!(context.user(), "user_u");
        assert_eq!(context._type(), "type_t");
        assert_eq!(context.role(), "role_r");
        assert_eq!(context.range(), "s0-s0:c0.c1023");

        context.set_type("unconfined_t");
        assert_eq!(context._type(), "unconfined_t");

        context.set_range("s0");
        assert_eq!(context.range(), "s0");

        assert_eq!(context.to_string(), "user_u:role_r:unconfined_t:s0");

        let context = Context::new(CONTEXT_WITHOUT_RANGE).unwrap();
        assert_eq!(context.range(), "");
        assert_eq!(context._type(), "type_t");

        let context1 = Context::new(CONTEXT).unwrap();
        let context2 = Context::new(CONTEXT).unwrap();
        assert_eq!(context1, context2);
    }

    #[test]
    fn test_formatter() {
        let context = Context::new(CONTEXT).unwrap();
        assert_eq!(format!("{}", context), CONTEXT);
        assert_eq!(format!("{:?}", context), CONTEXT);
    }

    #[test]
    fn context_retrieve() {
        if crate::enabled().unwrap() {
            Context::current().unwrap();
        }
    }
}

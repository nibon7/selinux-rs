use crate::{Result, SeError};
use selinux_sys::*;
use std::ffi::{CStr, CString};
use std::fmt::{Debug, Display, Formatter};
use std::os::unix::io::AsRawFd;
use std::path::Path;
pub struct Context {
    user: String,
    role: String,
    _type: String,
    range: String,
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
                0 if !$c.is_null() => {
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
        _ => Err(SeError::GenericFailure("Generic".into())),
    }
}

macro_rules! set_context {
    ($(#[$attr:meta])* $func:ident, $wrap:ident) => {
        $(#[$attr])*
        pub fn $func(&self) -> Result<()> {
            unsafe { handle_error(selinux_sys::$wrap(self.to_cstr())) }
        }
    };
}

macro_rules! set_path_context {
    ($func:ident, $wrap:ident) => {
        set_path_context!($func, $wrap, handle_error)
    };
    ($func:ident, $wrap:ident, $error:ident) => {
        pub fn $func(&self, path: impl AsRef<Path>) -> Result<()> {
            let path = CString::new(
                path.as_ref()
                    .to_str()
                    .ok_or(SeError::InvalidPath(path.as_ref().to_owned()))?,
            )?
            .as_ptr();
            unsafe { $error(selinux_sys::$wrap(self.to_cstr(), path)) }
        }
    };
}

macro_rules! set_fd_context {
    ($func:ident, $wrap:ident, $error:ident) => {
        pub fn $func(&self, fd: impl AsRawFd) -> Result<()> {
            unsafe { $error(selinux_sys::$wrap(fd.as_raw_fd(), self.to_cstr())) }
        }
    };
}

macro_rules! get_path_context {
    ($(#[$attr:meta])* $func:ident, $wrap:ident) => {
        pub fn $func(path: impl AsRef<Path>) -> Option<Self> {
            let path = path
                .as_ref()
                .to_str()
                .and_then(|s| CString::new(s).ok())?
                .as_ptr();
            wrap_ffi_get!($wrap, context, (path, &mut context))
        }
    };
}

macro_rules! get_fd_context {
    ($(#[$attr:meta])* $func:ident, $wrap:ident) => {
        pub fn $func(fd: impl AsRawFd) -> Option<Self> {
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

    fn to_cstr(&self) -> *const i8 {
        CString::new(self.to_string()).unwrap().as_ptr()
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

    set_path_context!(set_file, setfilecon, handle_error);
    set_path_context!(set_file_raw, setfilecon_raw, handle_error);
    set_path_context!(set_file_nolink, lsetfilecon, handle_error);
    set_path_context!(set_file_nolink_raw, lsetfilecon_raw, handle_error);
    set_path_context!(set_execfile, setexecfilecon, handle_error);

    set_fd_context!(set_fd, fsetfilecon, handle_error);
    set_fd_context!(set_fd_raw, fsetfilecon_raw, handle_error);
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

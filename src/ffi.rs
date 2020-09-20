use libc::{c_char, c_int, c_void, pid_t};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct context_s_t {
    pub ptr: *mut c_void,
}

#[allow(non_camel_case_types)]
pub type context_t = *mut context_s_t;

#[link(name = "selinux")]
extern "C" {

    pub fn is_selinux_enabled() -> c_int;
    pub fn is_selinux_mls_enabled() -> c_int;

    pub fn getcon(scon: *mut *mut c_char) -> c_int;
    pub fn getcon_raw(scon: *mut *mut c_char) -> c_int;
    pub fn getprevcon(scon: *mut *mut c_char) -> c_int;
    pub fn getprevcon_raw(scon: *mut *mut c_char) -> c_int;
    pub fn getexeccon(scon: *mut *mut c_char) -> c_int;
    pub fn getexeccon_raw(scon: *mut *mut c_char) -> c_int;
    pub fn getfscreatecon(scon: *mut *mut c_char) -> c_int;
    pub fn getfscreatecon_raw(scon: *mut *mut c_char) -> c_int;
    pub fn getkeycreatecon(scon: *mut *mut c_char) -> c_int;
    pub fn getkeycreatecon_raw(scon: *mut *mut c_char) -> c_int;
    pub fn getsockcreatecon(scon: *mut *mut c_char) -> c_int;
    pub fn getsockcreatecon_raw(scon: *mut *mut c_char) -> c_int;
    pub fn getpidcon(pid: pid_t, scon: *mut *mut c_char) -> c_int;
    pub fn getpeercon(fd: c_int, scon: *mut *mut c_char) -> c_int;
    pub fn getpeercon_raw(fd: c_int, scon: *mut *mut c_char) -> c_int;

    pub fn getfilecon(path: *const c_char, scon: *mut *mut c_char) -> c_int;
    pub fn getfilecon_raw(path: *const c_char, scon: *mut *mut c_char) -> c_int;
    pub fn lgetfilecon(path: *const c_char, scon: *mut *mut c_char) -> c_int;
    pub fn lgetfilecon_raw(path: *const c_char, scon: *mut *mut c_char) -> c_int;
    pub fn fgetfilecon(fd: c_int, scon: *mut *mut c_char) -> c_int;
    pub fn fgetfilecon_raw(fd: c_int, scon: *mut *mut c_char) -> c_int;

    pub fn setcon(scon: *const c_char) -> c_int;
    pub fn setcon_raw(scon: *const c_char) -> c_int;
    pub fn setexeccon(scon: *const c_char) -> c_int;
    pub fn setexeccon_raw(scon: *const c_char) -> c_int;
    pub fn setfscreatecon(scon: *const c_char) -> c_int;
    pub fn setfscreatecon_raw(scon: *const c_char) -> c_int;
    pub fn setkeycreatecon(scon: *const c_char) -> c_int;
    pub fn setkeycreatecon_raw(scon: *const c_char) -> c_int;
    pub fn setsockcreatecon(scon: *const c_char) -> c_int;
    pub fn setsockcreatecon_raw(scon: *const c_char) -> c_int;

    pub fn setfilecon(path: *const c_char, scon: *const c_char) -> c_int;
    pub fn setfilecon_raw(path: *const c_char, scon: *const c_char) -> c_int;
    pub fn lsetfilecon(path: *const c_char, scon: *const c_char) -> c_int;
    pub fn lsetfilecon_raw(path: *const c_char, scon: *const c_char) -> c_int;
    pub fn fsetfilecon(fd: libc::c_int, scon: *const c_char) -> c_int;
    pub fn fsetfilecon_raw(fd: libc::c_int, scon: *const c_char) -> c_int;

    pub fn setexecfilecon(filename: *const c_char, fallback_type: *const c_char) -> c_int;

    pub fn freecon(scon: *mut c_char);

    pub fn context_new(context_str: *const c_char) -> context_t;
    pub fn context_free(context: context_t);
    pub fn context_str(context: context_t) -> *const c_char;
    pub fn context_user_get(context: context_t) -> *const c_char;
    pub fn context_role_get(context: context_t) -> *const c_char;
    pub fn context_type_get(context: context_t) -> *const c_char;
    pub fn context_range_get(context: context_t) -> *const c_char;
    pub fn context_user_set(context: context_t, user_str: *const c_char) -> c_int;
    pub fn context_role_set(context: context_t, role_str: *const c_char) -> c_int;
    pub fn context_type_set(context: context_t, type_str: *const c_char) -> c_int;
    pub fn context_range_set(context: context_t, range_str: *const c_char) -> c_int;
}

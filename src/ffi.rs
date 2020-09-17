use libc::{c_char, c_int, c_void};

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

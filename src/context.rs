use crate::ffi;
use std::ffi::{CStr, CString};

pub struct Context(ffi::context_t);

impl Context {
    pub fn new(s: &str) -> Option<Self> {
        let cs = match CString::new(s) {
            Ok(_cs) => _cs,
            Err(_) => return None,
        };

        match unsafe { ffi::context_new(cs.as_ptr() as *const i8) } {
            p if !p.is_null() => Some(Self { 0: p }),
            _ => None,
        }
    }

    pub fn to_str(&self) -> Option<&str> {
        unsafe {
            match ffi::context_str(self.0) {
                p if !p.is_null() => CStr::from_ptr(p).to_str().ok(),
                _ => None,
            }
        }
    }

    pub fn get_user(&self) -> Option<&str> {
        unsafe {
            match ffi::context_user_get(self.0) {
                p if !p.is_null() => CStr::from_ptr(p).to_str().ok(),
                _ => None,
            }
        }
    }

    pub fn get_role(&self) -> Option<&str> {
        unsafe {
            match ffi::context_role_get(self.0) {
                p if !p.is_null() => CStr::from_ptr(p).to_str().ok(),
                _ => None,
            }
        }
    }

    pub fn get_type(&self) -> Option<&str> {
        unsafe {
            match ffi::context_type_get(self.0) {
                p if !p.is_null() => CStr::from_ptr(p).to_str().ok(),
                _ => None,
            }
        }
    }

    pub fn get_range(&self) -> Option<&str> {
        unsafe {
            match ffi::context_range_get(self.0) {
                p if !p.is_null() => CStr::from_ptr(p).to_str().ok(),
                _ => None,
            }
        }
    }

    pub fn set_user(&mut self, user_str: &str) -> Option<&mut Self> {
        let c_str = CString::new(user_str).unwrap();

        unsafe {
            match ffi::context_user_set(self.0, c_str.as_ptr() as *const i8) {
                0 => Some(self),
                _ => None,
            }
        }
    }

    pub fn set_role(&mut self, role_str: &str) -> Option<&mut Self> {
        let c_str = CString::new(role_str).unwrap();

        unsafe {
            match ffi::context_role_set(self.0, c_str.as_ptr() as *const i8) {
                0 => Some(self),
                _ => None,
            }
        }
    }

    pub fn set_type(&mut self, type_str: &str) -> Option<&mut Self> {
        let c_str = CString::new(type_str).unwrap();

        unsafe {
            match ffi::context_type_set(self.0, c_str.as_ptr() as *const i8) {
                0 => Some(self),
                _ => None,
            }
        }
    }

    pub fn set_range(&mut self, range_str: &str) -> Option<&mut Self> {
        let c_str = CString::new(range_str).unwrap();

        unsafe {
            match ffi::context_range_set(self.0, c_str.as_ptr() as *const i8) {
                0 => Some(self),
                _ => None,
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::context_free(self.0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_upper_case_globals)]
    const s: &str = "unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023";

    #[test]
    fn test_new() {
        assert!(Context::new(s).is_some());
    }

    #[test]
    fn test_to_str() {
        let ctx = Context::new(s).unwrap();
        assert_eq!(ctx.to_str(), Some(s));
    }

    #[test]
    fn test_get_user() {
        let ctx = Context::new(s).unwrap();
        assert_eq!(ctx.get_user(), Some("unconfined_u"));
    }

    #[test]
    fn test_get_role() {
        let ctx = Context::new(s).unwrap();
        assert_eq!(ctx.get_role(), Some("unconfined_r"));
    }

    #[test]
    fn test_get_type() {
        let ctx = Context::new(s).unwrap();
        assert_eq!(ctx.get_type(), Some("unconfined_t"));
    }

    #[test]
    fn test_get_range() {
        let ctx = Context::new(s).unwrap();
        assert_eq!(ctx.get_range(), Some("s0-s0:c0.c1023"));
    }

    #[test]
    fn test_set_user() {
        let mut ctx = Context::new(s).unwrap();
        ctx.set_user("user_u").unwrap();
        assert_eq!(ctx.get_user(), Some("user_u"));
    }

    #[test]
    fn test_set_role() {
        let mut ctx = Context::new(s).unwrap();
        ctx.set_role("user_r").unwrap();
        assert_eq!(ctx.get_role(), Some("user_r"));
    }

    #[test]
    fn test_set_type() {
        let mut ctx = Context::new(s).unwrap();
        ctx.set_type("user_t").unwrap();
        assert_eq!(ctx.get_type(), Some("user_t"));
    }

    #[test]
    fn test_set_range() {
        let mut ctx = Context::new(s).unwrap();
        ctx.set_range("s0").unwrap();
        assert_eq!(ctx.get_range(), Some("s0"));
    }
}
